import {
  ActionButton,
  ActionWithDialog,
  ConfirmButton,
} from "@components/util";
import { useExecute, useInvalidate, useRead, useWrite } from "@lib/hooks";
import { Types } from "@monitor/client";
import { IdComponent } from "@types";
import { Input } from "@ui/input";
import { useToast } from "@ui/use-toast";
import { Loader2, Pen, Scissors, Trash } from "lucide-react";
import { useState } from "react";
import { useNavigate } from "react-router-dom";

type PruneType = "Images" | "Containers" | "Networks";

const PruneButton = ({ type, id }: { type: PruneType; id: string }) => {
  const { mutate, isPending } = useExecute(`Prune${type}`);
  const pruning = useRead("GetServerActionState", { server: id }).data?.[
    `pruning_${type.toLowerCase()}` as keyof Types.ServerActionState
  ];
  const pending = isPending || pruning;
  return (
    <ConfirmButton
      title={`Prune ${type}`}
      icon={
        pending ? (
          <Loader2 className="w-4 h-4 animate-spin" />
        ) : (
          <Scissors className="w-4 h-4" />
        )
      }
      onClick={() => mutate({ server: id })}
    />
  );
};

const PRUNE_TYPES: PruneType[] = ["Images", "Containers"];

export const SERVER_ACTIONS: IdComponent[] = PRUNE_TYPES.map(
  (type) =>
    ({ id }) =>
      <PruneButton type={type} id={id} />
);

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
