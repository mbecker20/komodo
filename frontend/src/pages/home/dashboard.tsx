import { Page, Section } from "@components/layouts";
import { ResourceComponents } from "@components/resources";
import { ResourceName } from "@components/resources/common";
import { TagsWithBadge } from "@components/tags";
import { useRead, useUser } from "@lib/hooks";
import { cn, usableResourcePath } from "@lib/utils";
import { UsableResource } from "@types";
import { Card } from "@ui/card";
import { Separator } from "@ui/separator";
import { Boxes, History } from "lucide-react";
import { Link } from "react-router-dom";

export const Dashboard = () => {
  return (
    <Page>
      {/* <OpenAlerts /> */}
      {/* <AllUpdates /> */}

      <Section title="Resources" icon={<Boxes className="w-4 h-4" />}>
        <div className="flex flex-col gap-6 w-full">
          <ResourceRow type="Server" />
          <ResourceRow type="Deployment" />
          <ResourceRow type="Build" />
          <ResourceRow type="Procedure" />
          {/* <ResourceRow type="Repo" /> */}
          {/* <ResourceRow type="Server" /> */}
          {/* <ResourceRow type="Procedure" /> */}
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

  return (
    <div className="p-6 border rounded-md flex gap-8">
      <Link
        to={`/${type.toLowerCase()}s`}
        className="flex flex-col justify-between pr-8 border-r group"
      >
        <div className="flex items-center gap-4 text-xl group-hover:underline">
          <Components.Icon />
          {type}s
        </div>
        <Components.Dashboard />
      </Link>
      <div className="w-full flex flex-col gap-4">
        <p className="text-md text-muted-foreground flex items-center gap-2">
          <History className="w-4" />
          Recently Viewed
        </p>
        <div className=" grid grid-cols-3 gap-4">
          {ids.map((id, i) => (
            <RecentCard
              key={type + id}
              type={type}
              id={id}
              className={
                i > 3 ? "hidden 2xl:block" : i > 1 ? "hidden xl:block" : false
              }
            />
          ))}
        </div>
      </div>
    </div>
  );

  return (
    <div className="flex gap-4">
      <Components.Dashboard />
      <div className="hidden md:flex gap-4 w-full">
        <div className="py-2">
          <Separator orientation="vertical" />
        </div>
        <div className="flex flex-col gap-4 w-full pb-1">
          <div className="flex gap-2 items-center text-muted-foreground">
            <History className="w-4 h-4" />
            <h3>Recent {type}s</h3>
          </div>
          <div className="grid grid-rows-2 grid-cols-1 xl:grid-cols-2 2xl:grid-cols-3 gap-4 w-full h-full">
            {ids.map((id, i) => (
              <RecentCard
                key={type + id}
                type={type}
                id={id}
                className={
                  i > 3 ? "hidden 2xl:block" : i > 1 ? "hidden xl:block" : false
                }
              />
            ))}
          </div>
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
  className: string | false;
}) => {
  const Components = ResourceComponents[type];
  const resource = Components.list_item(id);

  if (!resource) return null;

  const tags = resource?.tags;

  return (
    <Link
      to={`${usableResourcePath(type)}/${id}`}
      className="w-full p-4 border rounded-md hover:bg-accent/25 hover:-translate-y-1 transition-all"
    >
      <div className="flex items-center justify-between">
        <div className="flex items-center gap-2 text-sm">
          <Components.Icon id={id} />
          <ResourceName type={type} id={id} />
        </div>
        {/* <Components.Status.State id={id} /> */}
      </div>

      <div className="flex gap-2 w-full mt-4">
        <TagsWithBadge tag_ids={tags} />
      </div>
    </Link>
  );

  return (
    <Link
      to={`${usableResourcePath(type)}/${id}`}
      className={cn("h-full", className)}
    >
      <Card className="h-full px-6 py-4 flex flex-col justify-between hover:bg-accent/50 transition-colors cursor-pointer">
        <div className="flex items-center justify-between w-full">
          <div className="flex items-center gap-2">
            <Components.Icon id={id} />
            <ResourceName type={type} id={id} />
          </div>
          <div className="text-sm">
            <Components.Status.State id={id} />
          </div>
        </div>
        <div className="flex items-end justify-end gap-2 w-full">
          <TagsWithBadge tag_ids={tags} />
        </div>
      </Card>
    </Link>
  );
};
