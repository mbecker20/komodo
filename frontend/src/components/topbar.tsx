import { useRead, useResourceParamType, useShiftKeyListener } from "@lib/hooks";
import { ResourceComponents } from "./resources";
import {
  AlertTriangle,
  Bell,
  Box,
  Boxes,
  FileQuestion,
  FolderTree,
  Keyboard,
  Settings,
  User,
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
import { Link } from "react-router-dom";
import { RESOURCE_TARGETS, usableResourcePath } from "@lib/utils";
import { OmniSearch, OmniDialog } from "./omnibar";
import { WsStatusIndicator } from "@lib/socket";
import { TopbarUpdates } from "./updates/topbar";
import { Logout } from "./util";
import { ThemeToggle } from "@ui/theme";
import { useAtom } from "jotai";
import { ReactNode, useState } from "react";
import { HomeView, homeViewAtom } from "@main";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from "@ui/dialog";
import { Badge } from "@ui/badge";

export const Topbar = () => {
  const [omniOpen, setOmniOpen] = useState(false);
  useShiftKeyListener("S", () => setOmniOpen(true));
  const version = useRead("GetVersion", {}).data?.version;
  return (
    <div className="sticky top-0 h-[70px] border-b z-50 w-full bg-card text-card-foreground shadow flex items-center">
      <div className="w-full p-4 grid grid-cols-2 lg:grid-cols-3">
        <div className="flex items-center justify-self-start w-fit gap-0 md:gap-4">
          <Link
            to="/"
            className="flex gap-3 items-center text-2xl tracking-widest md:mx-2"
          >
            <img
              src="/monitor-circle.png"
              className="w-[28px] dark:invert"
            />
            <div className="hidden md:block">MONITOR</div>
          </Link>
          <MobileDropdown />
        </div>
        <OmniSearch
          setOpen={setOmniOpen}
          className="hidden lg:flex justify-self-center"
        />
        <div className="flex md:gap-2 justify-self-end items-center">
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
          <KeyboardShortcuts />
          <TopbarUpdates />
          <ThemeToggle />
          <Logout />
        </div>
        <OmniDialog open={omniOpen} setOpen={setOmniOpen} />
      </div>
    </div>
  );
};

const MobileDropdown = () => {
  const type = useResourceParamType();
  const Components = type && ResourceComponents[type];
  const [view, setView] = useAtom<HomeView>(homeViewAtom);

  const [icon, title] = Components
    ? [
        <Components.Icon />,
        (type === "ServerTemplate"
          ? "Template"
          : type === "ResourceSync"
          ? "Sync"
          : type) + "s",
      ]
    : location.pathname === "/" && view === "Dashboard"
    ? [<Box className="w-4 h-4" />, "Dashboard"]
    : location.pathname === "/" && view === "Resources"
    ? [<Boxes className="w-4 h-4" />, "Resources"]
    : location.pathname === "/" && view === "Tree"
    ? [<FolderTree className="w-4 h-4" />, "Tree"]
    : location.pathname === "/settings"
    ? [<Settings className="w-4 h-4" />, "Settings"]
    : location.pathname === "/alerts"
    ? [<AlertTriangle className="w-4 h-4" />, "Alerts"]
    : location.pathname === "/updates"
    ? [<Bell className="w-4 h-4" />, "Updates"]
    : location.pathname.split("/")[1] === "user-groups"
    ? [<Users className="w-4 h-4" />, "User Groups"]
    : location.pathname.split("/")[1] === "users"
    ? [<User className="w-4 h-4" />, "Users"]
    : [<FileQuestion className="w-4 h-4" />, "Unknown"];

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
            label="Dashboard"
            icon={<Box className="w-4 h-4" />}
            to="/"
            onClick={() => setView("Dashboard")}
          />
          <DropdownLinkItem
            label="Resources"
            icon={<Boxes className="w-4 h-4" />}
            to="/"
            onClick={() => setView("Resources")}
          />
          <DropdownLinkItem
            label="Tree"
            icon={<FolderTree className="w-4 h-4" />}
            to="/"
            onClick={() => setView("Tree")}
          />

          <DropdownMenuSeparator />

          {RESOURCE_TARGETS.map((type) => {
            const RTIcon = ResourceComponents[type].Icon;
            const name =
              type === "ServerTemplate"
                ? "Template"
                : type === "ResourceSync"
                ? "Sync"
                : type;
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

          <DropdownMenuSeparator />

          <DropdownLinkItem
            label="Settings"
            icon={<Settings className="w-4 h-4" />}
            to="/settings"
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
  onClick,
}: {
  label: string;
  icon: ReactNode;
  to: string;
  onClick?: () => void;
}) => {
  return (
    <Link to={to} onClick={onClick}>
      <DropdownMenuItem className="flex items-center gap-2 cursor-pointer">
        {icon}
        {label}
      </DropdownMenuItem>
    </Link>
  );
};

const KeyboardShortcuts = () => {
  return (
    <Dialog>
      <DialogTrigger asChild>
        <Button variant="ghost" className="hidden md:flex items-center gap-2">
          <Keyboard className="w-4 h-4" />
        </Button>
      </DialogTrigger>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Keyboard Shortcuts</DialogTitle>
        </DialogHeader>
        <div className="grid gap-3 grid-cols-2 pt-8">
          <KeyboardShortcut label="Go Home" keys={["Shift", "H"]} />

          <KeyboardShortcut label="Go to Servers" keys={["Shift", "G"]} />
          <KeyboardShortcut label="Go to Deployments" keys={["Shift", "D"]} />
          <KeyboardShortcut label="Go to Builds" keys={["Shift", "B"]} />
          <KeyboardShortcut label="Go to Repos" keys={["Shift", "R"]} />
          <KeyboardShortcut label="Go to Procedures" keys={["Shift", "P"]} />

          <KeyboardShortcut label="Search" keys={["Shift", "S"]} />
          <KeyboardShortcut label="Add Filter Tag" keys={["Shift", "T"]} />
          <KeyboardShortcut
            label="Clear Filter Tags"
            keys={["Shift", "C"]}
            divider={false}
          />
        </div>
      </DialogContent>
    </Dialog>
  );
};

const KeyboardShortcut = ({
  label,
  keys,
  divider = true,
}: {
  label: string;
  keys: string[];
  divider?: boolean;
}) => {
  return (
    <>
      <div>{label}</div>
      <div className="flex items-center gap-2">
        {keys.map((key) => (
          <Badge variant="secondary" key={key}>
            {key}
          </Badge>
        ))}
      </div>

      {divider && (
        <div className="col-span-full bg-gray-600 h-[1px] opacity-40" />
      )}
    </>
  );
};
