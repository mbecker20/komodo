import { useParams } from "react-router-dom";
import { useSetRecentlyViewed } from "@hooks";
import { Resource } from "@layouts/resource";
import { CardDescription } from "@ui/card";
import {
  DeploymentBuild,
  DeploymentName,
  DeploymentServer,
  DeploymentStatus,
  DeploymentStatusIcon,
} from "./util";
import {
  RedeployContainer,
  RemoveContainer,
  StartOrStopContainer,
} from "./components/actions";
import { DeploymentLogs } from "./components/deployment-logs";

export const Deployment = () => {
  const { deploymentId } = useParams();
  const push = useSetRecentlyViewed();

  if (!deploymentId) return null;
  push("Deployment", deploymentId);

  return (
    <Resource
      title={<DeploymentName deploymentId={deploymentId} />}
      info={
        <div className="flex flex-col lg:flex-row lg:items-center lg:gap-4 text-muted-foreground">
          <div className="flex items-center gap-2 ">
            <DeploymentStatusIcon deploymentId={deploymentId} />
            <DeploymentStatus deploymentId={deploymentId} />
          </div>
          <CardDescription className="hidden lg:block">|</CardDescription>
          <DeploymentServer deploymentId={deploymentId} />
          <CardDescription className="hidden lg:block">|</CardDescription>
          <DeploymentBuild deploymentId={deploymentId} />
        </div>
      }
      actions={
        <div className="flex gap-4">
          <RedeployContainer deployment_id={deploymentId} />
          <StartOrStopContainer deployment_id={deploymentId} />
          <RemoveContainer deployment_id={deploymentId} />
        </div>
      }
      tabs={[
        {
          title: "Logs",
          component: <DeploymentLogs deployment_id={deploymentId} />,
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
