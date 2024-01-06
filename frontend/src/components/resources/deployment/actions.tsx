import {
  ActionButton,
  ActionWithDialog,
  ConfirmButton,
} from "@components/util";
import { Play, Trash, Pause, Rocket, Pen } from "lucide-react";
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
  const { mutate, isLoading } = useExecute("Deploy");
  const deployments = useRead("ListDeployments", {}).data;
  const deployment = deployments?.find((d) => d.id === id);
  const deploying = useRead("GetDeploymentActionState", { id }).data?.deploying;

  return (
    <ConfirmButton
      title={deployment?.info.status ? "Redeploy" : "Deploy"}
      //   intent="success"
      icon={<Rocket className="h-4 w-4" />}
      onClick={() => mutate({ deployment_id: id })}
      disabled={isLoading}
      loading={deploying}
    />
  );
};

const StartContainer = ({ id }: DeploymentId) => {
  const { data: d } = useRead("GetDeployment", { id });
  const { mutate, isLoading } = useExecute("StartContainer");
  const starting = useRead("GetDeploymentActionState", {
    id,
  }).data?.starting;

  if (!d) return null;
  return (
    <ActionWithDialog
      name={d.name}
      title="Start"
      //   intent="success"
      icon={<Play className="h-4 w-4" />}
      onClick={() => mutate({ deployment_id: id })}
      disabled={isLoading}
      loading={starting}
    />
  );
};

const StopContainer = ({ id }: DeploymentId) => {
  const { data: d } = useRead("GetDeployment", { id });
  const { mutate, isLoading } = useExecute("StopContainer");
  const stopping = useRead("GetDeploymentActionState", {
    id,
  }).data?.stopping;

  if (!d) return null;
  return (
    <ActionWithDialog
      name={d?.name}
      title="Stop"
      //   intent="warning"
      icon={<Pause className="h-4 w-4" />}
      onClick={() => mutate({ deployment_id: id })}
      disabled={isLoading}
      loading={stopping}
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
  const deployment = useRead("GetDeployment", { id }).data;
  const { mutate, isLoading } = useExecute("RemoveContainer");

  const deployments = useRead("ListDeployments", {}).data;
  const state = deployments?.find((d) => d.id === id)?.info.state;

  const removing = useRead("GetDeploymentActionState", {
    id,
  }).data?.removing;

  if (!deployment) return null;
  if (state === Types.DockerContainerState.NotDeployed) return null;

  return (
    <ActionWithDialog
      name={deployment.name}
      title="Remove"
      //   intent="warning"
      icon={<Trash className="h-4 w-4" />}
      onClick={() => mutate({ deployment_id: id })}
      disabled={isLoading}
      loading={removing}
    />
  );
};

export const DeleteDeployment = ({ id }: { id: string }) => {
  const nav = useNavigate();
  const { data: d } = useRead("GetDeployment", { id });
  const { mutateAsync, isLoading } = useWrite("DeleteDeployment");

  const deleting = useRead("GetDeploymentActionState", { id }).data?.deleting;

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
        disabled={isLoading}
        loading={deleting}
      />
    </div>
  );
};

export const RenameDeployment = ({ id }: { id: string }) => {
  const invalidate = useInvalidate();

  const { toast } = useToast();
  const { mutate, isLoading } = useWrite("RenameDeployment", {
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
          disabled={isLoading}
          onClick={() => mutate({ id, name })}
        />
      </div>
    </div>
  );
};
