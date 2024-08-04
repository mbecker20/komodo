import {
  ActionButton,
  ActionWithDialog,
  ConfirmButton,
} from "@components/util";
import { useExecute, useInvalidate, useRead, useWrite } from "@lib/hooks";
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

  if (state === Types.StackState.Unknown) {
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
        name={stack?.name ?? ""}
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

  if (state !== Types.StackState.Running && !destroying) {
    return null;
  }

  const pending = isPending || destroying;
  return (
    <ActionWithDialog
      name={stack?.name ?? ""}
      title="Destroy"
      icon={<Trash2 className="h-4 w-4" />}
      onClick={() => destroy({ stack: id })}
      disabled={pending}
      loading={pending}
    />
  );
};

export const StartStopStack = ({ id }: { id: string }) => {
  const stack = useStack(id);
  const state = stack?.info.state;
  const { mutate: start, isPending: startPending } = useExecute("StartStack");
  const { mutate: stop, isPending: stopPending } = useExecute("StopStack");
  const action_state = useRead(
    "GetStackActionState",
    { stack: id },
    { refetchInterval: 5000 }
  ).data;

  if (stack?.info.file_missing) {
    return null;
  }

  if (state === Types.StackState.Stopped) {
    return (
      <ConfirmButton
        title="Start"
        icon={<Play className="h-4 w-4" />}
        onClick={() => start({ stack: id })}
        disabled={startPending}
        loading={startPending || action_state?.starting}
      />
    );
  }
  if (state === Types.StackState.Running) {
    return (
      <ActionWithDialog
        name={stack?.name ?? ""}
        title="Stop"
        icon={<Square className="h-4 w-4" />}
        onClick={() => stop({ stack: id })}
        disabled={stopPending}
        loading={stopPending || action_state?.stopping}
      />
    );
  }
};

export const PauseUnpauseStack = ({ id }: { id: string }) => {
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

  if (stack?.info.file_missing) {
    return null;
  }

  if (state === Types.StackState.Paused) {
    return (
      <ConfirmButton
        title="Unpause"
        icon={<Play className="h-4 w-4" />}
        onClick={() => unpause({ stack: id })}
        disabled={unpausePending}
        loading={unpausePending || action_state?.unpausing}
      />
    );
  }
  if (state === Types.StackState.Running) {
    return (
      <ActionWithDialog
        name={stack?.name ?? ""}
        title="Pause"
        icon={<Pause className="h-4 w-4" />}
        onClick={() => pause({ stack: id })}
        disabled={pausePending}
        loading={pausePending || action_state?.pausing}
      />
    );
  }
};

export const RefreshStackCache = ({ id }: { id: string }) => {
  const inv = useInvalidate();
  const { mutate, isPending } = useWrite("RefreshStackCache", {
    onSuccess: () => inv(["GetStack", { stack: id }]),
  });
  const pending = isPending;
  return (
    <ActionButton
      title="Refresh"
      icon={<RefreshCcw className="w-4 h-4" />}
      onClick={() => mutate({ stack: id })}
      disabled={pending}
      loading={pending}
    />
  );
};
