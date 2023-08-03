import { ResourceUpdates } from "@components/updates/resource";
import { useRead, useSetRecentlyViewed } from "@hooks";
import { ResourceCard } from "@layouts/card";
import { Resource } from "@layouts/resource";
import {
  RedeployContainer,
  StartOrStopContainer,
  RemoveContainer,
} from "@resources/deployment/components/actions";
import { DeploymentLogs } from "@resources/deployment/components/deployment-logs";
import { DeploymentConfig } from "@resources/deployment/config";
import {
  DeploymentBuild,
  DeploymentName,
  DeploymentServer,
  DeploymentStatus,
  DeploymentStatusIcon,
} from "@resources/deployment/util";
import { CardDescription } from "@ui/card";
import { Link, useParams } from "react-router-dom";

export const DeploymentPage = () => {
  const id = useParams().deploymentId;
  const push = useSetRecentlyViewed();

  if (!id) return null;
  push("Deployment", id);

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
      <DeploymentConfig />
    </Resource>
  );
};

export const DeploymentCard = ({ id }: { id: string }) => {
  const deployments = useRead("ListDeployments", {}).data;
  const deployment = deployments?.find((d) => d.id === id);
  if (!deployment) return null;
  return (
    <Link to={`/deployments/${deployment.id}`}>
      <ResourceCard
        title={deployment.name}
        description={deployment.status ?? "not deployed"}
        statusIcon={<DeploymentStatusIcon deploymentId={id} />}
      >
        <div className="flex flex-col text-muted-foreground text-sm">
          <DeploymentServer deploymentId={id} />
          <DeploymentBuild deploymentId={id} />
        </div>
      </ResourceCard>
    </Link>
  );
};
