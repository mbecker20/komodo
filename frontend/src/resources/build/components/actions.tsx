import { ActionButton } from "@components/util";
import { useExecute } from "@hooks";
import { Hammer } from "lucide-react";

export const RebuildBuild = ({ buildId }: { buildId: string }) => {
  const { mutate, isLoading } = useExecute("RunBuild");
  return (
    <ActionButton
      intent="success"
      title="Build"
      icon={<Hammer className="h-4 w-4" />}
      onClick={() => mutate({ build_id: buildId })}
      disabled={isLoading}
    />
  );
};
