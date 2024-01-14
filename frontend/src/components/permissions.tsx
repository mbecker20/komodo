import { Section } from "./layouts";
import { Loader2, Lock, PlusCircle, User } from "lucide-react";
import { useRead, useWrite } from "@lib/hooks";
import { UsableResource } from "@types";
import { Card, CardHeader, CardTitle } from "@ui/card";
import {
  Select,
  SelectContent,
  SelectGroup,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@ui/select";
import { Types } from "@monitor/client";
import {
  Dialog,
  DialogContent,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from "@ui/dialog";
import { Button } from "@ui/button";
import { useState } from "react";
import { ResourceTarget } from "@monitor/client/dist/types";

const Username = ({ user_id }: { user_id: string }) => {
  const username = useRead("GetUsername", { user_id }).data?.username;
  return <>{username}</>;
};

const NewPermission = ({ id, type }: ResourceTarget) => {
  const [open, set] = useState(false);
  const [user_id, setUserId] = useState<string>();
  const [permission, setPermission] = useState<Types.PermissionLevel>();
  const users = useRead("GetUsers", {}).data?.filter((u) => !u.admin);
  const { mutate, isPending } = useWrite("UpdateUserPermissionsOnTarget");

  return (
    <Dialog open={open} onOpenChange={set}>
      <DialogTrigger asChild>
        <Button className="gap-2">
          Add Permission
          <PlusCircle className="w-4 h-4" />
        </Button>
      </DialogTrigger>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Add User</DialogTitle>
        </DialogHeader>
        <div className="grid gap-4 my-4">
          <div className="flex items-center justify-between">
            User
            <Select value={user_id} onValueChange={setUserId}>
              <SelectTrigger className="w-48">
                <SelectValue placeholder="Select User" />
              </SelectTrigger>
              <SelectContent className="w-48">
                <SelectGroup>
                  {users?.map((user) => (
                    <SelectItem key={user._id?.$oid} value={user._id!.$oid}>
                      {user.username}
                    </SelectItem>
                  ))}
                </SelectGroup>
              </SelectContent>
            </Select>
          </div>
          <div className="flex items-center justify-between">
            Permissions Level
            <Select
              value={permission}
              onValueChange={(lv: any) => setPermission(lv as Types.PermissionLevel)}
            >
              <SelectTrigger className="w-48">
                <SelectValue placeholder="Select Permission Level" />
              </SelectTrigger>
              <SelectContent className="w-48">
                <SelectGroup>
                  <SelectItem value={Types.PermissionLevel.Read}>
                    Read
                  </SelectItem>
                  <SelectItem value={Types.PermissionLevel.Update}>
                    Update
                  </SelectItem>
                  <SelectItem value={Types.PermissionLevel.Execute}>
                    Execute
                  </SelectItem>
                </SelectGroup>
              </SelectContent>
            </Select>
          </div>
        </div>
        <DialogFooter className="flex justify-end">
          <Button
            className="gap-2"
            disabled={isPending}
            onClick={() =>
              permission &&
              user_id &&
              mutate({ permission, user_id, target: { id, type } })
            }
          >
            Add Permissions
            {isPending ? (
              <Loader2 className="w-4 h-4 animate-spin" />
            ) : (
              <PlusCircle className="w-4 h-4" />
            )}
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
};

export const ResourcePermissions = ({
  type,
  id,
}: {
  type: UsableResource;
  id: string;
}) => {
  const admin = useRead("GetUser", {}).data?.admin;
  const me = useRead("GetUser", {}).data?._id?.$oid;
  const permissions = useRead(`Get${type}`, { id }, { enabled: admin }).data
    ?.permissions;

  const users = useRead("GetUsers", {}).data?.filter((u) => !u.admin);

  const { mutate: update, isPending } = useWrite(
    "UpdateUserPermissionsOnTarget"
  );

  const display = Object.keys(permissions ?? {})
    .filter((id) => id != me)
    .filter((id) => !users?.find((u) => u._id?.$oid === id)?.admin);

  if (!admin || !display.length) return null;

  return (
    <Section
      title="Permissions"
      icon={<Lock className="w-4 h-4" />}
      actions={<NewPermission id={id} type={type} />}
    >
      <div className="grid md:grid-cols-2 lg:grid-cols-3 gap-4">
        {display.map((user_id) => (
          <Card key={user_id}>
            <CardHeader className="justify-between">
              <CardTitle className="flex items-center gap-2">
                <User className="w-4 h-4" />
                <Username user_id={user_id} />
              </CardTitle>
              <Select
                value={permissions?.[user_id]}
                onValueChange={(p: any) =>
                  update({
                    permission: p as Types.PermissionLevel,
                    user_id,
                    target: { type, id },
                  })
                }
                disabled={isPending}
              >
                <SelectTrigger className="w-32">
                  <SelectValue placeholder="Set Permissions" />
                </SelectTrigger>
                <SelectContent className="w-32">
                  <SelectGroup>
                    <SelectItem value={Types.PermissionLevel.Read}>
                      Read
                    </SelectItem>
                    <SelectItem value={Types.PermissionLevel.Update}>
                      Update
                    </SelectItem>
                    <SelectItem value={Types.PermissionLevel.Execute}>
                      Execute
                    </SelectItem>
                  </SelectGroup>
                </SelectContent>
              </Select>
            </CardHeader>
          </Card>
        ))}
      </div>
    </Section>
  );
};
