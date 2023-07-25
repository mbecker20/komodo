import { useRead } from "@hooks";
import { Link } from "react-router-dom";
import {
  DeploymentBuild,
  DeploymentServer,
  DeploymentStatusIcon,
} from "./util";
import { ResourceCard } from "@layouts/card";

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
        // icon={<Rocket className="w-4 h-4" />}
      >
        <div className="flex flex-col text-muted-foreground">
          <DeploymentServer deploymentId={id} />
          <DeploymentBuild deploymentId={id} />
        </div>
      </ResourceCard>
    </Link>
    // <ResourceCard
    //   title={deployment.name}
    //   description={deployment.status ?? "not deployed"}
    //   statusIcon={<DeploymentStatusIcon deploymentId={id} />}
    //   icon={<Rocket className="w-4 h-4" />}
    // >
    //   <DeploymentInfo deploymentId={id} />
    // </ResourceCard>
  );
};
