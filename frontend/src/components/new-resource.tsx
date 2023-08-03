import { Types } from "@monitor/client";
import { Button } from "@ui/button";
import {
  Dialog,
  DialogContent,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "@ui/dialog";
import { ReactNode } from "react";

export const NewResource = ({
  open,
  loading,
  type,
  children,
  set,
  onSuccess,
}: {
  open: boolean;
  loading: boolean;
  type: Types.ResourceTarget["type"];
  children: ReactNode;
  set: (b: false) => void;
  onSuccess: () => void;
}) => {
  return (
    <Dialog open={open} onOpenChange={set}>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>New {type}</DialogTitle>
        </DialogHeader>
        <div className="flex flex-col gap-4">{children}</div>
        <DialogFooter>
          <Button
            variant="outline"
            intent="success"
            onClick={onSuccess}
            disabled={loading}
          >
            Create
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
};
