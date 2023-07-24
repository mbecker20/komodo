import { ActionButton } from "@components/util";
import { RefreshCw, Play, Trash, Pause } from "lucide-react";
import { useExecute, useRead } from "@hooks";
import { DockerContainerState } from "@monitor/client/dist/types";

export const RedeployContainer = ({
  deploymentId,
}: {
  deploymentId: string;
}) => {
  const { mutate, isLoading } = useExecute();
  return (
    <ActionButton
      title="Redeploy"
      intent="success"
      icon={<RefreshCw className="h-4 w-4" />}
      onClick={() =>
        mutate({ type: "Deploy", params: { deployment_id: deploymentId } })
      }
      disabled={isLoading}
    />
  );
};

const StartContainer = ({ deploymentId }: { deploymentId: string }) => {
  const { mutate, isLoading } = useExecute();

  return (
    <ActionButton
      title="Start"
      intent="success"
      icon={<Play className="h-4 w-4" />}
      onClick={() =>
        mutate({
          type: "StartContainer",
          params: { deployment_id: deploymentId },
        })
      }
      disabled={isLoading}
    />
  );
};

const StopContainer = ({ deploymentId }: { deploymentId: string }) => {
  const { mutate, isLoading } = useExecute();

  return (
    <ActionButton
      title="Stop"
      intent="warning"
      icon={<Pause className="h-4 w-4" />}
      onClick={() =>
        mutate({
          type: "StopContainer",
          params: { deployment_id: deploymentId },
        })
      }
      disabled={isLoading}
    />
  );
};

export const StartOrStopContainer = ({
  deploymentId,
}: {
  deploymentId: string;
}) => {
  const { data } = useRead({ type: "ListDeployments", params: {} });
  const deployment = data?.find((d) => d.id == deploymentId);
  if (deployment?.state === DockerContainerState.Running)
    return <StopContainer deploymentId={deploymentId} />;
  return <StartContainer deploymentId={deploymentId} />;
};

export const RemoveContainer = ({ deploymentId }: { deploymentId: string }) => {
  const { mutate, isLoading } = useExecute();

  return (
    <ActionButton
      title="Remove"
      intent="warning"
      icon={<Trash className="h-4 w-4" />}
      onClick={() =>
        mutate({
          type: "RemoveContainer",
          params: { deployment_id: deploymentId },
        })
      }
      disabled={isLoading}
    />
  );
};
