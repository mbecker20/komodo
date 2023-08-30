import { ActionButton, ActionWithDialog } from "@components/util";
import { useInvalidate, useRead, useWrite } from "@hooks";
import { Input } from "@ui/input";
import { useToast } from "@ui/toast/use-toast";
import { Pen, Trash } from "lucide-react";
import { useState } from "react";
import { useNavigate } from "react-router-dom";

export const DeleteServer = ({ id }: { id: string }) => {
  const nav = useNavigate();
  const { data: d } = useRead("GetServer", { id });
  const { mutateAsync, isLoading } = useWrite("DeleteServer");

  if (!d) return null;
  return (
    <ActionWithDialog
      name={d.name}
      title="Delete Server"
      intent="danger"
      icon={<Trash className="h-4 w-4" />}
      onClick={async () => {
        await mutateAsync({ id });
        nav("/");
      }}
      disabled={isLoading}
    />
  );
};

export const RenameServer = ({ id }: { id: string }) => {
  const invalidate = useInvalidate();

  const { toast } = useToast();
  const { mutate, isLoading } = useWrite("RenameServer", {
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
          disabled={isLoading}
          onClick={() => mutate({ id, name })}
        />
      </div>
    </div>
  );
};
