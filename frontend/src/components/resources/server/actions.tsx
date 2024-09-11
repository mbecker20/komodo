import { ActionWithDialog, ConfirmButton } from "@components/util";
import { useExecute, useInvalidate, useRead, useWrite } from "@lib/hooks";
import { Input } from "@ui/input";
import { useToast } from "@ui/use-toast";
import { Pen, Scissors } from "lucide-react";
import { useState } from "react";
import { useServer } from ".";
import { has_minimum_permissions } from "@lib/utils";
import { Types } from "@komodo/client";

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

export const Prune = ({
  server_id,
  type,
}: {
  server_id: string;
  type: "Containers" | "Networks" | "Images" | "Volumes" | "Buildx" | "System";
}) => {
  const server = useServer(server_id);
  const { mutate, isPending } = useExecute(`Prune${type}`);
  const action_state = useRead(
    "GetServerActionState",
    { server: server_id },
    { refetchInterval: 5000 }
  ).data;
  const perms = useRead("GetPermissionLevel", {
    target: { type: "Server", id: server_id },
  }).data;

  if (!server) return;

  const canExecute = has_minimum_permissions(
    perms,
    Types.PermissionLevel.Execute
  );

  const pruningKey =
    type === "Containers"
      ? "pruning_containers"
      : type === "Images"
      ? "pruning_images"
      : type === "Networks"
      ? "pruning_networks"
      : type === "Volumes"
      ? "pruning_volumes"
      : type === "Buildx"
      ? "pruning_buildx"
      : type === "System"
      ? "pruning_system"
      : "";

  const pending = isPending || action_state?.[pruningKey];

  if (type === "Images" || type === "Networks" || type === "Buildx") {
    return (
      <ConfirmButton
        title={`Prune ${type}`}
        icon={<Scissors className="w-4 h-4" />}
        onClick={() => mutate({ server: server_id })}
        loading={pending}
        disabled={!canExecute || pending}
      />
    );
  } else {
    return (
      <ActionWithDialog
        name={server?.name}
        title={`Prune ${type}`}
        icon={<Scissors className="w-4 h-4" />}
        onClick={() => mutate({ server: server_id })}
        loading={pending}
        disabled={!canExecute || pending}
      />
    );
  }
};
