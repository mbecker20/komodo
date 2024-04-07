import { ActionWithDialog, ConfirmButton } from "@components/util";
import { Play, Trash, Pause, Rocket, Pen } from "lucide-react";
import { useExecute, useInvalidate, useRead, useWrite } from "@lib/hooks";
import { useNavigate } from "react-router-dom";
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
import { DockerContainerState } from "@monitor/client/dist/types";

interface DeploymentId {
  id: string;
}

export const RedeployContainer = ({ id }: DeploymentId) => {
  const deployment = useRead("GetDeployment", { deployment: id }).data;
  const [signal, setSignal] = useState<Types.TerminationSignal>();

  useEffect(
    () => setSignal(deployment?.config.termination_signal),
    [deployment?.config.termination_signal]
  );

  const { mutate: deploy, isPending } = useExecute("Deploy");

  const deployments = useRead("ListDeployments", {}).data;
  const deployment_item = deployments?.find((d) => d.id === id);

  const deploying = useRead("GetDeploymentActionState", { deployment: id }).data
    ?.deploying;

  const pending = isPending || deploying;

  if (!deployment) return null;

  const deployed =
    deployment_item?.info.state !== DockerContainerState.NotDeployed &&
    deployment_item?.info.state !== DockerContainerState.Unknown;

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
          deployed && deployment.config.term_signal_labels.length > 1 ? (
            <TermSignalSelector
              signals={deployment.config.term_signal_labels}
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

const StartContainer = ({ id }: DeploymentId) => {
  const { data: d } = useRead("GetDeployment", { deployment: id });
  const { mutate, isPending } = useExecute("StartContainer");
  const starting = useRead("GetDeploymentActionState", {
    deployment: id,
  }).data?.starting;
  const pending = isPending || starting;

  if (!d) return null;

  return (
    <ConfirmButton
      title="Start"
      icon={<Play className="h-4 w-4" />}
      onClick={() => mutate({ deployment: id })}
      disabled={pending}
      loading={pending}
    />
  );
};

const StopContainer = ({ id }: DeploymentId) => {
  const deployment = useRead("GetDeployment", { deployment: id }).data;
  const [signal, setSignal] = useState<Types.TerminationSignal>();

  useEffect(
    () => setSignal(deployment?.config.termination_signal),
    [deployment?.config.termination_signal]
  );

  const { mutate, isPending } = useExecute("StopContainer");
  const stopping = useRead("GetDeploymentActionState", {
    deployment: id,
  }).data?.stopping;
  const pending = isPending || stopping;

  if (!deployment) return null;

  return (
    <ActionWithDialog
      name={deployment.name}
      title="Stop"
      icon={<Pause className="h-4 w-4" />}
      onClick={() => mutate({ deployment: id, signal })}
      disabled={pending}
      loading={pending}
      additional={
        deployment.config.term_signal_labels.length > 1 ? (
          <TermSignalSelector
            signals={deployment.config.term_signal_labels}
            signal={signal}
            setSignal={setSignal}
          />
        ) : undefined
      }
    />
  );
};

export const StartOrStopContainer = ({ id }: DeploymentId) => {
  const deployments = useRead("ListDeployments", {}).data;
  const deployment = deployments?.find((d) => d.id === id);
  const state = deployment?.info.state;

  if (
    state === Types.DockerContainerState.NotDeployed ||
    state === Types.DockerContainerState.Unknown
  ) {
    return null;
  }

  if (
    state === Types.DockerContainerState.Running ||
    state === Types.DockerContainerState.Restarting
  ) {
    return <StopContainer id={id} />;
  }

  return <StartContainer id={id} />;
};

export const RemoveContainer = ({ id }: DeploymentId) => {
  const deployment = useRead("GetDeployment", { deployment: id }).data;
  const [signal, setSignal] = useState<Types.TerminationSignal>();

  useEffect(
    () => setSignal(deployment?.config.termination_signal),
    [deployment?.config.termination_signal]
  );

  const { mutate, isPending } = useExecute("RemoveContainer");

  const deployments = useRead("ListDeployments", {}).data;
  const state = deployments?.find((d) => d.id === id)?.info.state;

  const removing = useRead("GetDeploymentActionState", {
    deployment: id,
  }).data?.removing;

  const pending = isPending || removing;

  if (!deployment) return null;
  if (state === Types.DockerContainerState.NotDeployed) return null;

  return (
    <ActionWithDialog
      name={deployment.name}
      title="Remove"
      icon={<Trash className="h-4 w-4" />}
      onClick={() => mutate({ deployment: id, signal })}
      disabled={pending}
      loading={pending}
      additional={
        deployment.config.term_signal_labels.length > 1 ? (
          <TermSignalSelector
            signals={deployment.config.term_signal_labels}
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

export const DeleteDeployment = ({ id }: { id: string }) => {
  const nav = useNavigate();
  const { data: d } = useRead("GetDeployment", { deployment: id });
  const { mutateAsync, isPending } = useWrite("DeleteDeployment");

  const deleting = useRead("GetDeploymentActionState", { deployment: id }).data
    ?.deleting;

  const pending = isPending || deleting;

  if (!d) return null;
  return (
    <div className="flex items-center justify-between">
      <div className="w-full">Delete Deployment</div>
      <ActionWithDialog
        name={d.name}
        title="Delete"
        icon={<Trash className="h-4 w-4" />}
        onClick={async () => {
          await mutateAsync({ id });
          nav("/deployments");
        }}
        disabled={pending}
        loading={pending}
      />
    </div>
  );
};

export const RenameDeployment = ({ id }: { id: string }) => {
  const invalidate = useInvalidate();

  const { toast } = useToast();
  const { mutate, isPending } = useWrite("RenameDeployment", {
    onSuccess: () => {
      invalidate(["ListDeployments"]);
      toast({ title: "Deployment Renamed" });
      set("");
    },
  });

  const [name, set] = useState("");

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
