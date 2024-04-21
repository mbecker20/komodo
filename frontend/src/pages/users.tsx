import { Page, Section } from "@components/layouts";
import { ResourceComponents } from "@components/resources";
import { ResourceLink } from "@components/resources/common";
import { ConfirmButton } from "@components/util";
import { text_color_class_by_intention } from "@lib/color";
import { useInvalidate, useRead, useSetTitle, useWrite } from "@lib/hooks";
import { resource_name } from "@lib/utils";
import { Types } from "@monitor/client";
import { UsableResource } from "@types";
import { DataTable, SortableHeader } from "@ui/data-table";
import { Input } from "@ui/input";
import { Label } from "@ui/label";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@ui/select";
import { Switch } from "@ui/switch";
import { useToast } from "@ui/use-toast";
import { UserCheck, UserMinus } from "lucide-react";
import { useState } from "react";
import { useNavigate, useParams } from "react-router-dom";

export const UsersPage = () => {
  useSetTitle("Users");
  const nav = useNavigate();
  const users = useRead("GetUsers", {}).data;
  return (
    <Page title="Users">
      <DataTable
        tableKey="users"
        data={users ?? []}
        columns={[
          { header: "Username", accessorKey: "username" },
          { header: "Type", accessorKey: "config.type" },
          {
            header: "Level",
            accessorFn: (user) => (user.admin ? "Admin" : "User"),
          },
          {
            header: "Enabled",
            cell: ({ row }) => {
              const enabledClass = row.original.enabled
                ? text_color_class_by_intention("Good")
                : text_color_class_by_intention("Critical");
              return (
                <div className={enabledClass}>
                  {row.original.enabled ? "Enabled" : "Disabled"}
                </div>
              );
            },
          },
        ]}
        onRowClick={(user) => nav(`/users/${user._id!.$oid}`)}
      />
    </Page>
  );
};

export const UserPage = () => {
  const { toast } = useToast();
  const inv = useInvalidate();
  const user_id = useParams().id as string;
  const user = useRead("GetUsers", {}).data?.find(
    (user) => user._id?.$oid === user_id
  );
  const { mutate } = useWrite("UpdateUserBasePermissions", {
    onSuccess: () => inv(["GetUsers"]),
    onError: (e) => {
      console.log(e);
      toast({ title: "Failed to update user permissions" });
    },
  });
  const enabledClass = user?.enabled ? "text-green-500" : "text-red-500";
  const avatar = (user?.config.data as any)?.avatar as string | undefined;
  return (
    user && (
      <Page
        title={
          <div className="flex gap-4 items-center">
            {user?.username}{" "}
            {avatar && <img src={avatar} alt="" className="w-7 h-7" />}
          </div>
        }
        subtitle={
          <div className="text-sm text-muted-foreground flex gap-2">
            <div className={enabledClass}>
              {user?.enabled ? "Enabled" : "Disabled"}
            </div>
            |<div className="">Level: {user?.admin ? "Admin" : "User"}</div>|
            <div className="">Type: {user?.config.type}</div>
          </div>
        }
        actions={
          !user.admin && (
            <div className="flex gap-4 items-center">
              {(["Server", "Build"] as Array<"Server" | "Build">).map(
                (item) => {
                  const key = `create_${item.toLowerCase()}_permissions` as
                    | "create_server_permissions"
                    | "create_build_permissions";
                  const req_key = `create_${item.toLowerCase()}s`;
                  return (
                    <div key={key} className="flex gap-2 items-center">
                      <Label htmlFor={key}>Create {item}</Label>
                      <Switch
                        id={key}
                        className="flex gap-4"
                        checked={user[key]}
                        onClick={() =>
                          mutate({ user_id, [req_key]: !user[key] })
                        }
                      />
                    </div>
                  );
                }
              )}
              <ConfirmButton
                title={user.enabled ? "Disable User" : "Enable User"}
                icon={
                  user.enabled ? (
                    <UserMinus className="w-4 h-4" />
                  ) : (
                    <UserCheck className="w-4 h-4" />
                  )
                }
                variant={user.enabled ? "destructive" : "outline"}
                onClick={() => mutate({ user_id, enabled: !user.enabled })}
              />
            </div>
          )
        }
      >
        {!user.admin && <PermissionsTable />}
      </Page>
    )
  );
};

const useUserPermissions = (user_id: string) => {
  const permissions = useRead("ListUserPermissions", { user_id }).data;
  const servers = useRead("ListServers", {}).data;
  const deployments = useRead("ListDeployments", {}).data;
  const builds = useRead("ListBuilds", {}).data;
  const repos = useRead("ListRepos", {}).data;
  const procedures = useRead("ListProcedures", {}).data;
  const builders = useRead("ListBuilders", {}).data;
  const alerters = useRead("ListAlerters", {}).data;
  const perms: (Types.Permission & { name: string })[] = [];
  addPerms(user_id, permissions, "Server", servers, perms);
  addPerms(user_id, permissions, "Deployment", deployments, perms);
  addPerms(user_id, permissions, "Build", builds, perms);
  addPerms(user_id, permissions, "Repo", repos, perms);
  addPerms(user_id, permissions, "Procedure", procedures, perms);
  addPerms(user_id, permissions, "Builder", builders, perms);
  addPerms(user_id, permissions, "Alerter", alerters, perms);
  return perms;
};

