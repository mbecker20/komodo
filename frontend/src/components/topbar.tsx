import { useRead, useResourceParamType, useUser } from "@lib/hooks";
import { ResourceComponents } from "./resources";
import {
  AlertTriangle,
  Bell,
  Box,
  Boxes,
  FileQuestion,
  FolderTree,
  Home,
  Key,
  SearchX,
  Tag,
  UserCircle2,
  Users,
} from "lucide-react";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuGroup,
  DropdownMenuItem,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from "@ui/dropdown-menu";
import { Button } from "@ui/button";
import { Link, useNavigate, useParams } from "react-router-dom";
import { RESOURCE_TARGETS, usableResourcePath } from "@lib/utils";
import { Omnibar } from "./omnibar";
import { WsStatusIndicator } from "@lib/socket";
import { TopbarUpdates } from "./updates/topbar";
import { Logout } from "./util";
import { ThemeToggle } from "@ui/theme";
import { UsableResource } from "@types";
import { atomWithStorage } from "jotai/utils";
import { useAtom } from "jotai";
import { Popover, PopoverContent, PopoverTrigger } from "@ui/popover";
import { ReactNode, useState } from "react";
import {
  Command,
  CommandEmpty,
  CommandGroup,
  CommandInput,
  CommandItem,
  CommandList,
} from "@ui/command";
import { ResourceLink } from "./resources/common";

export const Topbar = () => {
  return (
    <div className="sticky top-0 h-[70px] border-b z-50 w-full bg-card text-card-foreground shadow flex items-center">
      <div className="w-full flex items-center justify-between p-4 gap-8">
        <div className="flex items-center gap-4">
          <Link to={"/"} className="text-2xl tracking-widest mx-8">
            MONITOR
          </Link>
          <div className="flex gap-2">
            <PrimaryDropdown />
            <SecondaryDropdown />
          </div>
        </div>
        <div className="flex md:gap-4">
          <Omnibar />
          <div className="flex">
            <WsStatusIndicator />
            <TopbarUpdates />
            <ThemeToggle />
            {/* <UserSettings /> */}
            <Logout />
          </div>
        </div>
      </div>
    </div>
  );
};

const PrimaryDropdown = () => {
  const user = useUser().data;

  const type = useResourceParamType();
  const Components = type && ResourceComponents[type];

  const [icon, title] = Components
    ? [
        <Components.Icon />,
        (type === "ServerTemplate" ? "Template" : type) + "s",
      ]
    : location.pathname === "/"
    ? [<Home className="w-4 h-4" />, "Home"]
    : location.pathname === "/keys"
    ? [<Key className="w-4 h-4" />, "Api Keys"]
    : location.pathname === "/tags"
    ? [<Tag className="w-4 h-4" />, "Tags"]
    : location.pathname === "/alerts"
    ? [<AlertTriangle className="w-4 h-4" />, "Alerts"]
    : location.pathname === "/updates"
    ? [<Bell className="w-4 h-4" />, "Updates"]
    : location.pathname.split("/")[1] === "user-groups"
    ? [<Users className="w-4 h-4" />, "User Groups"]
    : location.pathname === "/users" ||
      location.pathname.split("/")[1] === "users"
    ? [<UserCircle2 className="w-4 h-4" />, "Users"]
    : [<FileQuestion className="w-4 h-4" />, "Unknown"];
  // : [<Box className="w-4 h-4" />, "Dashboard"];

  return (
    <DropdownMenu>
      <DropdownMenuTrigger asChild className="lg:hidden">
        <Button
          variant="ghost"
          className="flex justify-start items-center gap-2 w-36 px-3"
        >
          {icon}
          {title}
        </Button>
      </DropdownMenuTrigger>
      <DropdownMenuContent className="w-36" side="bottom">
        <DropdownMenuGroup>
          <DropdownLinkItem
            label="Home"
            icon={<Home className="w-4 h-4" />}
            to="/"
          />

          <DropdownMenuSeparator />

          {RESOURCE_TARGETS.map((type) => {
            const RTIcon = ResourceComponents[type].Icon;
            const name = type === "ServerTemplate" ? "Template" : type;
            return (
              <DropdownLinkItem
                key={type}
                label={`${name}s`}
                icon={<RTIcon />}
                to={`/${usableResourcePath(type)}`}
              />
            );
          })}

          <DropdownMenuSeparator />

          <DropdownLinkItem
            label="Alerts"
            icon={<AlertTriangle className="w-4 h-4" />}
            to="/alerts"
          />

          <DropdownLinkItem
            label="Updates"
            icon={<Bell className="w-4 h-4" />}
            to="/updates"
          />

          <DropdownLinkItem
            label="Tags"
            icon={<Tag className="w-4 h-4" />}
            to="/tags"
          />

          <DropdownMenuSeparator />

          <DropdownLinkItem
            label="Api Keys"
            icon={<Box className="w-4 h-4" />}
            to="/keys"
          />

          {user?.admin && (
            <DropdownLinkItem
              label="Users"
              icon={<UserCircle2 className="w-4 h-4" />}
              to="/users"
            />
          )}
        </DropdownMenuGroup>
      </DropdownMenuContent>
    </DropdownMenu>
  );
};

