import { RESOURCE_TARGETS, cn, usableResourcePath } from "@lib/utils";
import { Button } from "@ui/button";
import { Card, CardContent } from "@ui/card";
import {
  AlertTriangle,
  Bell,
  Box,
  Boxes,
  FolderTree,
  Key,
  Tag,
  User,
  Variable,
} from "lucide-react";
import { Link, useLocation } from "react-router-dom";
import { ResourceComponents } from "./resources";
import { Separator } from "@ui/separator";
import { ReactNode } from "react";
import { useAtom } from "jotai";
import { homeViewAtom } from "@main";
import { useUser } from "@lib/hooks";

export const Sidebar = () => {
  const user = useUser().data;
  const [view, setView] = useAtom(homeViewAtom);
  return (
    <Card className="h-fit m-4 hidden lg:flex">
      <CardContent className="h-fit grid gap-[2px] px-6 py-2">
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
          const name = type === "ServerTemplate" ? "Template" : type;
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

        <SidebarLink
          label="Variables"
          to="/variables"
          icon={<Variable className="w-4 h-4" />}
        />

        <SidebarLink
          label="Tags"
          to="/tags"
          icon={<Tag className="w-4 h-4" />}
        />

        <Separator />

        {user?.admin && (
          <SidebarLink
            label="Users"
            to="/users"
            icon={<User className="w-4 h-4" />}
          />
        )}

        <SidebarLink
          label="Api Keys"
          to="/keys"
          icon={<Key className="w-4 h-4" />}
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
