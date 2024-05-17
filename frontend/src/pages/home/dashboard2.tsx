import { OpenAlerts } from "@components/alert";
import { Page } from "@components/layouts";
import { ResourceComponents } from "@components/resources";
import { RecentCard } from "@components/resources/common";
import { AllUpdates } from "@components/updates/resource";
import { useUser } from "@lib/hooks";
import { UsableResource } from "@types";
import { Separator } from "@ui/separator";

export const Dashboard = () => {
  const user = useUser().data;
  return (
    <Page title="">
      <OpenAlerts />
      <AllUpdates />

      <div className="flex gap-4">
        <ResourceComponents.Deployment.Dashboard />
        <div className="py-2">
          <Separator orientation="vertical" />
        </div>
        <div className="grid grid-cols-3 gap-4 w-full">
          {user?.recent_deployments?.slice(0, 6).map((id) => (
            <RecentCard type="Deployment" id={id} />
          ))}
        </div>
      </div>
    </Page>
  );
};

// const ResourceRow = ({ type }: { type: UsableResource }) => {
//   const Components = 
// }