const DropdownLinkItem = ({
  label,
  icon,
  to,
}: {
  label: string;
  icon: ReactNode;
  to: string;
}) => {
  return (
    <Link to={to}>
      <DropdownMenuItem className="flex items-center gap-2 cursor-pointer">
        {icon}
        {label}
      </DropdownMenuItem>
    </Link>
  );
};

export type HomeView = "Dashboard" | "Tree" | "Resources";

export const homeViewAtom = atomWithStorage<HomeView>(
  "home-view-v1",
  "Dashboard"
);

const ICONS = {
  Dashboard: () => <Box className="w-4 h-4" />,
  Tree: () => <FolderTree className="w-4 h-4" />,
  Resources: () => <Boxes className="w-4 h-4" />,
};

const SecondaryDropdown = () => {
  const [view, setView] = useAtom(homeViewAtom);

  const type = useResourceParamType();
  if (type) return <ResourcesDropdown type={type} />;

  if (location.pathname === "/") {
    const Icon = ICONS[view];

    return (
      <DropdownMenu>
        <DropdownMenuTrigger asChild>
          <Button
            variant="ghost"
            className="flex justify-start items-center gap-2 w-48 px-3"
          >
            <Icon />
            {view}
          </Button>
        </DropdownMenuTrigger>
        <DropdownMenuContent className="w-48" side="bottom">
          <DropdownMenuGroup>
            {Object.entries(ICONS).map(([view, Icon]) => (
              <DropdownMenuItem
                key={view}
                className="flex items-center gap-2"
                onClick={() => setView(view as HomeView)}
              >
                <Icon />
                {view}
              </DropdownMenuItem>
            ))}
          </DropdownMenuGroup>
        </DropdownMenuContent>
      </DropdownMenu>
    );
  }

  const [_, base, id] = location.pathname.split("/");

  if (base === "users") {
    return <UsersDropdown user_id={id} />;
  } else if (base === "user-groups") {
    return <UserGroupDropdown group_id={id} />;
  }
};

