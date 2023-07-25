import { ActionButton } from "@components/util";
import { RefreshCw, Play, Trash, Pause } from "lucide-react";
import { useExecute, useRead, useWrite } from "@hooks";
import { DockerContainerState } from "@monitor/client/dist/types";
import { useState } from "react";
import {
  Dialog,
  DialogContent,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "@ui/dialog";
import { Input } from "@ui/input";
import { Button } from "@ui/button";
import { useNavigate } from "react-router-dom";

interface DeploymentId {
  deployment_id: string;
}

export const RedeployContainer = ({ deployment_id }: DeploymentId) => {
  const { mutate, isLoading } = useExecute("Deploy");
  const deployments = useRead("ListDeployments", {}).data;
  const deployment = deployments?.find((d) => d.id === deployment_id);
  return (
    <ActionButton
      title={deployment?.status ? "Redeploy" : "Deploy"}
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
  const deployments = useRead("ListDeployments", {}).data;
  const deployment = deployments?.find((d) => d.id === deployment_id);
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

export const DeleteDeployment = ({ id }: { id: string }) => {
  const nav = useNavigate();
  const deployments = useRead("ListDeployments", {}).data;
  const deployment = deployments?.find((d) => d.id === id);
  const { mutate, isLoading } = useWrite("DeleteDeployment", {
    onSuccess: () => nav("/deployments"),
  });
  const [open, setOpen] = useState(false);
  const [name, setName] = useState("");

  return (
    <>
      <ActionButton
        title="Delete"
        intent="danger"
        icon={<Trash className="h-4 w-4" />}
        onClick={() => setOpen(true)}
        disabled={isLoading}
      />
      <Dialog open={open} onOpenChange={setOpen}>
        <DialogContent>
          <DialogHeader>
            <DialogTitle>Delete Deployment</DialogTitle>
          </DialogHeader>
          <div className="flex flex-col gap-2">
            <p>
              Are you sure you wish to delete this deployment? If so, please
              type in <b>{deployment?.name}</b> below
            </p>
            <Input value={name} onChange={(e) => setName(e.target.value)} />
          </div>
          <DialogFooter>
            <Button
              variant="outline"
              intent="danger"
              disabled={name !== deployment?.name || isLoading}
              onClick={() => mutate({ id })}
            >
              Delete
            </Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>
    </>
  );
};
