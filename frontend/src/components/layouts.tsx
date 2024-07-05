import { Button } from "@ui/button";
import { PlusCircle } from "lucide-react";
import { ReactNode, useState } from "react";
import { Link, Outlet, useNavigate } from "react-router-dom";
import {
  Dialog,
  DialogContent,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from "@ui/dialog";
import { Types } from "@monitor/client";
import { ResourceComponents } from "./resources";
import { Card, CardHeader, CardTitle, CardContent, CardFooter } from "@ui/card";
import { ResourceTags } from "./tags";
import { Topbar } from "./topbar";
import { usableResourcePath } from "@lib/utils";
import { Sidebar } from "./sidebar";
import { ResourceName } from "./resources/common";
import { useShiftKeyListener } from "@lib/hooks";

export const Layout = () => {
  const nav = useNavigate();
  useShiftKeyListener("H", () => nav("/"));
  useShiftKeyListener("G", () => nav("/servers"));
  useShiftKeyListener("D", () => nav("/deployments"));
  useShiftKeyListener("B", () => nav("/builds"));
  useShiftKeyListener("R", () => nav("/repos"));
  useShiftKeyListener("P", () => nav("/procedures"));
  return (
    <>
      <Topbar />
      <div className="flex">
        <Sidebar />
        <div className="w-full h-[calc(100vh-70px)] overflow-y-auto">
          <div className="pb-24">
            <Outlet />
          </div>
        </div>
      </div>
    </>
  );
};

interface PageProps {
  title?: ReactNode;
  icon?: ReactNode;
  titleRight?: ReactNode;
  titleOther?: ReactNode;
  children?: ReactNode;
  subtitle?: ReactNode;
  actions?: ReactNode;
  wrapSize?: "md" | "lg" | "xl" | "2xl";
}

export const Page = ({
  title,
  icon,
  titleRight,
  titleOther,
  subtitle,
  actions,
  children,
}: PageProps) => (
  <div className="flex flex-col gap-10 container py-8 pr-12">
    {(title || icon || subtitle || actions) && (
      <div
        className={`flex flex-col gap-6 lg:flex-row lg:gap-0 lg:justify-between`}
      >
        <div className="flex flex-col gap-4">
          <div className="flex flex-wrap gap-4 items-center">
            {icon}
            <h1 className="text-4xl">{title}</h1>
            {titleRight}
          </div>
          <div className="flex flex-col">{subtitle}</div>
        </div>
        {actions}
      </div>
    )}
    {titleOther}
    {children}
  </div>
);

export const PageXlRow = ({
  title,
  icon,
  titleRight,
  titleOther,
  subtitle,
  actions,
  children,
}: PageProps) => (
  <div className="flex flex-col gap-10 container py-8 pr-12">
    {(title || icon || subtitle || actions) && (
      <div
        className={`flex flex-col gap-6 xl:flex-row xl:gap-0 xl:justify-between`}
      >
        <div className="flex flex-col gap-4">
          <div className="flex flex-wrap gap-4 items-center">
            {icon}
            <h1 className="text-4xl">{title}</h1>
            {titleRight}
          </div>
          <div className="flex flex-col">{subtitle}</div>
        </div>
        {actions}
      </div>
    )}
    {titleOther}
    {children}
  </div>
);

interface SectionProps {
  title?: ReactNode;
  icon?: ReactNode;
  titleOther?: ReactNode;
  children?: ReactNode;
  actions?: ReactNode;
}

export const Section = ({
  title,
  icon,
  titleOther,
  actions,
  children,
}: SectionProps) => (
  <div className="flex flex-col gap-4">
    <div className="flex flex-wrap gap-2 items-start justify-between">
      {title || icon ? (
        <div className="flex items-center gap-2 text-muted-foreground">
          {icon}
          {title && <h2 className="text-xl">{title}</h2>}
        </div>
      ) : (
        titleOther
      )}
      {actions}
    </div>
    {children}
  </div>
);

export const NewLayout = ({
  entityType,
  children,
  enabled,
  onSuccess,
  onOpenChange,
}: {
  entityType: string;
  children: ReactNode;
  enabled: boolean;
  onSuccess: () => Promise<unknown>;
  onOpenChange?: (open: boolean) => void;
}) => {
  const [open, set] = useState(false);
  const [loading, setLoading] = useState(false);
  return (
    <Dialog
      open={open}
      onOpenChange={(open) => {
        set(open);
        onOpenChange && onOpenChange(open);
      }}
    >
      <DialogTrigger asChild>
        <Button variant="secondary" className="items-center gap-2">
          New {entityType} <PlusCircle className="w-4 h-4" />
        </Button>
      </DialogTrigger>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>New {entityType}</DialogTitle>
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

export const ResourceCard = ({
  target: { type, id },
}: {
  target: Exclude<Types.ResourceTarget, { type: "System" }>;
}) => {
  const Components = ResourceComponents[type];

  return (
    <Link
      to={`/${usableResourcePath(type)}/${id}`}
      className="group hover:translate-y-[-2.5%] focus:translate-y-[-2.5%] transition-transform"
    >
      <Card className="h-full hover:bg-accent/50 group-focus:bg-accent/50 transition-colors">
        <CardHeader className="flex-row justify-between">
          <div>
            <CardTitle>
              <ResourceName type={type} id={id} />
            </CardTitle>
            {/* <CardDescription>
              <Components.Description id={id} />
            </CardDescription> */}
          </div>
          <Components.Icon id={id} />
        </CardHeader>
        <CardContent className="text-sm text-muted-foreground">
          {Object.entries(Components.Info).map(([key, Info]) => (
            <Info key={key} id={id} />
          ))}
        </CardContent>
        <CardFooter className="flex items-center gap-2">
          <ResourceTags target={{ type, id }} />
        </CardFooter>
      </Card>
    </Link>
  );
};

export const ResourceRow = ({
  target: { type, id },
}: {
  target: Exclude<Types.ResourceTarget, { type: "System" }>;
}) => {
  const Components = ResourceComponents[type];

  return (
    <Link
      to={`/${usableResourcePath(type)}/${id}`}
      className="group hover:translate-y-[-2.5%] focus:translate-y-[-2.5%] transition-transform"
    >
      <Card className="h-full hover:bg-accent/50 group-focus:bg-accent/50 transition-colors">
        <CardHeader className="grid grid-cols-4 items-center">
          <CardTitle>
            <ResourceName type={type} id={id} />
          </CardTitle>
          {Object.entries(Components.Info).map(([key, Info]) => (
            <Info key={key} id={id} />
          ))}
          <div className="flex items-center gap-2">
            <Components.Icon id={id} />
            {/* <CardDescription>
              <Components.Description id={id} />
            </CardDescription> */}
          </div>
        </CardHeader>
      </Card>
    </Link>
  );
};
