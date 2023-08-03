import { useState } from "react";
import { useWrite } from "@hooks";
import { Button } from "@ui/button";
import {
  Dialog,
  DialogContent,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "@ui/dialog";
import { Input } from "@ui/input";

export const NewBuild = ({
  open,
  set,
}: {
  open: boolean;
  set: (b: false) => void;
}) => {
  const { mutate } = useWrite("CreateBuild");
  const [name, setName] = useState("");

  return (
    <Dialog open={open} onOpenChange={set}>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>New Build</DialogTitle>
        </DialogHeader>
        <div className="flex items-center justify-between">
          <div>Build Name</div>
          <Input
            className="max-w-[50%]"
            placeholder="Build Name"
            name={name}
            onChange={(e) => setName(e.target.value)}
          />
        </div>
        <DialogFooter>
          <Button
            variant="outline"
            intent="success"
            onClick={() => {
              mutate({ name, config: {} });
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
