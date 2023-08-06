import { useRead } from "@hooks";
import { DockerContainerState } from "@monitor/client/dist/types";
import { BuildName } from "@resources/build/util";
import { ServerName } from "@resources/server/util";
import { cn } from "@util/helpers";
import { HardDrive, Rocket, Server } from "lucide-react";

export const DeploymentName = ({
  deploymentId,
}: {
  deploymentId: string | undefined;
}) => {
  const deployments = useRead("ListDeployments", {}).data;
  const deployment = deployments?.find((d) => d.id === deploymentId);
  return <>{deployment?.name ?? "..."}</>;
};

export const DeploymentStatus = ({
  deploymentId,
}: {
  deploymentId: string | undefined;
}) => {
  const deployments = useRead("ListDeployments", {}).data;
  const deployment = deployments?.find((d) => d.id === deploymentId);
  return <>{deployments ? deployment?.status ?? "not deployed" : "..."}</>;
};

export const DeploymentStatusIcon = ({
  deploymentId,
}: {
  deploymentId: string | undefined;
}) => {
  const deployments = useRead("ListDeployments", {}).data;
  const deployment = deployments?.find((d) => d.id === deploymentId);
  const s = deployment?.state;

  const color = () => {
    if (s === DockerContainerState.Running) return "fill-green-500";
    if (s === DockerContainerState.Paused) return "fill-orange-500";
    if (s === DockerContainerState.NotDeployed) return "fill-blue-500";
    return "fill-red-500";
  };

  return <Rocket className={cn("w-4 h-4 stroke-primary", color())} />;
};

export const DeploymentServer = ({
  deploymentId,
}: {
  deploymentId: string;
}) => {
  const deployments = useRead("ListDeployments", {}).data;
  const deployment = deployments?.find((d) => d.id === deploymentId);
  return (
    <div className="flex items-center gap-2">
      <Server className="w-4 h-4" />
      {deployment?.server_id ? (
        <ServerName serverId={deployment?.server_id} />
      ) : (
        "n/a"
      )}
    </div>
  );
};

export const DeploymentBuild = ({ deploymentId }: { deploymentId: string }) => {
  const deployments = useRead("ListDeployments", {}).data;
  const deployment = deployments?.find((d) => d.id === deploymentId);
  return (
    <div className="flex items-center">
      <HardDrive className="w-4 h-4 mr-2" />
      {deployment?.build_id ? <BuildName id={deployment?.build_id} /> : "n/a"}
    </div>
  );
};
