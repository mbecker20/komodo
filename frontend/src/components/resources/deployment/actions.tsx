import {
  ActionButton,
  ActionWithDialog,
  ConfirmButton,
} from "@components/util";
import { Play, Trash, Pause, Rocket, Pen, Loader2 } from "lucide-react";
import { useExecute, useInvalidate, useRead, useWrite } from "@lib/hooks";
import { useNavigate } from "react-router-dom";
import { Input } from "@ui/input";
import { useToast } from "@ui/use-toast";
import { useState } from "react";
import { Types } from "@monitor/client";

interface DeploymentId {
  id: string;
}

export const RedeployContainer = ({ id }: DeploymentId) => {
  const { mutate, isPending } = useExecute("Deploy");
  const deployments = useRead("ListDeployments", {}).data;
  const deployment = deployments?.find((d) => d.id === id);
  const deploying = useRead("GetDeploymentActionState", { deployment: id }).data
    ?.deploying;
  const pending = isPending || deploying;
  return (
    <ConfirmButton
      title={deployment?.info.status ? "Redeploy" : "Deploy"}
      icon={
        pending ? (
          <Loader2 className="w-4 h-4 animate-spin" />
        ) : (
          <Rocket className="h-4 w-4" />
        )
      }
      onClick={() => mutate({ deployment: id })}
      disabled={pending}
      loading={pending}
    />
  );
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
    <ActionWithDialog
      name={d.name}
      title="Start"
      //   intent="success"
      icon={<Play className="h-4 w-4" />}
      onClick={() => mutate({ deployment: id })}
      disabled={pending}
      loading={pending}
    />
  );
};

const StopContainer = ({ id }: DeploymentId) => {
  const { data: d } = useRead("GetDeployment", { deployment: id });
  const { mutate, isPending } = useExecute("StopContainer");
  const stopping = useRead("GetDeploymentActionState", {
    deployment: id,
  }).data?.stopping;
  const pending = isPending || stopping;

  if (!d) return null;

  return (
    <ActionWithDialog
      name={d?.name}
      title="Stop"
      //   intent="warning"
      icon={<Pause className="h-4 w-4" />}
      onClick={() => mutate({ deployment: id })}
      disabled={pending}
      loading={pending}
    />
  );
};

export const StartOrStopContainer = ({ id }: DeploymentId) => {
  const deployments = useRead("ListDeployments", {}).data;
  const deployment = deployments?.find((d) => d.id === id);

  if (deployment?.info.state === Types.DockerContainerState.NotDeployed)
    return null;

  if (deployment?.info.state === Types.DockerContainerState.Running)
    return <StopContainer id={id} />;
  return <StartContainer id={id} />;
};

export const RemoveContainer = ({ id }: DeploymentId) => {
  const deployment = useRead("GetDeployment", { deployment: id }).data;
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
      //   intent="warning"
      icon={<Trash className="h-4 w-4" />}
      onClick={() => mutate({ deployment: id })}
      disabled={pending}
      loading={pending}
    />
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
          nav("/");
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
        <ActionButton
          title="Rename"
          icon={<Pen className="w-4 h-4" />}
          disabled={isPending}
          onClick={() => mutate({ id, name })}
        />
      </div>
    </div>
  );
};
