import { ActionButton } from "@components/util";
import { RefreshCw, Play, Trash, Pause } from "lucide-react";
import { useExecute, useRead } from "@hooks";
import { DockerContainerState } from "@monitor/client/dist/types";

interface DeploymentId {
  deployment_id: string;
}

export const RedeployContainer = ({ deployment_id }: DeploymentId) => {
  const { mutate, isLoading } = useExecute("Deploy");
  return (
    <ActionButton
      title="Redeploy"
      intent="success"
      icon={<RefreshCw className="h-4 w-4" />}
      onClick={() => mutate({ deployment_id })}
      disabled={isLoading}
    />
  );
};

const StartContainer = ({ deployment_id }: DeploymentId) => {
  const { mutate, isLoading } = useExecute("StartContainer");
  return (
    <ActionButton
      title="Start"
      intent="success"
      icon={<Play className="h-4 w-4" />}
      onClick={() => mutate({ deployment_id })}
      disabled={isLoading}
    />
  );
};

const StopContainer = ({ deployment_id }: DeploymentId) => {
  const { mutate, isLoading } = useExecute("StopContainer");

  return (
    <ActionButton
      title="Stop"
      intent="warning"
      icon={<Pause className="h-4 w-4" />}
      onClick={() => mutate({ deployment_id })}
      disabled={isLoading}
    />
  );
};

export const StartOrStopContainer = ({ deployment_id }: DeploymentId) => {
  const { data } = useRead({ type: "ListDeployments", params: {} });
  const deployment = data?.find((d) => d.id == deployment_id);
  if (deployment?.state === DockerContainerState.Running)
    return <StopContainer deployment_id={deployment_id} />;
  return <StartContainer deployment_id={deployment_id} />;
};

export const RemoveContainer = ({ deployment_id }: DeploymentId) => {
  const { mutate, isLoading } = useExecute("RemoveContainer");
  return (
    <ActionButton
      title="Remove"
      intent="warning"
      icon={<Trash className="h-4 w-4" />}
      onClick={() => mutate({ deployment_id })}
      disabled={isLoading}
    />
  );
};
