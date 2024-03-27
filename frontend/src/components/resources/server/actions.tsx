import { ActionButton, ActionWithDialog } from "@components/util";
import { useInvalidate, useRead, useWrite } from "@lib/hooks";
import { Input } from "@ui/input";
import { useToast } from "@ui/use-toast";
import { Pen, Trash } from "lucide-react";
import { useState } from "react";
import { useNavigate } from "react-router-dom";

export const DeleteServer = ({ id }: { id: string }) => {
  const nav = useNavigate();
  const server = useRead("GetServer", { server: id }).data;
  const { mutateAsync, isPending } = useWrite("DeleteServer");

  if (!server) return null;
  return (
    <div className="flex items-center justify-between">
      <div className="w-full">Delete Server</div>
      <ActionWithDialog
        name={server.name}
        title="Delete"
        icon={<Trash className="h-4 w-4" />}
        onClick={async () => {
          await mutateAsync({ id });
          nav("/");
        }}
        disabled={isPending}
        loading={isPending}
      />
    </div>
  );
};

export const RenameServer = ({ id }: { id: string }) => {
  const invalidate = useInvalidate();

  const { toast } = useToast();
  const { mutate, isPending } = useWrite("RenameServer", {
    onSuccess: () => {
      invalidate(["ListServers"]);
      toast({ title: "Server Renamed" });
      set("");
    },
  });

  const [name, set] = useState("");

  return (
    <div className="flex items-center justify-between">
      <div className="w-full">Rename Server</div>
      <div className="flex gap-4 w-full justify-end">
        <Input
          value={name}
          onChange={(e) => set(e.target.value)}
          className="w-96"
          placeholder="Enter new name"
        />
        <ActionButton
          title="Rename"
          icon={<Pen className="w-4 h-4" />}
          disabled={isPending}
          onClick={() => mutate({ id, name })}
        />
      </div>
    </div>
  );
};
