import { useState } from "react";
import { Button } from "@ui/button";
import {
  Dialog,
  DialogContent,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "@ui/dialog";
import { Input } from "@ui/input";
import { useWrite } from "@hooks";
import { useNavigate } from "react-router-dom";

export const NewDeployment = ({
  open,
  set,
}: {
  open: boolean;
  set: (b: boolean) => void;
}) => {
  const nav = useNavigate();
  const { mutateAsync } = useWrite("CreateDeployment");
  const [name, setName] = useState("");

  return (
    <Dialog open={open} onOpenChange={set}>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>New Deployment</DialogTitle>
        </DialogHeader>
        <div className="flex items-center justify-between">
          <div>Deployment Name</div>
          <Input
            className="max-w-[50%]"
            placeholder="Deployment Name"
            name={name}
            onChange={(e) => setName(e.target.value)}
          />
        </div>
        <DialogFooter>
          <Button
            variant="outline"
            intent="success"
            onClick={async () => {
              const { _id } = await mutateAsync({ name, config: {} });
              nav(`/deployments/${_id?.$oid}`);
              set(false);
            }}
          >
            Create
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
};
