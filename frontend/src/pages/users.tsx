import { Page, Section } from "@components/layouts";
import { NewServiceUser, NewUserGroup } from "@components/users/new";
import { PermissionsTable } from "@components/users/permissions-table";
import { ConfirmButton } from "@components/util";
import { text_color_class_by_intention } from "@lib/color";
import { useInvalidate, useRead, useSetTitle, useWrite } from "@lib/hooks";
import { Button } from "@ui/button";
import { DataTable } from "@ui/data-table";
import { Input } from "@ui/input";
import { Label } from "@ui/label";
import { Switch } from "@ui/switch";
import { useToast } from "@ui/use-toast";
import { Save, UserCheck, UserMinus } from "lucide-react";
import { useEffect, useState } from "react";
import { useNavigate, useParams } from "react-router-dom";

export const UsersPage = () => {
  useSetTitle("Users");
  const nav = useNavigate();
  const groups = useRead("ListUserGroups", {}).data;
  const users = useRead("ListUsers", {}).data;
  const [search, setSearch] = useState("");
  const searchSplit = search.split(" ");
  return (
    <Page
      actions={
        <Input
          placeholder="Search"
          value={search}
          onChange={(e) => setSearch(e.target.value)}
          className="w-[250px]"
        />
      }
    >
      {/* User Groups */}
      <Section title="User Groups" actions={<NewUserGroup />}>
        <DataTable
          tableKey="user-groups"
          data={
            groups?.filter((group) =>
              searchSplit.every((term) => group.name.includes(term))
            ) ?? []
          }
          columns={[
            { header: "Name", accessorKey: "name" },
            {
              header: "Members",
              accessorFn: (group) => group.users.length,
            },
          ]}
          onRowClick={(group) => nav(`/user-groups/${group._id!.$oid}`)}
        />
      </Section>

      {/* Users */}
      <Section title="Users" actions={<NewServiceUser />}>
        <DataTable
          tableKey="users"
          data={
            users?.filter((user) =>
              searchSplit.every((term) => user.username.includes(term))
            ) ?? []
          }
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
      </Section>
    </Page>
  );
};

export const UserGroupPage = () => {
  const { toast } = useToast();
  const inv = useInvalidate();
  const group_id = useParams().id as string;
  const group = useRead("ListUserGroups", {}).data?.find(
    (group) => group._id?.$oid === group_id
  );
  const [name, setName] = useState<string>();
  useEffect(() => {
    if (group) setName(group.name);
  }, [group?.name]);
  const renameMutate = useWrite("RenameUserGroup", {
    onSuccess: () => {
      inv(["ListUserGroups"]);
      toast({ title: "Renamed User Group" });
    },
  }).mutate;
  const rename = () => {
    if (!name) {
      toast({ title: "New name cannot be empty" });
      if (group) setName(group.name);
      return;
    }
    renameMutate({ id: group_id, name });
  };
  if (!group) return null;
  return (
    <Page
      title={
        <div className="flex gap-4 items-center">
          <Input
            value={name}
            onChange={(e) => setName(e.target.value)}
            onKeyDown={(e) => {
              if (e.key === "Enter") rename();
            }}
            className="text-3xl h-fit p-2"
          />
          {name !== group.name && (
            <Button size="icon" onClick={rename}>
              <Save className="w-4 h-4" />
            </Button>
          )}
        </div>
      }
    >
      <PermissionsTable user_target={{ type: "UserGroup", id: group_id }} />
    </Page>
  );
};

export const UserPage = () => {
  const { toast } = useToast();
  const inv = useInvalidate();
  const user_id = useParams().id as string;
  const user = useRead("ListUsers", {}).data?.find(
    (user) => user._id?.$oid === user_id
  );
  const { mutate } = useWrite("UpdateUserBasePermissions", {
    onSuccess: () => inv(["ListUsers"]),
    onError: (e) => {
      console.log(e);
      toast({ title: "Failed to update user permissions" });
    },
  });
  const enabledClass = user?.enabled ? "text-green-500" : "text-red-500";
  const avatar = (user?.config.data as any)?.avatar as string | undefined;
  if (!user) return null;
  return (
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
            {(["Server", "Build"] as Array<"Server" | "Build">).map((item) => {
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
                    onClick={() => mutate({ user_id, [req_key]: !user[key] })}
                  />
                </div>
              );
            })}
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
      {!user.admin && (
        <PermissionsTable user_target={{ type: "User", id: user_id }} />
      )}
    </Page>
  );
};
