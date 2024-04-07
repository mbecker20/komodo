import {
  ActionWithDialog,
  ConfirmButton,
} from "@components/util";
import { useExecute, useInvalidate, useRead, useWrite } from "@lib/hooks";
import { IdComponent } from "@types";
import { Input } from "@ui/input";
import { useToast } from "@ui/use-toast";
import { Pen, Scissors, XOctagon } from "lucide-react";
import { useState } from "react";
import { useServer } from ".";

export const SERVER_ACTIONS: IdComponent[] = [
  ({ id }) => {
    const { mutate, isPending } = useExecute(`PruneImages`);
    const pruning = useRead("GetServerActionState", { server: id }).data
      ?.pruning_images;
    const pending = isPending || pruning;
    return (
      <ConfirmButton
        title="Prune Images"
        icon={<Scissors className="w-4 h-4" />}
        onClick={() => mutate({ server: id })}
        loading={pending}
        disabled={pending}
      />
    );
  },
  ({ id }) => {
    const server = useServer(id);
    const { mutate, isPending } = useExecute(`StopAllContainers`);
    const stopping = useRead("GetServerActionState", { server: id }).data
      ?.stopping_containers;
    const pending = isPending || stopping;
    return (
      server && (
        <ActionWithDialog
          name={server?.name}
          title="Stop Containers"
          icon={<XOctagon className="w-4 h-4" />}
          onClick={() => mutate({ server: id })}
          disabled={pending}
          loading={pending}
        />
      )
    );
  },
];

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
        <ConfirmButton
          title="Rename"
          icon={<Pen className="w-4 h-4" />}
          disabled={!name || isPending}
          loading={isPending}
          onClick={() => mutate({ id, name })}
        />
      </div>
    </div>
  );
};
