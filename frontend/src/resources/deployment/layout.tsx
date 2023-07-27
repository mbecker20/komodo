import { Link, useParams } from "react-router-dom";
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
import { Button } from "@ui/button";
import { Settings } from "lucide-react";

export const DeploymentLayout = () => {
  const { deploymentId } = useParams();
  const push = useSetRecentlyViewed();

  if (!deploymentId) return null;
  push("Deployment", deploymentId);

  return (
    <Resource
      title={
        <Link to={`/deployments/${deploymentId}`}>
          <DeploymentName deploymentId={deploymentId} />
        </Link>
      }
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
          <Link to={`/deployments/${deploymentId}/config`}>
            <Button variant="outline">
              <Settings className="w-4 h-4" />
            </Button>
          </Link>
        </div>
      }
    />
  );
};
