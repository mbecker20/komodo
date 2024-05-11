import { RESOURCE_TARGETS, cn, usableResourcePath } from "@lib/utils";
import { Button } from "@ui/button";
import { Card, CardContent } from "@ui/card";
import { AlertTriangle, Bell, Box, Home, Tag, UserCircle2 } from "lucide-react";
import { Link, useLocation } from "react-router-dom";
import { ResourceComponents } from "./resources";
import { Separator } from "@ui/separator";
import { ReactNode } from "react";

export const Sidebar = () => {
  return (
    <Card className="h-fit m-4 hidden lg:flex">
      <CardContent className="h-fit grid gap-2 px-6 py-4">
        <SidebarLink label="Home" to="/" icon={<Home className="w-4 h-4" />} />

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
          label="Tags"
          to="/tags"
          icon={<Tag className="w-4 h-4" />}
        />

        <Separator />

        <SidebarLink
          label="Api Keys"
          to="/keys"
          icon={<Box className="w-4 h-4" />}
        />

        <SidebarLink
          label="Users"
          to="/users"
          icon={<UserCircle2 className="w-4 h-4" />}
        />
      </CardContent>
    </Card>
  );
};

const SidebarLink = ({
  to,
  icon,
  label,
}: {
  to: string;
  icon: ReactNode;
  label: string;
}) => {
  const location = useLocation();
  return (
    <Link to={to} className="w-full">
      <Button
        variant="link"
        className={cn(
          "flex justify-start items-center gap-2 w-full hover:bg-accent",
          "/" + location.pathname.split("/")[1] === to && "bg-accent"
        )}
      >
        {icon}
        {label}
      </Button>
    </Link>
  );
};
