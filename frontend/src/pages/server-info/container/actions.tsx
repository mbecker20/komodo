import { ActionWithDialog, ConfirmButton } from "@components/util";
import { useExecute, useRead } from "@lib/hooks";
import { Types } from "komodo_client";
import { Pause, Play, RefreshCcw, Square, Trash } from "lucide-react";
import { useNavigate } from "react-router-dom";

const useContainer = (id: string, container_name: string) => {
  return useRead("ListDockerContainers", { server: id }).data?.find(
    (container) => container.name === container_name
  );
};

const DestroyContainer = ({
  id,
  container: container_name,
}: {
  id: string;
  container: string;
}) => {
  const container = useContainer(id, container_name);
  const nav = useNavigate();
  const { mutate: destroy, isPending } = useExecute("DestroyContainer", {
    onSuccess: () => nav("/servers/" + id),
  });
  const destroying = useRead(
    "GetServerActionState",
    { server: id },
    { refetchInterval: 5000 }
  ).data?.pruning_containers;

  if (!container) {
    return null;
  }

  return (
    <ActionWithDialog
      name={container_name}
      title="Destroy"
      icon={<Trash className="h-4 w-4" />}
      onClick={() => destroy({ server: id, container: container_name })}
      disabled={isPending}
      loading={isPending || destroying}
    />
  );
};

const RestartContainer = ({
  id,
  container: container_name,
}: {
  id: string;
  container: string;
}) => {
  const container = useContainer(id, container_name);
  const state = container?.state;
  const { mutate: restart, isPending: restartPending } =
    useExecute("RestartContainer");
  const action_state = useRead(
    "GetServerActionState",
    { server: id },
    { refetchInterval: 5000 }
  ).data;

  if (!container || state !== Types.ContainerStateStatusEnum.Running) {
    return null;
  }

  return (
    <ActionWithDialog
      name={container_name}
      title="Restart"
      icon={<RefreshCcw className="h-4 w-4" />}
      onClick={() => restart({ server: id, container: container_name })}
      disabled={restartPending}
      loading={restartPending || action_state?.restarting_containers}
    />
  );
};

const StartStopContainer = ({
  id,
  container: container_name,
}: {
  id: string;
  container: string;
}) => {
  const container = useContainer(id, container_name);
  const state = container?.state;
  const { mutate: start, isPending: startPending } =
    useExecute("StartContainer");
  const { mutate: stop, isPending: stopPending } = useExecute("StopContainer");
  const action_state = useRead(
    "GetServerActionState",
    { server: id },
    { refetchInterval: 5000 }
  ).data;

  if (!container) {
    return null;
  }

  if (state === Types.ContainerStateStatusEnum.Exited) {
    return (
      <ConfirmButton
        title="Start"
        icon={<Play className="h-4 w-4" />}
        onClick={() => start({ server: id, container: container_name })}
        disabled={startPending}
        loading={startPending || action_state?.starting_containers}
      />
    );
  }
  if (state === Types.ContainerStateStatusEnum.Running) {
    return (
      <ActionWithDialog
        name={container_name}
        title="Stop"
        icon={<Square className="h-4 w-4" />}
        onClick={() => stop({ server: id, container: container_name })}
        disabled={stopPending}
        loading={stopPending || action_state?.stopping_containers}
      />
    );
  }
};

const PauseUnpauseContainer = ({
  id,
  container: container_name,
}: {
  id: string;
  container: string;
}) => {
  const container = useContainer(id, container_name);
  const state = container?.state;
  const { mutate: unpause, isPending: unpausePending } =
    useExecute("UnpauseContainer");
  const { mutate: pause, isPending: pausePending } =
    useExecute("PauseContainer");
  const action_state = useRead(
    "GetServerActionState",
    { server: id },
    { refetchInterval: 5000 }
  ).data;

  if (!container) {
    return null;
  }

  if (state === Types.ContainerStateStatusEnum.Paused) {
    return (
      <ConfirmButton
        title="Unpause"
        icon={<Play className="h-4 w-4" />}
        onClick={() => unpause({ server: id, container: container_name })}
        disabled={unpausePending}
        loading={unpausePending || action_state?.unpausing_containers}
      />
    );
  }
  if (state === Types.ContainerStateStatusEnum.Running) {
    return (
      <ActionWithDialog
        name={container_name}
        title="Pause"
        icon={<Pause className="h-4 w-4" />}
        onClick={() => pause({ server: id, container: container_name })}
        disabled={pausePending}
        loading={pausePending || action_state?.pausing_containers}
      />
    );
  }
};

type IdContainerComponent = React.FC<{ id: string; container: string }>;

export const Actions: { [action: string]: IdContainerComponent } = {
  RestartContainer,
  PauseUnpauseContainer,
  StartStopContainer,
  DestroyContainer,
};
