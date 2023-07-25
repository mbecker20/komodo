import { useParams } from "react-router-dom";
import { useSetRecentlyViewed } from "@hooks";
import { Resource } from "@layouts/resource";
import { CardDescription } from "@ui/card";
import {
  DeploymentInfo,
  DeploymentName,
  DeploymentStatus,
  DeploymentStatusIcon,
} from "./util";
import {
  RedeployContainer,
  RemoveContainer,
  StartOrStopContainer,
} from "./components/actions";
import { DeploymentLogs } from "./components/deployment-logs";
import { Rocket } from "lucide-react";

export const Deployment = () => {
  const { deploymentId } = useParams();
  const push = useSetRecentlyViewed();

  if (!deploymentId) return null;
  push("Deployment", deploymentId);

  return (
    <Resource
      title={<DeploymentName deploymentId={deploymentId} />}
      info={
        <div className="flex items-center gap-4">
          <Rocket className="w-4 h-4" />
          <CardDescription className="hidden md:block">|</CardDescription>
          <div className="flex items-center gap-2 text-muted-foreground">
            <DeploymentStatusIcon deploymentId={deploymentId} />
            <DeploymentStatus deploymentId={deploymentId} />
          </div>
          <CardDescription className="hidden md:block">|</CardDescription>
          <DeploymentInfo deploymentId={deploymentId} />
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
