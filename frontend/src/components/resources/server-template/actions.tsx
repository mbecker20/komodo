import { ActionButton, ConfirmButton } from "@components/util";
import { useExecute } from "@lib/hooks";
import { useToast } from "@ui/use-toast";
import { useState } from "react";
import { useServerTemplate } from ".";
import { Server } from "lucide-react";
import { Input } from "@ui/input";
import {
  Dialog,
  DialogContent,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from "@ui/dialog";
import { useWebsocketMessages } from "@lib/socket";
import { useNavigate } from "react-router-dom";
import { Types } from "@monitor/client";

export const LaunchServer = ({ id }: { id: string }) => {
  const nav = useNavigate();
  const { toast } = useToast();
  const [name, setName] = useState("");
  const [open, setOpen] = useState(false);
  const { mutate } = useExecute("LaunchServer");
  const template = useServerTemplate(id);

  useWebsocketMessages("server-launch", (update) => {
    if (
      update.target.type === "ServerTemplate" &&
      update.target.id === id &&
      update.operation === Types.Operation.LaunchServer &&
      update.status === Types.UpdateStatus.Complete &&
      update.success &&
      update.other_data
    ) {
      // The 'other_data' in this case will be created server id
      nav(`/servers/${update.other_data}`);
    }
  });

  if (!template) return;

  const launch = () => {
    if (!name) {
      toast({ title: "Name cannot be empty" });
      return;
    }
    mutate({ name, server_template: id });
  };

  return (
    <Dialog
      open={open}
      onOpenChange={(open) => {
        setOpen(open);
        setName("");
      }}
    >
      <DialogTrigger asChild>
        <ActionButton
          title="Launch Server"
          icon={<Server className="w-4 h-4" />}
          onClick={() => setOpen(true)}
        />
      </DialogTrigger>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Launch Server</DialogTitle>
        </DialogHeader>
        <Input
          placeholder="Enter server name"
          value={name}
          onChange={(e) => setName(e.target.value)}
          className="w-full"
        />
        <DialogFooter>
          <ConfirmButton
            title="Launch Server"
            icon={<Server className="w-4 h-4" />}
            onClick={() => {
              launch();
              setOpen(false);
            }}
          />
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
};
