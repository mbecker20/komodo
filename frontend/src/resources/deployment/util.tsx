import { useRead } from "@hooks";
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
  return (
    <Rocket
      className={cn(
        "w-4 h-4 stroke-primary",
        deployment?.status === "running" && "fill-green-500",
        deployment?.status === "exited" && "fill-red-500",
        deployment?.status === null && "fill-blue-500"
      )}
    />
  );
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
