import { useParams } from "react-router-dom";
import { useSetRecentlyViewed } from "@hooks";
import { Resource } from "@layouts/resource";
import { CardDescription } from "@ui/card";
import { DeploymentName, DeploymentStatus, DeploymentStatusIcon } from "./util";
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
          <RedeployContainer deploymentId={deploymentId} />
          <StartOrStopContainer deploymentId={deploymentId} />
          <RemoveContainer deploymentId={deploymentId} />
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
