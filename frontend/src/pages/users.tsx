import { Page, Section } from "@components/layouts";
import { ConfirmButton } from "@components/util";
import { useInvalidate, useRead, useWrite } from "@lib/hooks";
import { DataTable } from "@ui/data-table";
import { Label } from "@ui/label";
import { Switch } from "@ui/switch";
import { useToast } from "@ui/use-toast";
import { UserCheck, UserMinus } from "lucide-react";
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
          { header: "Enabled", accessorKey: "enabled" },
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
  return (
    user && (
      <Page
        title={user?.username}
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
        <PermissionsTable />
      </Page>
    )
  );
};

const PermissionsTable = () => {
  const inv = useInvalidate();
  const user_id = useParams().id as string;
  // const permissions = useRead("")
  return (
    <Section title="Permissions">
      <DataTable data={[]} columns={[]} />
    </Section>
  );
}