function addPerms<I>(
  user_id: string,
  permissions: Types.Permission[] | undefined,
  resource_type: UsableResource,
  resources: Types.ResourceListItem<I>[] | undefined,
  perms: (Types.Permission & { name: string })[]
) {
  resources?.forEach((resource) => {
    const perm = permissions?.find(
      (p) =>
        p.resource_target.type === resource_type &&
        p.resource_target.id === resource.id
    );
    if (perm) {
      perms.push({ ...perm, name: resource.name });
    } else {
      perms.push({
        user_target: { type: "User", id: user_id },
        name: resource.name,
        level: Types.PermissionLevel.None,
        resource_target: { type: resource_type, id: resource.id },
      });
    }
  });
}

const PermissionsTable = () => {
  const { toast } = useToast();
  const [showNone, setShowNone] = useState(false);
  const [search, setSearch] = useState("");
  const searchSplit = search.toLowerCase().split(" ");
  const inv = useInvalidate();
  const user_id = useParams().id as string;
  const permissions = useUserPermissions(user_id);
  const { mutate } = useWrite("UpdatePermissionOnTarget", {
    onSuccess: () => {
      toast({ title: "Updated user permission" });
      inv(["ListUserPermissions"]);
    },
  });
  return (
    <Section
      title="Permissions"
      actions={
        <div className="flex gap-6 items-center">
          <Input
            placeholder="search"
            value={search}
            onChange={(e) => setSearch(e.target.value)}
            className="w-[300px]"
          />
          <div
            className="flex gap-3 items-center"
            onClick={() => setShowNone(!showNone)}
          >
            <Label htmlFor="show-none">Show All Resources</Label>
            <Switch id="show-none" checked={showNone} />
          </div>
        </div>
      }
    >
      <DataTable
        tableKey="permissions"
        data={
          permissions?.filter(
            (permission) =>
              (showNone
                ? true
                : permission.level !== Types.PermissionLevel.None) &&
              searchSplit.every(
                (search) =>
                  permission.name.toLowerCase().includes(search) ||
                  permission.resource_target.type.toLowerCase().includes(search)
              )
          ) ?? []
        }
        columns={[
          {
            accessorKey: "resource_target.type",
            header: ({ column }) => (
              <SortableHeader column={column} title="Resource" />
            ),
            cell: ({ row }) => {
              const Components =
                ResourceComponents[
                  row.original.resource_target.type as UsableResource
                ];
              return (
                <div className="flex gap-2 items-center">
                  <Components.Icon />
                  {row.original.resource_target.type}
                </div>
              );
            },
          },
          {
            accessorKey: "resource_target",
            sortingFn: (a, b) => {
              const ra = resource_name(
                a.original.resource_target.type as UsableResource,
                a.original.resource_target.id
              );
              const rb = resource_name(
                b.original.resource_target.type as UsableResource,
                b.original.resource_target.id
              );

              if (!ra && !rb) return 0;
              if (!ra) return -1;
              if (!rb) return 1;

              if (ra > rb) return 1;
              else if (ra < rb) return -1;
              else return 0;
            },
            header: ({ column }) => (
              <SortableHeader column={column} title="Target" />
            ),
            cell: ({
              row: {
                original: { resource_target },
              },
            }) => {
              return (
                <ResourceLink
                  type={resource_target.type as UsableResource}
                  id={resource_target.id}
                />
              );
            },
          },
          {
            accessorKey: "level",
            sortingFn: (a, b) => {
              const al = levelToNumber(a.original.level);
              const bl = levelToNumber(b.original.level);
              const dif = al - bl;
              return dif === 0 ? 0 : dif / Math.abs(dif);
            },
            header: ({ column }) => (
              <SortableHeader column={column} title="Level" />
            ),
            cell: ({ row: { original: permission } }) => (
              <Select
                value={permission.level}
                onValueChange={(value) =>
                  mutate({
                    ...permission,
                    user_target: { type: "User", id: user_id },
                    permission: value as Types.PermissionLevel,
                  })
                }
              >
                <SelectTrigger className="w-32 capitalize">
                  <SelectValue />
                </SelectTrigger>
                <SelectContent className="w-32">
                  {Object.keys(Types.PermissionLevel).map((permission) => (
                    <SelectItem
                      value={permission}
                      key={permission}
                      className="capitalize"
                    >
                      {permission}
                    </SelectItem>
                  ))}
                </SelectContent>
              </Select>
            ),
          },
        ]}
      />
    </Section>
  );
};

const levelToNumber = (level: Types.PermissionLevel | undefined) => {
  switch (level) {
    case undefined:
      return 0;
    case Types.PermissionLevel.None:
      return 0;
    case Types.PermissionLevel.Read:
      return 1;
    case Types.PermissionLevel.Execute:
      return 2;
    case Types.PermissionLevel.Write:
      return 3;
  }
};
