import { useRead, useResourceParamType } from "@lib/hooks";
import { ResourceComponents } from "./resources";
import {
  Box,
  Boxes,
  FolderTree,
  Key,
  Moon,
  SunMedium,
  Tag,
  UserCircle2,
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
import { Link, useParams } from "react-router-dom";
import { RESOURCE_TARGETS } from "@lib/utils";
import { Omnibar } from "./omnibar";
import { WsStatusIndicator } from "@lib/socket";
import { HeaderUpdates } from "./updates/header";
import { useEffect, useState } from "react";
import { Logout } from "./util";

export const Topbar = () => {
  const type = useResourceParamType();
  return (
    <div className="sticky top-0 border-b bg-background z-50 w-full">
      <div className="container flex items-center justify-between py-4 gap-8">
        <div className="flex items-center gap-4">
          <Link to={"/"} className="text-2xl tracking-widest">
            MONITOR
          </Link>
          <div className="flex gap-2">
            <ResourceTypeDropdown />
            {type && <ResourcesDropdown />}
          </div>
        </div>
        <div className="flex md:gap-4">
          <Omnibar />
          <div className="flex">
            <WsStatusIndicator />
            <HeaderUpdates />
            <ThemeToggle />
            {/* <UserSettings /> */}
            <Logout />
          </div>
        </div>
      </div>
    </div>
  );
};

const ThemeToggle = () => {
  const [theme, set] = useState(localStorage.getItem("theme"));

  useEffect(() => {
    localStorage.setItem("theme", theme ?? "dark");
    if (theme === "dark") document.body.classList.remove("dark");
    else document.body.classList.add("dark");
  }, [theme]);

  return (
    <Button
      variant="ghost"
      onClick={() => set(theme === "dark" ? "light" : "dark")}
    >
      <SunMedium className="w-4 h-4 rotate-0 scale-100 transition-all dark:-rotate-90 dark:scale-0" />
      <Moon className="w-4 h-4 absolute rotate-90 scale-0 transition-all dark:rotate-0 dark:scale-100" />
    </Button>
  );
};

const ResourceTypeDropdown = () => {
  const type = useResourceParamType();
  const Components = ResourceComponents[type];

  const [icon, title] = type
    ? [<Components.Icon />, type + "s"]
    : location.pathname === "/tree"
    ? [<FolderTree className="w-4 h-4" />, "Tree"]
    : location.pathname === "/keys"
    ? [<Key className="w-4 h-4" />, "Api Keys"]
    : location.pathname === "/tags"
    ? [<Tag className="w-4 h-4" />, "Tags"]
    : location.pathname === "/users"
    ? [<UserCircle2 className="w-4 h-4" />, "Users"]
    : [<Box className="w-4 h-4" />, "Dashboard"];

  return (
    <DropdownMenu>
      <DropdownMenuTrigger asChild>
        <Button variant="ghost" className="w-36 justify-between px-3">
          <div className="flex items-center gap-2">
            {icon}
            {title}
          </div>
        </Button>
      </DropdownMenuTrigger>
      <DropdownMenuContent className="w-36" side="bottom">
        <DropdownMenuGroup>
          <Link to="/">
            <DropdownMenuItem className="flex items-center gap-2 cursor-pointer">
              <Box className="w-4 h-4" />
              Dashboard
            </DropdownMenuItem>
          </Link>

          <DropdownMenuSeparator />

          <Link to="/resources">
            <DropdownMenuItem className="flex items-center gap-2 cursor-pointer">
              <Boxes className="w-4 h-4" />
              Resources
            </DropdownMenuItem>
          </Link>
          <Link to="/tree">
            <DropdownMenuItem className="flex items-center gap-2 cursor-pointer">
              <FolderTree className="w-4 h-4" />
              Tree
            </DropdownMenuItem>
          </Link>

          <DropdownMenuSeparator />

          {RESOURCE_TARGETS.map((rt) => {
            const RTIcon = ResourceComponents[rt].Icon;
            return (
              <Link key={rt} to={`/${rt.toLowerCase()}s`}>
                <DropdownMenuItem className="flex items-center gap-2 cursor-pointer">
                  <RTIcon />
                  {rt}s
                </DropdownMenuItem>
              </Link>
            );
          })}

          <DropdownMenuSeparator />

          <Link to="/tags">
            <DropdownMenuItem className="flex items-center gap-2 cursor-pointer">
              <Tag className="w-4 h-4" />
              Tags
            </DropdownMenuItem>
          </Link>

          <DropdownMenuSeparator />

          <Link to="/keys">
            <DropdownMenuItem className="flex items-center gap-2 cursor-pointer">
              <Box className="w-4 h-4" />
              Api Keys
            </DropdownMenuItem>
          </Link>
          <Link to="/users">
            <DropdownMenuItem className="flex items-center gap-2 cursor-pointer">
              <UserCircle2 className="w-4 h-4" />
              Users
            </DropdownMenuItem>
          </Link>
        </DropdownMenuGroup>
      </DropdownMenuContent>
    </DropdownMenu>
  );
};

const ResourcesDropdown = () => {
  const type = useResourceParamType();
  const id = useParams().id as string;
  const list = useRead(`List${type}s`, {}).data;

  const selected = list?.find((i) => i.id === id);
  const Components = ResourceComponents[type];

  return (
    <DropdownMenu>
      <DropdownMenuTrigger asChild>
        <Button variant="ghost" className="w-48 justify-between px-3">
          <div className="flex items-center gap-2">
            <Components.Icon id={selected?.id} />
            {selected ? selected.name : `All ${type}s`}
          </div>
        </Button>
      </DropdownMenuTrigger>
      <DropdownMenuContent className="w-48" side="bottom">
        <DropdownMenuGroup>
          <Link to={`/${type.toLowerCase()}s`}>
            <DropdownMenuItem className="flex items-center gap-2">
              <Components.Icon />
              All {type}s
            </DropdownMenuItem>
          </Link>
        </DropdownMenuGroup>
        <DropdownMenuGroup>
          {!list?.length && (
            <DropdownMenuItem disabled>No {type}s Found.</DropdownMenuItem>
          )}

          {list?.map(({ id, name }) => (
            <Link key={id} to={`/${type.toLowerCase()}s/${id}`}>
              <DropdownMenuItem className="flex items-center gap-2">
                <Components.Icon id={id} />
                {name}
              </DropdownMenuItem>
            </Link>
          ))}
        </DropdownMenuGroup>
      </DropdownMenuContent>
    </DropdownMenu>
  );
};
