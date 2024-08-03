import { ActionWithDialog, ConfirmButton } from "@components/util";
import { useExecute, useRead } from "@lib/hooks";
import { Rocket, Trash2 } from "lucide-react";
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
  const pending = isPending || deploying;
  const deployed =
    state !== undefined &&
    [Types.StackState.Healthy, Types.StackState.Unhealthy].includes(state);

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

  if (state !== Types.StackState.Healthy && !destroying) {
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
