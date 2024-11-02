import { useExecute, useRead, useSelectedResources } from "@lib/hooks";
import { DialogTitle } from "@radix-ui/react-dialog";
import { Button } from "@ui/button";
import { Dialog, DialogContent, DialogFooter, DialogHeader } from "@ui/dialog";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@ui/select";
import { useState } from "react";

const DEPLOYMENT_ACTIONS = ["Deploy"] as const;
type DeploymentActions = (typeof DEPLOYMENT_ACTIONS)[number];

export const DeploymentGroupActions = () => {
  const [action, setAction] = useState<DeploymentActions>();

  return (
    <>
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
      {action && (
        <DeploymentGroupActionDialog
          action={action}
          onClose={() => setAction(undefined)}
        />
      )}
    </>
  );
};

const DeploymentGroupActionDialog = ({
  action,
  onClose,
}: {
  action: DeploymentActions;
  onClose: () => void;
}) => {
  const deployments = useRead("ListDeployments", {}).data;
  const [selected] = useSelectedResources("Deployment");

  const { mutate } = useExecute(`Batch${action}`);

  return (
    <Dialog open onOpenChange={(o) => !o && onClose()}>
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
          <Button onClick={() => mutate({ pattern: selected.join(",") })}>
            Confirm
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
};
