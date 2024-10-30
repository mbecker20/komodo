import { useExecute, useRead, useSelectedResources } from "@lib/hooks";
import { DialogTitle } from "@radix-ui/react-dialog";
import { Button } from "@ui/button";
import {
  Dialog,
  DialogContent,
  DialogFooter,
  DialogHeader,
  //   DialogTrigger,
} from "@ui/dialog";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@ui/select";
// import { Types } from "komodo_client";
import { useState } from "react";

type DeploymentActions = (typeof DEPLOYMENT_ACTIONS)[number];
const DEPLOYMENT_ACTIONS = [
  "Deploy",
  "DestroyDeployment",
  "StopDeployment",
  "PauseDeployment",
  "StartDeployment",
  "RestartDeployment",
  "UnpauseDeployment",
] as const;

export const DeploymentGroupActions = () => {
  const [action, setAction] = useState<DeploymentActions>();

  return (
    <Select
      key={action}
      value={action}
      onValueChange={(action) => setAction(action as DeploymentActions)}
    >
      <SelectTrigger>
        <SelectValue />
      </SelectTrigger>
      <SelectContent>
        {DEPLOYMENT_ACTIONS.map((action) => (
          <SelectItem key={action} value={action}>
            {action}
          </SelectItem>
        ))}
      </SelectContent>
    </Select>
  );
};

export const DeploymentGroupActionDialog = ({
  action,
}: {
  action: DeploymentActions;
}) => {
  const deployments = useRead("ListDeployments", {}).data;
  const [selected] = useSelectedResources("Deployment");

  const { mutate } = useExecute(action);

  return (
    <Dialog>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Group Execute - {action}</DialogTitle>
        </DialogHeader>
        <div className="py-8">
          <p>
            Are you sure you wish to execute <b>{action}</b> for the selected
            Deployments?
          </p>
          <ul>
            {selected.map((s, i) => (
              <li key={i}>{deployments?.find((d) => d.name === s)?.name}</li>
            ))}
          </ul>
        </div>
        <DialogFooter>
          <Button onClick={() => mutate({ deployment })}>Confirm</Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
};
