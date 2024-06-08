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
  User,
  Users,
  Variable,
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
import {
  RESOURCE_TARGETS,
  filterBySplit,
  usableResourcePath,
} from "@lib/utils";
import { OmniSearch, OmniDialog } from "./omnibar";
import { WsStatusIndicator } from "@lib/socket";
import { TopbarUpdates } from "./updates/topbar";
import { Logout, UserAvatar } from "./util";
import { ThemeToggle } from "@ui/theme";
import { UsableResource } from "@types";
import { useAtom } from "jotai";
import { Popover, PopoverContent, PopoverTrigger } from "@ui/popover";
import { ReactNode, useEffect, useState } from "react";
import {
  Command,
  CommandEmpty,
  CommandGroup,
  CommandInput,
  CommandItem,
  CommandList,
} from "@ui/command";
import { ResourceLink } from "./resources/common";
import { HomeView, homeViewAtom } from "@main";
import { Types } from "@monitor/client";

export const Topbar = () => {
  const [omniOpen, setOmniOpen] = useState(false);
  useEffect(() => {
    const keydown = (e: KeyboardEvent) => {
      // This will ignore Shift + S if it is sent from input / textarea
      const target = e.target as any;
      if (target.matches("input") || target.matches("textarea")) return;

      if (e.shiftKey && e.key === "S") {
        e.preventDefault();
        setOmniOpen(true);
      }
    };
    document.addEventListener("keydown", keydown);
    return () => document.removeEventListener("keydown", keydown);
  });
  const version = useRead("GetVersion", {}).data?.version;
  return (
    <div className="sticky top-0 h-[70px] border-b z-50 w-full bg-card text-card-foreground shadow flex items-center">
      <div className="w-full p-4 grid grid-cols-2 lg:grid-cols-3">
        <div className="flex items-center gap-4 justify-self-start w-fit">
          <Link
            to={"/"}
            className="flex gap-3 items-start text-2xl tracking-widest lg:mx-2"
          >
            <img
              src="/monitor-lizard.png"
              className="w-9 h-7 dark:invert hidden lg:block"
            />
            MONITOR
          </Link>
          <div className="flex gap-2">
            <PrimaryDropdown />
            <SecondaryDropdown />
          </div>
        </div>
        <OmniSearch
          setOpen={setOmniOpen}
          className="hidden lg:flex justify-self-center"
        />
        <div className="flex md:gap-2 justify-self-end items-center">
          {/* <div className="hidden lg:block px-2 text-muted-foreground cursor-default">
            v{version}
          </div> */}
          <a
            href="https://docs.monitor.mogh.tech"
            target="_blank"
            className="hidden lg:block"
          >
            <Button variant="link" className="text-muted-foreground p-2">
              <div>v{version ? version : "x.x.x"}</div>
            </Button>
          </a>
          <OmniSearch setOpen={setOmniOpen} className="lg:hidden" />
          <WsStatusIndicator />
          <TopbarUpdates />
          <ThemeToggle />
          <Logout />
        </div>
        <OmniDialog open={omniOpen} setOpen={setOmniOpen} />
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
        (type === "ServerTemplate"
          ? "Template"
          : type === "ResourceSync"
          ? "Sync"
          : type) + "s",
      ]
    : location.pathname === "/"
    ? [<Home className="w-4 h-4" />, "Home"]
    : location.pathname === "/keys"
    ? [<Key className="w-4 h-4" />, "Api Keys"]
    : location.pathname === "/tags"
    ? [<Tag className="w-4 h-4" />, "Tags"]
    : location.pathname === "/variables"
    ? [<Variable className="w-4 h-4" />, "Variables"]
    : location.pathname === "/alerts"
    ? [<AlertTriangle className="w-4 h-4" />, "Alerts"]
    : location.pathname === "/updates"
    ? [<Bell className="w-4 h-4" />, "Updates"]
    : location.pathname.split("/")[1] === "user-groups"
    ? [<Users className="w-4 h-4" />, "User Groups"]
    : location.pathname === "/users" ||
      location.pathname.split("/")[1] === "users"
    ? [<User className="w-4 h-4" />, "Users"]
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
      <DropdownMenuContent className="w-36" side="bottom" align="start">
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
            label="Variables"
            icon={<Variable className="w-4 h-4" />}
            to="/variables"
          />

          <DropdownLinkItem
            label="Tags"
            icon={<Tag className="w-4 h-4" />}
            to="/tags"
          />

          <DropdownMenuSeparator />

          {user?.admin && (
            <DropdownLinkItem
              label="Users"
              icon={<User className="w-4 h-4" />}
              to="/users"
            />
          )}

          <DropdownLinkItem
            label="Api Keys"
            icon={<Box className="w-4 h-4" />}
            to="/keys"
          />
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

const ICONS = {
  Dashboard: () => <Box className="w-4 h-4" />,
  Resources: () => <Boxes className="w-4 h-4" />,
  Tree: () => <FolderTree className="w-4 h-4" />,
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
            className="hidden sm:flex lg:hidden justify-start items-center gap-2 w-48 px-3"
          >
            <Icon />
            {view}
          </Button>
        </DropdownMenuTrigger>
        <DropdownMenuContent className="w-48" side="bottom" align="start">
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
  const list = useRead(`List${type}s`, {}).data ?? [];

  const [open, setOpen] = useState(false);
  const [search, setSearch] = useState("");

  const selected = list?.find((i) => i.id === id);
  const Components = ResourceComponents[type];

  const filtered = filterBySplit(
    list as Types.ResourceListItem<unknown>[],
    search,
    (item) => item.name
  );

  return (
    <Popover open={open} onOpenChange={setOpen}>
      <PopoverTrigger asChild>
        <Button
          variant="ghost"
          className="hidden sm:flex justify-start items-center gap-2 w-48 px-3"
        >
          <Components.Icon id={selected?.id} />
          {selected ? selected.name : `All ${type}s`}
        </Button>
      </PopoverTrigger>
      <PopoverContent className="w-[300px] max-h-[400px] p-0" align="start">
        <Command shouldFilter={false}>
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
              {filtered?.map((resource) => (
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
  const [search, setSearch] = useState("");

  const groups = useRead("ListUserGroups", {}).data ?? [];

  const selected = group_id
    ? groups?.find((user) => user._id?.$oid === group_id)
    : undefined;

  const filtered = filterBySplit(
    groups as Types.UserGroup[],
    search,
    (item) => item.name
  );

  return (
    <Popover open={open} onOpenChange={setOpen}>
      <PopoverTrigger asChild>
        <Button
          variant="ghost"
          className="hidden sm:flex justify-start items-center gap-2 w-48 px-3"
        >
          <Users className="w-4 h-4" />
          {selected ? selected.name : "All User Groups"}
        </Button>
      </PopoverTrigger>
      <PopoverContent className="w-[300px] max-h-[400px] p-0" sideOffset={12}>
        <Command shouldFilter={false}>
          <CommandInput
            placeholder="Search User Groups"
            className="h-9"
            value={search}
            onValueChange={setSearch}
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
                  <User className="w-4" />
                  All User Groups
                </Button>
              </CommandItem>
              {filtered?.map((group) => (
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
  const [search, setSearch] = useState("");

  const users = useRead("ListUsers", {}).data ?? [];

  const selected = user_id
    ? users?.find((user) => user._id?.$oid === user_id)
    : undefined;
  const avatar = (selected?.config.data as { avatar?: string })?.avatar;

  const filtered = filterBySplit(
    users as Types.User[],
    search,
    (item) => item.username
  );

  return (
    <Popover open={open} onOpenChange={setOpen}>
      <PopoverTrigger asChild>
        <Button
          variant="ghost"
          className="hidden sm:flex justify-start items-center gap-2 w-48 px-3"
        >
          <UserAvatar avatar={avatar} />
          {selected ? selected.username : "All Users"}
        </Button>
      </PopoverTrigger>
      <PopoverContent className="w-[300px] max-h-[400px] p-0" sideOffset={12}>
        <Command shouldFilter={false}>
          <CommandInput
            placeholder="Search Users"
            className="h-9"
            value={search}
            onValueChange={setSearch}
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
                  <User className="w-4" />
                  All Users
                </Button>
              </CommandItem>
              {filtered?.map((user) => (
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
