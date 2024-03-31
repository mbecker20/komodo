import { Page, Section } from "@components/layouts";
import { ConfirmButton, ResourceLink } from "@components/util";
import { useInvalidate, useRead, useWrite } from "@lib/hooks";
import { Types } from "@monitor/client";
import { UsableResource } from "@types";
import { DataTable } from "@ui/data-table";
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
  const nav = useNavigate();
  const users = useRead("GetUsers", {}).data;
  return (
    <Page title="Users">
      <DataTable
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
                ? "text-green-500"
                : "text-red-500";
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
  const { mutate } = useWrite("UpdateUserPerimissions", {
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
  servers?.forEach((server) => {
    const perm = permissions?.find(
      (p) => p.target.type === "Server" && p.target.id === server.id
    );
    if (perm) {
      perms.push({ ...perm, name: server.name });
    } else {
      perms.push({
        user_id,
        name: server.name,
        level: Types.PermissionLevel.None,
        target: { type: "Server", id: server.id },
      });
    }
  });
  deployments?.forEach((deployment) => {
    const perm = permissions?.find(
      (p) => p.target.type === "Deployment" && p.target.id === deployment.id
    );
    if (perm) {
      perms.push({ ...perm, name: deployment.name });
    } else {
      perms.push({
        user_id,
        name: deployment.name,
        level: Types.PermissionLevel.None,
        target: { type: "Deployment", id: deployment.id },
      });
    }
  });
  builds?.forEach((build) => {
    const perm = permissions?.find(
      (p) => p.target.type === "Build" && p.target.id === build.id
    );
    if (perm) {
      perms.push({ ...perm, name: build.name });
    } else {
      perms.push({
        user_id,
        name: build.name,
        level: Types.PermissionLevel.None,
        target: { type: "Build", id: build.id },
      });
    }
  });
  repos?.forEach((repo) => {
    const perm = permissions?.find(
      (p) => p.target.type === "Repo" && p.target.id === repo.id
    );
    if (perm) {
      perms.push({ ...perm, name: repo.name });
    } else {
      perms.push({
        user_id,
        name: repo.name,
        level: Types.PermissionLevel.None,
        target: { type: "Repo", id: repo.id },
      });
    }
  });
  procedures?.forEach((procedure) => {
    const perm = permissions?.find(
      (p) => p.target.type === "Procedure" && p.target.id === procedure.id
    );
    if (perm) {
      perms.push({ ...perm, name: procedure.name });
    } else {
      perms.push({
        user_id,
        name: procedure.name,
        level: Types.PermissionLevel.None,
        target: { type: "Procedure", id: procedure.id },
      });
    }
  });
  builders?.forEach((builder) => {
    const perm = permissions?.find(
      (p) => p.target.type === "Builder" && p.target.id === builder.id
    );
    if (perm) {
      perms.push({ ...perm, name: builder.name });
    } else {
      perms.push({
        user_id,
        name: builder.name,
        level: Types.PermissionLevel.None,
        target: { type: "Builder", id: builder.id },
      });
    }
  });
  alerters?.forEach((alerter) => {
    const perm = permissions?.find(
      (p) => p.target.type === "Alerter" && p.target.id === alerter.id
    );
    if (perm) {
      perms.push({ ...perm, name: alerter.name });
    } else {
      perms.push({
        user_id,
        name: alerter.name,
        level: Types.PermissionLevel.None,
        target: { type: "Alerter", id: alerter.id },
      });
    }
  });
  return perms;
};

const PermissionsTable = () => {
  const [showNone, setShowNone] = useState(false);
  const [search, setSearch] = useState("");
  const searchSplit = search.toLowerCase().split(" ");
  const inv = useInvalidate();
  const user_id = useParams().id as string;
  const permissions = useUserPermissions(user_id);
  const { mutate } = useWrite("UpdateUserPermissionsOnTarget", {
    onSuccess: () => inv(["ListUserPermissions"]),
  });
  return (
    <Section
      title="Permissions"
      actions={
        <div className="flex gap-6 items-center">
          <div
            className="flex gap-3 items-center"
            onClick={() => setShowNone(!showNone)}
          >
            <Label htmlFor="show-none">Show All Resources</Label>
            <Switch id="show-none" checked={showNone} />
          </div>
          <Input
            placeholder="search"
            value={search}
            onChange={(e) => setSearch(e.target.value)}
            className="w-48"
          />
        </div>
      }
    >
      <DataTable
        data={
          permissions
            ?.filter((permission) =>
              showNone ? true : permission.level !== Types.PermissionLevel.None
            )
            .filter((permission) =>
              searchSplit.every(
                (search) =>
                  permission.name.toLowerCase().includes(search) ||
                  permission.target.type.toLowerCase().includes(search)
              )
            ) ?? []
        }
        columns={[
          {
            header: "Resource",
            accessorKey: "target.type",
          },
          {
            header: "Target",
            cell: ({
              row: {
                original: { target },
              },
            }) => {
              return (
                <ResourceLink
                  type={target.type as UsableResource}
                  id={target.id}
                />
              );
            },
          },
          {
            header: "Level",
            cell: ({ row: { original: permission } }) => (
              <Select
                value={permission.level}
                onValueChange={(value) =>
                  mutate({
                    ...permission,
                    user_id,
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
