import { ResourceUpdates } from "@components/updates/resource";
import { useRead, useAddRecentlyViewed } from "@hooks";
import { ResourceCard } from "@layouts/card";
import { Resource } from "@layouts/resource";
import { CardDescription } from "@ui/card";
import { Link, useParams } from "react-router-dom";
import { DeploymentActions } from "./actions";
import { DeploymentConfig } from "./config";
import { DeploymentLogs } from "./logs";
import {
  DeploymentStatusIcon,
  DeploymentServer,
  DeploymentBuild,
  DeploymentName,
  DeploymentStatus,
} from "./util";

export const DeploymentCard = ({ id }: { id: string }) => {
  const deployments = useRead("ListDeployments", {}).data;
  const deployment = deployments?.find((d) => d.id === id);
  if (!deployment) return null;
  return (
    <Link to={`/deployments/${deployment.id}`}>
      <ResourceCard
        title={deployment.name}
        description={deployment.info.status ?? "not deployed"}
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
      actions={<DeploymentActions id={id} />}
    >
      <ResourceUpdates type="Deployment" id={id} />
      <DeploymentLogs deployment_id={id} />
      <DeploymentConfig id={id} />
    </Resource>
  );
};
