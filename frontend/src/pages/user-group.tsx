import { Page, Section } from "@components/layouts";
import { PermissionsTable } from "@components/users/permissions-table";
import { UserTable } from "@components/users/table";
import { useInvalidate, useRead, useWrite } from "@lib/hooks";
import { Button } from "@ui/button";
import {
  Command,
  CommandEmpty,
  CommandGroup,
  CommandInput,
  CommandItem,
  CommandList,
} from "@ui/command";
import { Input } from "@ui/input";
import { Popover, PopoverContent, PopoverTrigger } from "@ui/popover";
import { useToast } from "@ui/use-toast";
import { PlusCircle, Save, SearchX, UserCircle2 } from "lucide-react";
import { useEffect, useState } from "react";
import { useParams } from "react-router-dom";

export const UserGroupPage = () => {
  const { toast } = useToast();
  const inv = useInvalidate();
  const group_id = useParams().id as string;
  const group = useRead("ListUserGroups", {}).data?.find(
    (group) => group._id?.$oid === group_id
  );
  const users = useRead("ListUsers", {}).data;
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
  const removeMutate = useWrite("RemoveUserFromUserGroup", {
    onSuccess: () => {
      inv(["ListUserGroups"]);
      toast({ title: "Removed User from User Group" });
    },
  }).mutate;
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
      <Section
        title="Users"
        icon={<UserCircle2 className="w-4 h-4" />}
        actions={<AddUserToGroup group_id={group_id} />}
      >
        <UserTable
          users={
            users?.filter((user) =>
              group ? group.users.includes(user._id?.$oid!) : false
            ) ?? []
          }
          onUserRemove={(user_id) =>
            removeMutate({ user_group: group_id, user_id })
          }
        />
      </Section>
      <PermissionsTable user_target={{ type: "UserGroup", id: group_id }} />
    </Page>
  );
};

const AddUserToGroup = ({ group_id }: { group_id: string }) => {
  const inv = useInvalidate();
  const { toast } = useToast();

  const [open, setOpen] = useState(false);
  const [input, setInput] = useState("");

  const group = useRead("ListUserGroups", {}).data?.find(
    (group) => group._id?.$oid === group_id
  );

  const users = useRead("ListUsers", {}).data?.filter(
    (user) =>
      // Only show users not already in group
      !group?.users.includes(user._id?.$oid!)
  );

  const addUser = useWrite("AddUserToUserGroup", {
    onSuccess: () => {
      inv(["ListUserGroups"]);
      toast({ title: "Added User to User Group" });
    },
  }).mutate;

  if (!users || users.length === 0) return null;

  return (
    <Popover open={open} onOpenChange={setOpen}>
      <PopoverTrigger asChild>
        <Button
          variant="secondary"
          className="flex justify-start items-center gap-2 w-48 px-3"
        >
          <PlusCircle className="w-4 h-4" />
          Add User
        </Button>
      </PopoverTrigger>
      <PopoverContent className="w-[300px] max-h-[400px] p-0" sideOffset={12}>
        <Command>
          <CommandInput
            placeholder="Search Users"
            className="h-9"
            value={input}
            onValueChange={setInput}
          />
          <CommandList>
            <CommandEmpty className="flex justify-evenly items-center">
              No Users Found
              <SearchX className="w-3 h-3" />
            </CommandEmpty>

            <CommandGroup>
              {users?.map((user) => (
                <CommandItem
                  key={user.username}
                  onSelect={() => {
                    setOpen(false);
                    addUser({ user_group: group_id, user_id: user._id?.$oid! });
                  }}
                >
                  <Button
                    variant="ghost"
                    className="flex gap-2 items-center p-0"
                  >
                    <UserAvatar avatar={(user.config.data as any).avatar} />
                    {user.username}
                  </Button>
                </CommandItem>
              ))}
            </CommandGroup>
          </CommandList>
        </Command>
      </PopoverContent>
    </Popover>
  );
};

const UserAvatar = ({ avatar }: { avatar: string | undefined }) =>
  avatar ? (
    <img src={avatar} alt="Avatar" className="w-4" />
  ) : (
    <UserCircle2 className="w-4" />
  );
