import { useRead } from "@hooks";
import { cn } from "@util/helpers";
import { Circle } from "lucide-react";

export const DeploymentName = ({
  deploymentId,
}: {
  deploymentId: string | undefined;
}) => {
  const deployments = useRead({ type: "ListDeployments", params: {} }).data;
  const deployment = deployments?.find((d) => d.id === deploymentId);
  return <>{deployment?.name ?? "..."}</>;
};

export const DeploymentStatus = ({
  deploymentId,
}: {
  deploymentId: string | undefined;
}) => {
  const deployments = useRead({ type: "ListDeployments", params: {} }).data;
  const deployment = deployments?.find((d) => d.id === deploymentId);
  return <>{deployments ? deployment?.status ?? "not deployed" : "..."}</>;
};

export const DeploymentStatusIcon = ({
  deploymentId,
}: {
  deploymentId: string | undefined;
}) => {
  const deployments = useRead({ type: "ListDeployments", params: {} }).data;
  const deployment = deployments?.find((d) => d.id === deploymentId);
  return (
    <Circle
      className={cn(
        "w-4 h-4 stroke-none",
        deployment?.status === "running" && "fill-green-500",
        deployment?.status === "exited" && "fill-red-500",
        deployment?.status === undefined && "fill-blue-500"
      )}
    />
  );
};
