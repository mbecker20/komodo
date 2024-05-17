import { OpenAlerts } from "@components/alert";
import { Page } from "@components/layouts";
import { ResourceComponents } from "@components/resources";
import { RecentCard } from "@components/resources/common";
import { AllUpdates } from "@components/updates/resource";
import { useRead, useUser } from "@lib/hooks";
import { UsableResource } from "@types";
import { Separator } from "@ui/separator";

export const Dashboard = () => {
  return (
    <Page title="">
      <OpenAlerts />
      <AllUpdates />

      <ResourceRow type="Deployment" />
      <ResourceRow type="Build" />
      <ResourceRow type="Repo" />
      <ResourceRow type="Server" />
      <ResourceRow type="Procedure" />
    </Page>
  );
};

const ResourceRow = ({ type }: { type: UsableResource }) => {
  const recents = useUser().data?.[`recent_${type.toLowerCase()}s`]?.slice(
    0,
    6
  ) as string[] | undefined;
  const resources = useRead(`List${type}s`, {})
    .data?.filter((r) => !recents?.includes(r.id))
    .map((r) => r.id);
  const ids = [
    ...(recents ?? []),
    ...(resources?.slice(0, 6 - (recents?.length || 0)) ?? []),
  ];
  const Components = ResourceComponents[type];
  return (
    <div className="flex gap-4">
      <Components.Dashboard />
      <div className="py-2">
        <Separator orientation="vertical" />
      </div>
      <div className="grid grid-cols-3 grid-rows-2 gap-4 w-full">
        {ids.map((id: string) => (
          <RecentCard key={type + id} type={type} id={id} />
        ))}
      </div>
    </div>
  );
};
