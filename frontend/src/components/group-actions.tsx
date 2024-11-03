import { useRead, useSelectedResources, useExecute } from "@lib/hooks";
import { UsableResource } from "@types";
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
import { Types } from "komodo_client";
import { ChevronDown, Loader2, CheckCircle } from "lucide-react";
import { useState } from "react";

export const GroupActions = <T extends Types.ExecuteRequest["type"]>({
  type,
  actions,
}: {
  type: UsableResource;
  actions: T[];
}) => {
  const [action, setAction] = useState<T>();
  const [selected] = useSelectedResources("Action");

  return (
    <>
      <GroupActionDropdownMenu
        actions={actions}
        onSelect={setAction}
        disabled={!selected.length}
      />
      <GroupActionDialog
        type={type}
        action={action}
        onClose={() => setAction(undefined)}
      />
    </>
  );
};

const GroupActionDropdownMenu = <T extends Types.ExecuteRequest["type"]>({
  actions,
  onSelect,
  disabled,
}: {
  actions: T[];
  onSelect: (item: T) => void;
  disabled: boolean;
}) => (
  <DropdownMenu>
    <DropdownMenuTrigger asChild disabled={disabled}>
      <Button variant="outline" className="w-40 justify-between">
        Group Actions <ChevronDown className="w-4" />
      </Button>
    </DropdownMenuTrigger>
    <DropdownMenuContent align="start" className="w-40">
      {actions.map((action) => (
        <DropdownMenuItem key={action} onClick={() => onSelect(action)}>
          {action}
        </DropdownMenuItem>
      ))}
    </DropdownMenuContent>
  </DropdownMenu>
);

const GroupActionDialog = ({
  type,
  action,
  onClose,
}: {
  type: UsableResource;
  action: Types.ExecuteRequest["type"] | undefined;
  onClose: () => void;
}) => {
  const resources = useRead(`List${type}s`, {}).data;

  const [selected] = useSelectedResources(type);
  const [text, setText] = useState("");

  const { mutate, isPending } = useExecute(action!, { onSuccess: onClose });

  return (
    <Dialog open={!!action} onOpenChange={(o) => !o && onClose()}>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Group Execute</DialogTitle>
        </DialogHeader>
        <div className="py-8 flex flex-col gap-4">
          <p>
            Are you sure you wish to execute <b>{action}</b> for the selected
            resources?
          </p>
          <ul className="p-4 bg-accent text-sm list-disc list-inside">
            {selected.map((s) => (
              <li key={s}>{resources?.find((r) => r.id === s)?.name}</li>
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
            className="gap-4"
          >
            Confirm
            {isPending ? (
              <Loader2 className="w-4 aspect-auto" />
            ) : (
              <CheckCircle className="w-4" />
            )}
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
};
