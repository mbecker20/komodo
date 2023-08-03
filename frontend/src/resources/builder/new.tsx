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

export const NewBuilder = ({
  open,
  set,
}: {
  open: boolean;
  set: (b: false) => void;
}) => {
  const nav = useNavigate();
  const { mutateAsync } = useWrite("CreateBuilder");
  const [name, setName] = useState("");

  return (
    <Dialog open={open} onOpenChange={set}>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>New Builder</DialogTitle>
        </DialogHeader>
        <div className="flex items-center justify-between">
          <div>Builder Name</div>
          <Input
            className="max-w-[50%]"
            placeholder="Builder Name"
            name={name}
            onChange={(e) => setName(e.target.value)}
          />
        </div>
        <DialogFooter>
          <Button
            variant="outline"
            intent="success"
            onClick={async () => {
              const { _id } = await mutateAsync({
                name,
                config: { type: "Aws", params: {} },
              });
              nav(`/builders/${_id?.$oid}`);
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
