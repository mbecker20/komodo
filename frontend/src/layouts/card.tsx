import { ResourceTarget } from "@monitor/client/dist/types";
import { DeploymentStatusIcon } from "@resources/deployment/util";
import { ServerStatusIcon } from "@resources/server/util";
import {
  Card,
  CardHeader,
  CardTitle,
  CardDescription,
  CardContent,
} from "@ui/card";
import {
  AlarmClock,
  Factory,
  GitBranch,
  Hammer,
  Rocket,
  Server,
} from "lucide-react";
import { ReactNode } from "react";
import { Link } from "react-router-dom";

interface CardProps {
  title: string;
  description: string;
  children: ReactNode;
  statusIcon?: ReactNode;
}

export const ResourceCard = ({
  title,
  description,
  children,
  statusIcon,
}: CardProps) => (
  <Card hoverable>
    <CardHeader className="flex flex-row justify-between">
      <div>
        <CardTitle>{title}</CardTitle>
        <CardDescription>{description}</CardDescription>
      </div>
      <div className="flex gap-2">{statusIcon}</div>
    </CardHeader>
    <CardContent className="flex flex-col gap-6">{children}</CardContent>
  </Card>
);

const ResourceIcons = ({ type }: { type: ResourceTarget["type"] }) => {
  if (type === "Deployment") return <Rocket className="w-4 h-4" />;
  if (type === "Server") return <Server className="w-4 h-4" />;
  if (type === "Build") return <Hammer className="w-4 h-4" />;
  if (type === "Builder") return <Factory className="w-4 h-4" />;
  if (type === "Alerter") return <AlarmClock className="w-4 h-4" />;
  if (type === "Repo") return <GitBranch className="w-4 h-4" />;
  return null;
};

interface ResourceOverviewCardProps {
  type: ResourceTarget["type"];
  children?: ReactNode;
}
export const ResourceOverviewCard = ({
  type,
  children,
}: ResourceOverviewCardProps) => (
  <Link to={`/${type.toLowerCase()}s`} className="w-full">
    <Card hoverable>
      <CardHeader className="flex-row justify-between">
        <CardTitle>{type}s</CardTitle>
        <ResourceIcons type={type} />
      </CardHeader>
      {children && <CardContent className="h-[200px]">{children}</CardContent>}
    </Card>
  </Link>
);
