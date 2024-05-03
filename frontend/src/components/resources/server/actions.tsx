import { ConfirmButton } from "@components/util";
import { useInvalidate, useWrite } from "@lib/hooks";
import { Input } from "@ui/input";
import { useToast } from "@ui/use-toast";
import { Pen } from "lucide-react";
import { useState } from "react";

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
