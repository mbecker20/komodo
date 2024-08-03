import { RESOURCE_TARGETS, cn, usableResourcePath } from "@lib/utils";
import { Button } from "@ui/button";
import { Card, CardContent } from "@ui/card";
import {
  AlertTriangle,
  Bell,
  Box,
  Boxes,
  FolderTree,
  Settings,
} from "lucide-react";
import { Link, useLocation } from "react-router-dom";
import { ResourceComponents } from "./resources";
import { Separator } from "@ui/separator";
import { ReactNode } from "react";
import { useAtom } from "jotai";
import { homeViewAtom } from "@main";

export const Sidebar = () => {
  const [view, setView] = useAtom(homeViewAtom);
  return (
    <div className="sticky top-24 w-64 h-fir border-r hidden md:flex flex-col gap-1 pr-8 pb-6">
      <p className="pl-4 pb-1 text-xs text-muted-foreground">Overviews</p>
      <SidebarLink
        label="Dashboard"
        to="/"
        icon={<Box className="w-4 h-4" />}
        onClick={() => setView("Dashboard")}
        highlighted={view === "Dashboard"}
      />
      <SidebarLink
        label="Resources"
        to="/"
        icon={<Boxes className="w-4 h-4" />}
        onClick={() => setView("Resources")}
        highlighted={view === "Resources"}
      />
      <SidebarLink
        label="Tree"
        to="/"
        icon={<FolderTree className="w-4 h-4" />}
        onClick={() => setView("Tree")}
        highlighted={view === "Tree"}
      />
      <Separator className="my-3" />

      <p className="pl-4 pb-1 text-xs text-muted-foreground">Resources</p>
      {RESOURCE_TARGETS.map((type) => {
        const RTIcon = ResourceComponents[type].Icon;
        const name =
          type === "ServerTemplate"
            ? "Template"
            : type === "ResourceSync"
            ? "Sync"
            : type;
        return (
          <SidebarLink
            key={type}
            label={`${name}s`}
            to={`/${usableResourcePath(type)}`}
            icon={<RTIcon />}
          />
        );
      })}
      <Separator className="my-3" />

      <p className="pl-4 pb-1 text-xs text-muted-foreground">Notifications</p>
      <SidebarLink
        label="Alerts"
        to="/alerts"
        icon={<AlertTriangle className="w-4 h-4" />}
      />
      <SidebarLink
        label="Updates"
        to="/updates"
        icon={<Bell className="w-4 h-4" />}
      />
      <Separator className="my-3" />

      <SidebarLink
        label="Settings"
        to="/settings"
        icon={<Settings className="w-4 h-4" />}
      />
      {/* <Separator className="mt-3" /> */}
    </div>
  );

  return (
    <Card className="h-fit m-4 hidden lg:flex">
      <CardContent className="h-fit grid gap-[4px] px-6 py-2">
        <SidebarLink
          label="Dashboard"
          to="/"
          icon={<Box className="w-4 h-4" />}
          onClick={() => setView("Dashboard")}
          highlighted={view === "Dashboard"}
        />
        <SidebarLink
          label="Resources"
          to="/"
          icon={<Boxes className="w-4 h-4" />}
          onClick={() => setView("Resources")}
          highlighted={view === "Resources"}
        />
        <SidebarLink
          label="Tree"
          to="/"
          icon={<FolderTree className="w-4 h-4" />}
          onClick={() => setView("Tree")}
          highlighted={view === "Tree"}
        />

        <Separator />

        {RESOURCE_TARGETS.map((type) => {
          const RTIcon = ResourceComponents[type].Icon;
          const name =
            type === "ServerTemplate"
              ? "Template"
              : type === "ResourceSync"
              ? "Sync"
              : type;
          return (
            <SidebarLink
              key={type}
              label={`${name}s`}
              to={`/${usableResourcePath(type)}`}
              icon={<RTIcon />}
            />
          );
        })}

        <Separator />

        <SidebarLink
          label="Alerts"
          to="/alerts"
          icon={<AlertTriangle className="w-4 h-4" />}
        />

        <SidebarLink
          label="Updates"
          to="/updates"
          icon={<Bell className="w-4 h-4" />}
        />

        <Separator />

        <SidebarLink
          label="Settings"
          to="/settings"
          icon={<Settings className="w-4 h-4" />}
        />
      </CardContent>
    </Card>
  );
};

const SidebarLink = ({
  to,
  icon,
  label,
  onClick,
  highlighted,
}: {
  to: string;
  icon: ReactNode;
  label: string;
  onClick?: () => void;
  highlighted?: boolean;
}) => {
  const location = useLocation();
  const hl =
    "/" + location.pathname.split("/")[1] === to && (highlighted ?? true);
  return (
    <Link to={to} className="w-full">
      <Button
        variant="link"
        className={cn(
          "flex justify-start items-center gap-2 w-full hover:bg-accent",
          hl && "bg-accent"
        )}
        onClick={onClick}
      >
        {icon}
        {label}
      </Button>
    </Link>
  );
};
