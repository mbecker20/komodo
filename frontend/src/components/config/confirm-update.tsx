import { Button } from "@ui/button";
import {
  Dialog,
  DialogContent,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from "@ui/dialog";
import { Save } from "lucide-react";
import { useState } from "react";

interface ConfirmUpdateProps {
  content: string;
  onConfirm: () => void;
}

export const ConfirmUpdate = ({ content, onConfirm }: ConfirmUpdateProps) => {
  const [open, set] = useState(false);
  return (
    <Dialog open={open} onOpenChange={set}>
      <DialogTrigger asChild>
        <Button variant="outline" intent="success" onClick={() => set(true)}>
          <Save className="w-4 h-4" />
        </Button>
      </DialogTrigger>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Confirm Update</DialogTitle>
        </DialogHeader>
        <div className="flex flex-col gap-4 py-4 my-4">
          New configuration to be applied:
          <pre className="h-[300px] overflow-auto">{content}</pre>
        </div>
        <DialogFooter>
          <Button
            variant="outline"
            intent="success"
            onClick={() => {
              onConfirm();
              set(false);
            }}
          >
            Confirm
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
};
