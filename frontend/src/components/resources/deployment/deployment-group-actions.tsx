import { useExecute, useRead, useSelectedResources } from "@lib/hooks";
import { Button } from "@ui/button";
import {
  Dialog,
  DialogContent,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "@ui/dialog";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from "@ui/dropdown-menu";
import { Input } from "@ui/input";
import { ChevronDown } from "lucide-react";
import { useState } from "react";

const DEPLOYMENT_ACTIONS = ["Deploy"] as const;
type DeploymentActions = (typeof DEPLOYMENT_ACTIONS)[number];

export const DeploymentGroupActions = () => {
  const [action, setAction] = useState<DeploymentActions>();

  return (
    <>
      <DropdownMenu>
        <DropdownMenuTrigger asChild>
          <Button variant="outline" className="w-40 justify-between">
            Group Actions <ChevronDown className="w-4" />
          </Button>
        </DropdownMenuTrigger>
        <DropdownMenuContent align="start" className="w-40">
          {DEPLOYMENT_ACTIONS.map((action) => (
            <DropdownMenuItem key={action} onClick={() => setAction(action)}>
              {action}
            </DropdownMenuItem>
          ))}
        </DropdownMenuContent>
      </DropdownMenu>
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

  const [text, setText] = useState("");

  return (
    <Dialog open onOpenChange={(o) => !o && onClose()}>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Group Execute - {action}</DialogTitle>
        </DialogHeader>
        <div className="py-8 flex flex-col gap-4">
          <p>
            Are you sure you wish to execute <b>{action}</b> for the selected
            resources?
          </p>
          <ul className="p-4 bg-accent text-sm list-disc list-inside">
            {selected.map((s, i) => (
              <li key={i}>{deployments?.find((d) => d.id === s)?.name}</li>
            ))}
          </ul>
          <p>
            Please enter <b>{action}</b> in the input below to confirm.
          </p>
          <Input value={text} onChange={(e) => setText(e.target.value)} />
        </div>
        <DialogFooter>
          <Button
            disabled={text !== action}
            onClick={() => mutate({ pattern: selected.join(",") })}
          >
            Confirm
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
};
