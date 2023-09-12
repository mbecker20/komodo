import { Section } from "./layouts";
import { Lock, PlusCircle, User } from "lucide-react";
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

const Username = ({ user_id }: { user_id: string }) => {
  const username = useRead("GetUsername", { user_id }).data?.username;
  return <>{username}</>;
};

const NewPermission = () => {
  const [open, set] = useState(false);

  //   const users = useRead("Lis");

  //   const { mutate: update, isLoading } = useWrite(
  //     "UpdateUserPermissionsOnTarget"
  //   );

  return (
    <Dialog open={open} onOpenChange={set}>
      <DialogTrigger asChild disabled>
        <Button size="icon">
          <PlusCircle className="w-4 h-4" />
        </Button>
      </DialogTrigger>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Add User</DialogTitle>
        </DialogHeader>
        <div></div>
        <DialogFooter></DialogFooter>
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
  const permissions = useRead(`Get${type}`, { id }, { enabled: admin }).data
    ?.permissions;
  const { mutate: update, isLoading } = useWrite(
    "UpdateUserPermissionsOnTarget"
  );

  if (!admin) return null;

  return (
    <Section
      title="Permissions"
      icon={<Lock className="w-4 h-4" />}
      actions={<NewPermission />}
    >
      <div className="grid md:grid-cols-2 lg:grid-cols-3 gap-4">
        {Object.keys(permissions ?? {}).map((user_id) => {
          return (
            <Card key={user_id}>
              <CardHeader className="justify-between">
                <CardTitle className="flex items-center gap-2">
                  <User className="w-4 h-4" />
                  <Username user_id={user_id} />
                </CardTitle>
                <Select
                  value={permissions?.[user_id]}
                  onValueChange={(p) =>
                    update({
                      permission: p as Types.PermissionLevel,
                      user_id,
                      target: { type, id },
                    })
                  }
                  disabled={isLoading}
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
          );
        })}
      </div>
    </Section>
  );
};
