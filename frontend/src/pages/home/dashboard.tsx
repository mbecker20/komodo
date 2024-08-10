import { ExportButton } from "@components/export";
import { Page, Section } from "@components/layouts";
import { ResourceComponents } from "@components/resources";
import { ResourceName } from "@components/resources/common";
import { TagsWithBadge } from "@components/tags";
import {
  ColorIntention,
  hex_color_by_intention,
  text_color_class_by_intention,
} from "@lib/color";
import { useNoResources, useRead, useUser } from "@lib/hooks";
import { cn, usableResourcePath } from "@lib/utils";
import { UsableResource } from "@types";
import { AlertTriangle, Boxes, History } from "lucide-react";
import { PieChart } from "react-minimal-pie-chart";
import { Link } from "react-router-dom";

export const Dashboard = () => {
  const noResources = useNoResources();
  const user = useUser().data!;
  return (
    <Page>
      <Section
        title="Dashboard"
        icon={<Boxes className="w-4 h-4" />}
        actions={<ExportButton />}
      >
        <div className="flex flex-col gap-6 w-full">
          {noResources && (
            <div className="flex items-center gap-4 px-2 text-muted-foreground">
              <AlertTriangle className="w-4 h-4" />
              <p className="text-lg">
                No resources found.{" "}
                {user.admin
                  ? "To get started, create a server."
                  : "Contact an admin for access to resources."}
              </p>
            </div>
          )}
          <ResourceRow type="Server" />
          <ResourceRow type="Deployment" />
          <ResourceRow type="Stack" />
          <ResourceRow type="Build" />
          <ResourceRow type="Repo" />
          <ResourceRow type="ResourceSync" />
          <ResourceRow type="Procedure" />
        </div>
      </Section>
    </Page>
  );
};

const ResourceRow = ({ type }: { type: UsableResource }) => {
  const recents = useUser().data?.recents?.[type]?.slice(0, 6);
  const resources = useRead(`List${type}s`, {})
    .data?.filter((r) => !recents?.includes(r.id))
    .map((r) => r.id);
  const ids = [
    ...(recents ?? []),
    ...(resources?.slice(0, 6 - (recents?.length || 0)) ?? []),
  ];
  if (ids.length === 0) return;
  const Components = ResourceComponents[type];
  const name =
    type === "ServerTemplate"
      ? "Server Template"
      : type === "ResourceSync"
      ? "Resource Sync"
      : type;
  return (
    <div className="p-6 border rounded-md flex flex-col lg:flex-row gap-8">
      <Link
        to={`/${usableResourcePath(type)}`}
        className="flex flex-col justify-between pr-8 lg:border-r group"
      >
        <div className="flex items-center gap-4 text-xl group-hover:underline">
          <Components.Icon />
          {name}s
        </div>
        <Components.Dashboard />
      </Link>
      <div className="w-full flex flex-col gap-4">
        <p className="text-md text-muted-foreground flex items-center gap-2">
          <History className="w-4" />
          Recently Viewed
        </p>
        <div className="h-52 grid md:grid-cols-2 lg:grid-cols-3 gap-4">
          {ids.map((id, i) => (
            <RecentCard
              key={type + id}
              type={type}
              id={id}
              className={
                i > 3 ? "hidden lg:flex" : i > 1 ? "hidden md:flex" : undefined
              }
            />
          ))}
        </div>
      </div>
    </div>
  );
};

const RecentCard = ({
  type,
  id,
  className,
}: {
  type: UsableResource;
  id: string;
  className?: string;
}) => {
  const Components = ResourceComponents[type];
  const resource = Components.list_item(id);

  if (!resource) return null;

  const tags = resource?.tags;

  return (
    <Link
      to={`${usableResourcePath(type)}/${id}`}
      className={cn(
        "w-full p-4 border rounded-md hover:bg-accent/25 hover:-translate-y-1 transition-all h-24 flex flex-col justify-between",
        className
      )}
    >
      <div className="flex items-center justify-between">
        <div className="flex items-center gap-2 text-sm">
          <Components.Icon id={id} />
          <ResourceName type={type} id={id} />
        </div>
      </div>
      <div className="flex gap-2 w-full">
        <TagsWithBadge tag_ids={tags} />
      </div>
    </Link>
  );
};

export const DashboardPieChart = ({
  data,
}: {
  data: Array<{ title: string; intention: ColorIntention; value: number }>;
}) => {
  return (
    <div className="flex items-center gap-8">
      <div className="flex flex-col gap-2 w-24">
        {data.map(({ title, value, intention }) => (
          <p className="flex gap-2 text-xs text-muted-foreground">
            <span
              className={cn(
                "font-bold",
                text_color_class_by_intention(intention)
              )}
            >
              {value}
            </span>
            {title}
          </p>
        ))}
      </div>
      <PieChart
        className="w-32 h-32"
        radius={42}
        lineWidth={30}
        data={data.map(({ title, value, intention }) => ({
          title,
          value,
          color: hex_color_by_intention(intention),
        }))}
      />
    </div>
  );
};
