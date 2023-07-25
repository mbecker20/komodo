import { useRead } from "@hooks";
import { Link } from "react-router-dom";
import { DeploymentInfo, DeploymentStatusIcon } from "./util";
import { Rocket } from "lucide-react";
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
        icon={<Rocket className="w-4 h-4" />}
      >
        <DeploymentInfo deploymentId={id} />
      </ResourceCard>
    </Link>
  );
};
