import { ActionWithDialog, ConfirmButton } from "@components/util";
import { useExecute, useRead } from "@lib/hooks";
import { Pause, Play, RefreshCcw, Rocket, Square, Trash2 } from "lucide-react";
import { useStack } from ".";
import { Types } from "@monitor/client";

export const DeployStack = ({ id }: { id: string }) => {
  const stack = useStack(id);
  const state = stack?.info.state;
  const { mutate: deploy, isPending } = useExecute("DeployStack");
  const deploying = useRead(
    "GetStackActionState",
    { stack: id },
    { refetchInterval: 5000 }
  ).data?.deploying;

  if (!stack || state === Types.StackState.Unknown) {
    return null;
  }

  const pending = isPending || deploying;
  const deployed =
    state !== undefined &&
    [
      Types.StackState.Running,
      Types.StackState.Paused,
      Types.StackState.Stopped,
      Types.StackState.Restarting,
      Types.StackState.Unhealthy,
    ].includes(state);

  if (deployed) {
    return (
      <ActionWithDialog
        name={stack.name}
        title="Redeploy"
        icon={<Rocket className="h-4 w-4" />}
        onClick={() => deploy({ stack: id })}
        disabled={pending}
        loading={pending}
      />
    );
  }

  return (
    <ConfirmButton
      title="Deploy"
      icon={<Rocket className="w-4 h-4" />}
      onClick={() => deploy({ stack: id })}
      disabled={pending}
      loading={pending}
    />
  );
};

export const DestroyStack = ({ id }: { id: string }) => {
  const stack = useStack(id);
  const state = stack?.info.state;
  const { mutate: destroy, isPending } = useExecute("DestroyStack");
  const destroying = useRead(
    "GetStackActionState",
    { stack: id },
    { refetchInterval: 5000 }
  ).data?.destroying;

  if (
    state === undefined ||
    [Types.StackState.Unknown, Types.StackState.Down].includes(state)
  ) {
    return null;
  }

  const pending = isPending || destroying;

  if (!stack) {
    return null;
  }

  return (
    <ActionWithDialog
      name={stack.name}
      title="Destroy"
      icon={<Trash2 className="h-4 w-4" />}
      onClick={() => destroy({ stack: id })}
      disabled={pending}
      loading={pending}
    />
  );
};

export const RestartStack = ({
  id,
  service,
}: {
  id: string;
  service?: string;
}) => {
  const stack = useStack(id);
  const state = stack?.info.state;
  const { mutate: restart, isPending: restartPending } =
    useExecute("RestartStack");
  const action_state = useRead(
    "GetStackActionState",
    { stack: id },
    { refetchInterval: 5000 }
  ).data;
  const services = useRead("ListStackServices", { stack: id }).data;
  const container_state =
    (service &&
      services?.find((s) => s.service === service)?.container?.state) ??
    Types.DeploymentState.Unknown;

  if (
    stack?.info.project_missing ||
    (service && container_state !== Types.DeploymentState.Running) ||
    state !== Types.StackState.Running
  ) {
    return null;
  }

  return (
    <ActionWithDialog
      name={`${stack?.name}${service ? ` - ${service}` : ""}`}
      title="Restart"
      icon={<RefreshCcw className="h-4 w-4" />}
      onClick={() => restart({ stack: id, service })}
      disabled={restartPending}
      loading={restartPending || action_state?.restarting}
    />
  );
};

export const StartStopStack = ({
  id,
  service,
}: {
  id: string;
  service?: string;
}) => {
  const stack = useStack(id);
  const state = stack?.info.state;
  const { mutate: start, isPending: startPending } = useExecute("StartStack");
  const { mutate: stop, isPending: stopPending } = useExecute("StopStack");
  const action_state = useRead(
    "GetStackActionState",
    { stack: id },
    { refetchInterval: 5000 }
  ).data;
  const services = useRead("ListStackServices", { stack: id }).data;
  const container_state =
    (service &&
      services?.find((s) => s.service === service)?.container?.state) ??
    Types.DeploymentState.Unknown;

  if (stack?.info.project_missing) {
    return null;
  }

  if (
    (service && container_state === Types.DeploymentState.Exited) ||
    state === Types.StackState.Stopped
  ) {
    return (
      <ConfirmButton
        title="Start"
        icon={<Play className="h-4 w-4" />}
        onClick={() => start({ stack: id, service })}
        disabled={startPending}
        loading={startPending || action_state?.starting}
      />
    );
  }
  if (
    (service && container_state === Types.DeploymentState.Running) ||
    state === Types.StackState.Running
  ) {
    return (
      <ActionWithDialog
        name={`${stack?.name}${service ? ` - ${service}` : ""}`}
        title="Stop"
        icon={<Square className="h-4 w-4" />}
        onClick={() => stop({ stack: id, service })}
        disabled={stopPending}
        loading={stopPending || action_state?.stopping}
      />
    );
  }
};

export const PauseUnpauseStack = ({
  id,
  service,
}: {
  id: string;
  service?: string;
}) => {
  const stack = useStack(id);
  const state = stack?.info.state;
  const { mutate: unpause, isPending: unpausePending } =
    useExecute("UnpauseStack");
  const { mutate: pause, isPending: pausePending } = useExecute("PauseStack");
  const action_state = useRead(
    "GetStackActionState",
    { stack: id },
    { refetchInterval: 5000 }
  ).data;
  const services = useRead("ListStackServices", { stack: id }).data;
  const container_state =
    (service &&
      services?.find((s) => s.service === service)?.container?.state) ??
    Types.DeploymentState.Unknown;

  if (stack?.info.project_missing) {
    return null;
  }

  if (
    (service && container_state === Types.DeploymentState.Paused) ||
    state === Types.StackState.Paused
  ) {
    return (
      <ConfirmButton
        title="Unpause"
        icon={<Play className="h-4 w-4" />}
        onClick={() => unpause({ stack: id, service })}
        disabled={unpausePending}
        loading={unpausePending || action_state?.unpausing}
      />
    );
  }
  if (
    (service && container_state === Types.DeploymentState.Running) ||
    state === Types.StackState.Running
  ) {
    return (
      <ActionWithDialog
        name={`${stack?.name}${service ? ` - ${service}` : ""}`}
        title="Pause"
        icon={<Pause className="h-4 w-4" />}
        onClick={() => pause({ stack: id, service })}
        disabled={pausePending}
        loading={pausePending || action_state?.pausing}
      />
    );
  }
};
