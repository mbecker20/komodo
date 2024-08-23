import { ActionWithDialog, ConfirmButton } from "@components/util";
import { Play, Trash, Pause, Rocket, Pen, RefreshCcw, Square } from "lucide-react";
import { useExecute, useInvalidate, useRead, useWrite } from "@lib/hooks";
import { Input } from "@ui/input";
import { useToast } from "@ui/use-toast";
import { useEffect, useState } from "react";
import { Types } from "@monitor/client";
import {
  Select,
  SelectContent,
  SelectGroup,
  SelectItem,
  SelectTrigger,
} from "@ui/select";
import { useDeployment } from ".";

interface DeploymentId {
  id: string;
}

export const DeployDeployment = ({ id }: DeploymentId) => {
  const deployment = useRead("GetDeployment", { deployment: id }).data;
  const [signal, setSignal] = useState<Types.TerminationSignal>();

  useEffect(
    () => setSignal(deployment?.config?.termination_signal),
    [deployment?.config?.termination_signal]
  );

  const { mutate: deploy, isPending } = useExecute("Deploy");

  const deployments = useRead(
    "ListDeployments",
    {},
    { refetchInterval: 5000 }
  ).data;
  const deployment_item = deployments?.find((d) => d.id === id);

  const deploying = useRead(
    "GetDeploymentActionState",
    { deployment: id },
    { refetchInterval: 5000 }
  ).data?.deploying;

  const pending = isPending || deploying;

  if (!deployment) return null;

  const deployed =
    deployment_item?.info.state !== Types.DeploymentState.NotDeployed &&
    deployment_item?.info.state !== Types.DeploymentState.Unknown;

  if (deployed) {
    return (
      <ActionWithDialog
        name={deployment.name}
        title="Redeploy"
        icon={<Rocket className="h-4 w-4" />}
        onClick={() => deploy({ deployment: id, stop_signal: signal })}
        disabled={pending}
        loading={pending}
        additional={
          deployed && deployment.config!.term_signal_labels.length > 1 ? (
            <TermSignalSelector
              signals={deployment.config!.term_signal_labels}
              signal={signal}
              setSignal={setSignal}
            />
          ) : undefined
        }
      />
    );
  } else {
    return (
      <ConfirmButton
        title="Deploy"
        icon={<Rocket className="h-4 w-4" />}
        onClick={() => deploy({ deployment: id })}
        disabled={pending}
        loading={pending}
      />
    );
  }
};

export const DestroyDeployment = ({ id }: DeploymentId) => {
  const deployment = useRead("GetDeployment", { deployment: id }).data;
  const [signal, setSignal] = useState<Types.TerminationSignal>();

  useEffect(
    () => setSignal(deployment?.config?.termination_signal),
    [deployment?.config?.termination_signal]
  );

  const { mutate, isPending } = useExecute("DestroyDeployment");

  const deployments = useRead("ListDeployments", {}).data;
  const state = deployments?.find((d) => d.id === id)?.info.state;

  const destroying = useRead(
    "GetDeploymentActionState",
    {
      deployment: id,
    },
    { refetchInterval: 5000 }
  ).data?.destroying;

  const pending = isPending || destroying;

  if (!deployment) return null;
  if (state === Types.DeploymentState.NotDeployed) return null;

  return (
    <ActionWithDialog
      name={deployment.name}
      title="Destroy"
      icon={<Trash className="h-4 w-4" />}
      onClick={() => mutate({ deployment: id, signal })}
      disabled={pending}
      loading={pending}
      additional={
        deployment.config!.term_signal_labels.length > 1 ? (
          <TermSignalSelector
            signals={deployment.config!.term_signal_labels}
            signal={signal}
            setSignal={setSignal}
          />
        ) : undefined
      }
    />
  );
};

export const RestartDeployment = ({ id }: DeploymentId) => {
  const deployment = useDeployment(id);
  const state = deployment?.info.state;
  const { mutate: restart, isPending: restartPending } =
    useExecute("RestartDeployment");
  const action_state = useRead(
    "GetDeploymentActionState",
    {
      deployment: id,
    },
    { refetchInterval: 5000 }
  ).data;
  if (!deployment) return null;

  if (state !== Types.DeploymentState.Running) {
    return null;
  }

  return (
    <ActionWithDialog
      name={deployment.name}
      title="Restart"
      icon={<RefreshCcw className="h-4 w-4" />}
      onClick={() => restart({ deployment: id })}
      disabled={restartPending}
      loading={restartPending || action_state?.restarting}
    />
  );
};

