import { useRead } from "@hooks";
import { CardDescription } from "@ui/card";
import { cn } from "@util/helpers";
import { Circle, HardDrive, Server } from "lucide-react";

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
        deployment?.status === null && "fill-blue-500"
      )}
    />
  );
};

export const DeploymentInfo = ({ deploymentId }: { deploymentId: string }) => {
  const deployments = useRead({ type: "ListDeployments", params: {} }).data;
  const deployment = deployments?.find((d) => d.id === deploymentId);

  return (
    <div className="flex flex-col gap-2 md:flex-row md:gap-4">
      <CardDescription className="flex items-center">
        <HardDrive className="w-4 h-4 mr-2" />
        {/* {data ? deployment?.container?.image ?? "no image" : "..."} */}
        build.name @ build.version {deployment?.image}
      </CardDescription>
      <CardDescription className="flex items-center gap-2">
        <Server className="w-4 h-4" />
        server name
        {/* <ServerName serverId={deployment?.deployment.server_id} /> */}
      </CardDescription>
    </div>
  );
};
