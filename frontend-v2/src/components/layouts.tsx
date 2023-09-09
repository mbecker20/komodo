import { useResourceParamType } from "@lib/hooks";
import { RESOURCE_TARGETS } from "@lib/utils";
import { Button } from "@ui/button";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuGroup,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from "@ui/dropdown-menu";
import { ThemeToggle } from "@ui/theme";
import { ChevronDown, LogOut, PlusCircle } from "lucide-react";
import { ReactNode, useState } from "react";
import { Link, Outlet } from "react-router-dom";
import { Omnibar } from "./omnibar";
import { WsStatusIndicator } from "@lib/socket";
import { UsableResource } from "@types";
import {
  Dialog,
  DialogContent,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from "@ui/dialog";

export const Layout = () => {
  const type = useResourceParamType();
  return (
    <>
      <div className="fixed top-0 border-b bg-background z-50 w-full">
        <div className="container flex items-center justify-between py-4">
          <Link to={"/"} className="text-xl">
            Monitor
          </Link>
          <div className="flex gap-4">
            <Omnibar />
            <DropdownMenu>
              <DropdownMenuTrigger asChild>
                <Button variant="outline" className="w-48 justify-between">
                  {type ? type + "s" : "Dashboard"}
                  <ChevronDown className="w-4 h-4" />
                </Button>
              </DropdownMenuTrigger>
              <DropdownMenuContent className="w-48" side="bottom">
                <DropdownMenuGroup>
                  <Link to="/">
                    <DropdownMenuItem>Dashboard</DropdownMenuItem>
                  </Link>
                  {RESOURCE_TARGETS.map((rt) => (
                    <Link key={rt} to={`/${rt.toLowerCase()}s`}>
                      <DropdownMenuItem>{rt}s</DropdownMenuItem>
                    </Link>
                  ))}
                </DropdownMenuGroup>
              </DropdownMenuContent>
            </DropdownMenu>
            <div className="flex">
              <WsStatusIndicator />
              <ThemeToggle />
              <Button
                variant="ghost"
                size="icon"
                onClick={() => {
                  localStorage.removeItem("monitor-auth-token");
                  window.location.reload();
                }}
              >
                <LogOut className="w-4 h-4" />
              </Button>
            </div>
          </div>
        </div>
      </div>
      <Outlet />
    </>
  );
};

interface PageProps {
  title: ReactNode;
  children: ReactNode;
  subtitle?: ReactNode;
  actions?: ReactNode;
}

export const Page = ({ title, subtitle, actions, children }: PageProps) => (
  <div className="flex flex-col gap-12 container py-32">
    <div className="flex flex-col gap-6 lg:flex-row lg:gap-0 lg:items-start justify-between">
      <div className="flex flex-col">
        <h1 className="text-4xl">{title}</h1>
        {subtitle}
      </div>
      {actions}
    </div>
    {children}
  </div>
);

interface SectionProps {
  title: string;
  children: ReactNode;
  icon?: ReactNode;
  actions?: ReactNode;
}

export const Section = ({ title, icon, actions, children }: SectionProps) => (
  <div className="flex flex-col gap-2">
    <div className="flex items-start justify-between min-h-[40px]">
      <div className="flex items-center gap-2 text-muted-foreground">
        {icon}
        <h2 className="text-xl">{title}</h2>
      </div>
      {actions}
    </div>
    {children}
  </div>
);

export const NewResource = ({
  type,
  children,
  enabled,
  onSuccess,
}: {
  type: UsableResource;
  children: ReactNode;
  enabled: boolean;
  onSuccess: () => Promise<unknown>;
}) => {
  const [open, set] = useState(false);
  const [loading, setLoading] = useState(false);
  return (
    <Dialog open={open} onOpenChange={set}>
      <DialogTrigger asChild>
        <Button className="items-center gap-2">
          New {type} <PlusCircle className="w-4 h-4" />
        </Button>
      </DialogTrigger>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>New {type}</DialogTitle>
        </DialogHeader>
        <div className="flex flex-col gap-4 py-8">{children}</div>
        <DialogFooter>
          <Button
            variant="outline"
            onClick={async () => {
              setLoading(true);
              await onSuccess();
              setLoading(false);
              set(false);
            }}
            disabled={!enabled || loading}
          >
            Create
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
};