const ResourcesDropdown = ({ type }: { type: UsableResource }) => {
  const nav = useNavigate();
  const id = useParams().id as string;
  const list = useRead(`List${type}s`, {}).data;

  const [open, setOpen] = useState(false);
  const [search, setSearch] = useState("");

  const selected = list?.find((i) => i.id === id);
  const Components = ResourceComponents[type];

  return (
    <Popover open={open} onOpenChange={setOpen}>
      <PopoverTrigger asChild>
        <Button
          variant="ghost"
          className="flex justify-start items-center gap-2 w-48 px-3"
        >
          <Components.Icon id={selected?.id} />
          {selected ? selected.name : `All ${type}s`}
        </Button>
      </PopoverTrigger>
      <PopoverContent className="w-[300px] max-h-[400px] p-0" align="start">
        <Command>
          <CommandInput
            placeholder={`Search ${type}s`}
            className="h-9"
            value={search}
            onValueChange={setSearch}
          />
          <CommandList>
            <CommandEmpty className="flex justify-evenly items-center">
              {`No ${type}s Found`}
              <SearchX className="w-3 h-3" />
            </CommandEmpty>

            <CommandGroup>
              <CommandItem
                onSelect={() => {
                  setOpen(false);
                  nav(`/${usableResourcePath(type)}`);
                }}
              >
                <Button variant="link" className="flex gap-2 items-center p-0">
                  <Components.Icon />
                  All {type}s
                </Button>
              </CommandItem>
              {list?.map((resource) => (
                <CommandItem
                  key={resource.id}
                  onSelect={() => {
                    setOpen(false);
                    nav(`/${usableResourcePath(type)}/${resource.id}`);
                  }}
                >
                  <ResourceLink type={type} id={resource.id} />
                </CommandItem>
              ))}
            </CommandGroup>
          </CommandList>
        </Command>
      </PopoverContent>
    </Popover>
  );
};

const UserGroupDropdown = ({ group_id }: { group_id: string | undefined }) => {
  const nav = useNavigate();
  const [open, setOpen] = useState(false);
  const [input, setInput] = useState("");

  const groups = useRead("ListUserGroups", {}).data;

  const selected = group_id
    ? groups?.find((user) => user._id?.$oid === group_id)
    : undefined;

  return (
    <Popover open={open} onOpenChange={setOpen}>
      <PopoverTrigger asChild>
        <Button
          variant="ghost"
          className="flex justify-start items-center gap-2 w-48 px-3"
        >
          <Users className="w-4 h-4" />
          {selected ? selected.name : "All User Groups"}
        </Button>
      </PopoverTrigger>
      <PopoverContent className="w-[300px] max-h-[400px] p-0" sideOffset={12}>
        <Command>
          <CommandInput
            placeholder="Search User Groups"
            className="h-9"
            value={input}
            onValueChange={setInput}
          />
          <CommandList>
            <CommandEmpty className="flex justify-evenly items-center">
              No User Groups Found
              <SearchX className="w-3 h-3" />
            </CommandEmpty>

            <CommandGroup>
              <CommandItem
                onSelect={() => {
                  setOpen(false);
                  nav(`/users`);
                }}
              >
                <Button variant="link" className="flex gap-2 items-center p-0">
                  <UserCircle2 className="w-4" />
                  All User Groups
                </Button>
              </CommandItem>
              {groups?.map((group) => (
                <CommandItem
                  key={group.name}
                  onSelect={() => {
                    setOpen(false);
                    nav(`/user-groups/${group._id?.$oid}`);
                  }}
                >
                  <Button
                    variant="link"
                    className="flex gap-2 items-center p-0"
                  >
                    <Users className="w-4 h-4" />
                    {group.name}
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

const UsersDropdown = ({ user_id }: { user_id: string | undefined }) => {
  const nav = useNavigate();
  const [open, setOpen] = useState(false);
  const [input, setInput] = useState("");

  const users = useRead("ListUsers", {}).data;

  const selected = user_id
    ? users?.find((user) => user._id?.$oid === user_id)
    : undefined;
  const avatar = (selected?.config.data as { avatar?: string })?.avatar;

  return (
    <Popover open={open} onOpenChange={setOpen}>
      <PopoverTrigger asChild>
        <Button
          variant="ghost"
          className="flex justify-start items-center gap-2 w-48 px-3"
        >
          <UserAvatar avatar={avatar} />
          {selected ? selected.username : "All Users"}
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
              <CommandItem
                onSelect={() => {
                  setOpen(false);
                  nav(`/users`);
                }}
              >
                <Button variant="link" className="flex gap-2 items-center p-0">
                  <UserCircle2 className="w-4" />
                  All Users
                </Button>
              </CommandItem>
              {users?.map((user) => (
                <CommandItem
                  key={user.username}
                  onSelect={() => {
                    setOpen(false);
                    nav(`/users/${user._id?.$oid}`);
                  }}
                >
                  <Button
                    variant="link"
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
