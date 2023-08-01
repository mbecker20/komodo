import { useParams } from "react-router-dom";
import { useSetRecentlyViewed } from "@hooks";
import { DeploymentLogs } from "./components/deployment-logs";
import { ResourceUpdates } from "@components/updates/resource";
import { DeploymentConfig } from "./config";

export const DeploymentPage = () => {
  const { deploymentId } = useParams();
  const push = useSetRecentlyViewed();

  if (!deploymentId) return null;
  push("Deployment", deploymentId);

  return (
    <div className="flex flex-col gap-12">
      <ResourceUpdates id={deploymentId} />
      <DeploymentLogs deployment_id={deploymentId} />
      <DeploymentConfig />
    </div>
  );
};
