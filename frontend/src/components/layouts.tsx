import { Button } from "@ui/button";
import { PlusCircle } from "lucide-react";
import { ReactNode, useState } from "react";
import { Link, Outlet } from "react-router-dom";
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

export const Layout = () => {
  return (
    <>
      <Topbar />
      <Outlet />
    </>
  );
};

interface PageProps {
  title?: ReactNode;
  titleRight?: ReactNode;
  children?: ReactNode;
  subtitle?: ReactNode;
  actions?: ReactNode;
}

export const Page = ({
  title,
  titleRight,
  subtitle,
  actions,
  children,
}: PageProps) => (
  <div className="flex flex-col gap-12 container py-8">
    {(title || subtitle || actions) && (
      <div className="flex flex-col gap-6 lg:flex-row lg:gap-0 lg:items-start justify-between">
        <div className="flex flex-col gap-2">
          <div className="flex gap-4 items-center">
            <h1 className="text-4xl">{title}</h1>
            {titleRight}
          </div>
          <div className="flex flex-col">{subtitle}</div>
        </div>
        {actions}
      </div>
    )}
    {children}
  </div>
);

interface SectionProps {
  title?: ReactNode;
  children?: ReactNode;
  icon?: ReactNode;
  actions?: ReactNode;
}

export const Section = ({ title, icon, actions, children }: SectionProps) => (
  <div className="flex flex-col gap-4">
    <div className="flex items-start justify-between">
      {(title || icon) && (
        <div className="flex items-center gap-2 text-muted-foreground">
          {icon}
          <h2 className="text-xl">{title}</h2>
        </div>
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
}: {
  entityType: string;
  children: ReactNode;
  enabled: boolean;
  onSuccess: () => Promise<unknown>;
}) => {
  const [open, set] = useState(false);
  const [loading, setLoading] = useState(false);
  return (
    <Dialog open={open} onOpenChange={set}>
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
      to={`/${type.toLowerCase()}s/${id}`}
      className="group hover:translate-y-[-2.5%] focus:translate-y-[-2.5%] transition-transform"
    >
      <Card className="h-full hover:bg-accent/50 group-focus:bg-accent/50 transition-colors">
        <CardHeader className="flex-row justify-between">
          <div>
            <CardTitle>
              <Components.Name id={id} />
            </CardTitle>
            {/* <CardDescription>
              <Components.Description id={id} />
            </CardDescription> */}
          </div>
          <Components.Icon id={id} />
        </CardHeader>
        <CardContent className="text-sm text-muted-foreground">
          {Components.Info.map((Info) => (
            <Info id={id} />
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
      to={`/${type.toLowerCase()}s/${id}`}
      className="group hover:translate-y-[-2.5%] focus:translate-y-[-2.5%] transition-transform"
    >
      <Card className="h-full hover:bg-accent/50 group-focus:bg-accent/50 transition-colors">
        <CardHeader className="grid grid-cols-4 items-center">
          <CardTitle>
            <Components.Name id={id} />
          </CardTitle>
          {Components.Info.map((Info) => (
            <Info id={id} />
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
