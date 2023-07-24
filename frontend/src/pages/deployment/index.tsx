// import { DeploymentLogs } from "@pages/resource/deployment/components/logs";
import { Resource } from "@layouts/resource";
// import { Updates } from "@components/updates";
import { useParams } from "react-router-dom";
import { CardDescription } from "@ui/card";
// import { DeploymentConfig } from "@pages/resource/deployment/components/config";
// import {
//   RedeployContainer,
//   StartOrStopContainer,
//   RemoveContainer,
// } from "@pages/resource/deployment/components/actions";
// import { DeleteDeployment } from "@pages/resource/deployment/components/delete";
import { useRead, useSetRecentlyViewed } from "@hooks";
import { Circle } from "lucide-react";
import { cn } from "@util/helpers";
import { DeploymentLogs } from "./components/deployment-logs";
import { Updates } from "@components/updates/updates";

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
        deployment?.status === "not_deployed" && "fill-blue-500"
      )}
    />
  );
};

// export const DeploymentInfo = ({ deploymentId }: { deploymentId: string }) => {
//   const deployments = useRead({ type: "ListDeployments", params: {} }).data;
//   const deployment = deployments?.find((d) => d.id === deploymentId);

//   return (
//     <div className="flex flex-col gap-2 md:flex-row md:gap-4">
//       <Link to={`/builds/${deployment?.deployment.build_id}`}>
//         <CardDescription className="flex items-center">
//           <HardDrive className="w-4 h-4 mr-2" />
//           {data ? deployment?.container?.image ?? "no image" : "..."}
//         </CardDescription>
//       </Link>
//       <Link to={`/servers/${deployment?.deployment.server_id}`}>
//         <CardDescription className="flex items-center gap-2">
//           <Server className="w-4 h-4" />
//           <ServerName serverId={deployment?.deployment.server_id} />
//         </CardDescription>
//       </Link>
//     </div>
//   );
// };

export const Deployment = () => {
  const { deploymentId } = useParams();
  const push = useSetRecentlyViewed();

  if (!deploymentId) return null;
  push("Deployment", deploymentId);

  return (
    <Resource
      title={<DeploymentName deploymentId={deploymentId} />}
      info={
        <div className="flex flex-col gap-2 md:flex-row md:items-center md:gap-4">
          <div>deployment info</div>
          {/* <DeploymentInfo deploymentId={deploymentId} /> */}
          <CardDescription className="hidden md:block">|</CardDescription>
          <CardDescription className="flex items-center gap-2">
            <DeploymentStatusIcon deploymentId={deploymentId} />
            <DeploymentStatus deploymentId={deploymentId} />
          </CardDescription>
          <CardDescription className="hidden md:block">|</CardDescription>
          {/* <DeleteDeployment deploymentId={deploymentId} /> */}
        </div>
      }
      actions={
        <>
          {/* <RedeployContainer deploymentId={deploymentId} />
          <StartOrStopContainer deploymentId={deploymentId} />
          <RemoveContainer deploymentId={deploymentId} /> */}
        </>
      }
      tabs={[
        {
          title: "Logs",
          component: <DeploymentLogs deploymentId={deploymentId} />,
        },
        {
          title: "Config",
          component: <>Config</>,
        },
        {
          title: "Updates",
          component: <>Updates</>,
        },
      ]}
    />
  );
};
