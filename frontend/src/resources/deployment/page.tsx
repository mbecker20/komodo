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
          <div className="flex items-center gap-2 text-muted-foreground">
            <DeploymentStatusIcon deploymentId={deploymentId} />
            <DeploymentStatus deploymentId={deploymentId} />
          </div>
          <CardDescription className="hidden md:block">|</CardDescription>
          <DeploymentInfo deploymentId={deploymentId} />
        </div>
      }
      actions={
        <>
          <RedeployContainer deployment_id={deploymentId} />
          <StartOrStopContainer deployment_id={deploymentId} />
          <RemoveContainer deployment_id={deploymentId} />
        </>
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