export const StartStopDeployment = ({ id }: DeploymentId) => {
  const deployment = useDeployment(id);
  const state = deployment?.info.state;
  const { mutate: start, isPending: startPending } =
    useExecute("StartDeployment");
  const action_state = useRead(
    "GetDeploymentActionState",
    {
      deployment: id,
    },
    { refetchInterval: 5000 }
  ).data;
  if (!deployment) return null;

  if (state === Types.DeploymentState.Exited) {
    return (
      <ConfirmButton
        title="Start"
        icon={<Play className="h-4 w-4" />}
        onClick={() => start({ deployment: id })}
        disabled={startPending}
        loading={startPending || action_state?.starting}
      />
    );
  }
  if (state === Types.DeploymentState.Running) {
    return <StopDeployment id={id} />;
  }
};

const StopDeployment = ({ id }: DeploymentId) => {
  const deployment = useRead("GetDeployment", { deployment: id }).data;
  const [signal, setSignal] = useState<Types.TerminationSignal>();

  useEffect(
    () => setSignal(deployment?.config?.termination_signal),
    [deployment?.config?.termination_signal]
  );

  const { mutate, isPending } = useExecute("StopDeployment");
  const stopping = useRead(
    "GetDeploymentActionState",
    {
      deployment: id,
    },
    { refetchInterval: 5000 }
  ).data?.stopping;
  const pending = isPending || stopping;

  if (!deployment) return null;

  return (
    <ActionWithDialog
      name={deployment.name}
      title="Stop"
      icon={<Square className="h-4 w-4" />}
      onClick={() => mutate({ deployment: id, signal })}
      disabled={pending}
      loading={pending}
      additional={
        deployment.config!.term_signal_labels.length > 1 ? (
          <TermSignalSelector
            signals={deployment.config!.term_signal_labels}
            signal={signal}
            setSignal={setSignal}
          />
        ) : undefined
      }
    />
  );
};

const TermSignalSelector = ({
  signals,
  signal,
  setSignal,
}: {
  signals: Types.TerminationSignalLabel[];
  signal: Types.TerminationSignal | undefined;
  setSignal: (signal: Types.TerminationSignal) => void;
}) => {
  const label = signals.find((s) => s.signal === signal)?.label;
  return (
    <div className="flex flex-col gap-2">
      <div className="text-muted-foreground flex justify-end">Termination</div>
      <div className="text-muted-foreground flex gap-4 items-center justify-end">
        {label}
        <Select
          value={signal}
          onValueChange={(value) => setSignal(value as Types.TerminationSignal)}
        >
          <SelectTrigger className="w-[200px]">{signal}</SelectTrigger>
          <SelectContent>
            <SelectGroup>
              {signals.map(({ signal }) => (
                <SelectItem
                  key={signal}
                  value={signal}
                  className="cursor-pointer"
                >
                  {signal}
                </SelectItem>
              ))}
            </SelectGroup>
          </SelectContent>
        </Select>
      </div>
    </div>
  );
};

export const PauseUnpauseDeployment = ({ id }: DeploymentId) => {
  const deployment = useDeployment(id);
  const state = deployment?.info.state;
  const { mutate: unpause, isPending: unpausePending } =
    useExecute("UnpauseDeployment");
  const { mutate: pause, isPending: pausePending } =
    useExecute("PauseDeployment");
  const action_state = useRead(
    "GetDeploymentActionState",
    {
      deployment: id,
    },
    { refetchInterval: 5000 }
  ).data;
  if (!deployment) return null;

  if (state === Types.DeploymentState.Paused) {
    return (
      <ConfirmButton
        title="Unpause"
        icon={<Play className="h-4 w-4" />}
        onClick={() => unpause({ deployment: id })}
        disabled={unpausePending}
        loading={unpausePending || action_state?.unpausing}
      />
    );
  }
  if (state === Types.DeploymentState.Running) {
    return (
      <ActionWithDialog
        name={deployment.name}
        title="Pause"
        icon={<Pause className="h-4 w-4" />}
        onClick={() => pause({ deployment: id })}
        disabled={pausePending}
        loading={pausePending || action_state?.pausing}
      />
    );
  }
};

export const RenameDeployment = ({ id }: { id: string }) => {
  const invalidate = useInvalidate();
  const [name, set] = useState("");
  const { toast } = useToast();
  const { mutate, isPending } = useWrite("RenameDeployment", {
    onSuccess: () => {
      invalidate(["ListDeployments"]);
      toast({ title: "Deployment renamed" });
      set("");
    },
  });
  return (
    <div className="flex items-center justify-between">
      <div className="w-full">Rename Deployment</div>
      <div className="flex gap-4 w-full justify-end">
        <Input
          value={name}
          onChange={(e) => set(e.target.value)}
          className="w-96"
          placeholder="Enter new name"
        />
        <ConfirmButton
          title="Rename"
          icon={<Pen className="w-4 h-4" />}
          loading={isPending}
          disabled={!name || isPending}
          onClick={() => mutate({ id, name })}
        />
      </div>
    </div>
  );
};
