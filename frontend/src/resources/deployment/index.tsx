import { ResourceUpdates } from "@components/updates/resource";
import { useAddRecentlyViewed } from "@hooks";
import { Resource } from "@layouts/resource";
import {
  RedeployContainer,
  StartOrStopContainer,
  RemoveContainer,
} from "@resources/deployment/components/actions";
import { DeploymentLogs } from "@resources/deployment/components/deployment-logs";
import {
  DeploymentBuild,
  DeploymentName,
  DeploymentServer,
  DeploymentStatus,
  DeploymentStatusIcon,
} from "@resources/deployment/util";
import { CardDescription } from "@ui/card";
import { useParams } from "react-router-dom";
import { DeploymentConfig } from "./components/config";

export const DeploymentPage = () => {
  const id = useParams().deploymentId;
  if (!id) return null;
  useAddRecentlyViewed("Deployment", id);

  return (
    <Resource
      title={<DeploymentName deploymentId={id} />}
      info={
        <div className="flex flex-col lg:flex-row lg:items-center lg:gap-4 text-muted-foreground">
          <div className="flex items-center gap-2 ">
            <DeploymentStatusIcon deploymentId={id} />
            <DeploymentStatus deploymentId={id} />
          </div>
          <CardDescription className="hidden lg:block">|</CardDescription>
          <DeploymentServer deploymentId={id} />
          <CardDescription className="hidden lg:block">|</CardDescription>
          <DeploymentBuild deploymentId={id} />
        </div>
      }
      actions={
        <div className="flex gap-4">
          <RedeployContainer deployment_id={id} />
          <StartOrStopContainer deployment_id={id} />
          <RemoveContainer deployment_id={id} />
        </div>
      }
    >
      <ResourceUpdates type="Deployment" id={id} />
      <DeploymentLogs deployment_id={id} />
      <DeploymentConfig id={id} />
    </Resource>
  );
};
