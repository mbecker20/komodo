import { ConfirmButton } from "@components/util";
import { useExecute, useRead } from "@lib/hooks";
import { ArrowDownToDot, ArrowDownToLine, Loader2 } from "lucide-react";
import { useRepo } from ".";

export const CloneRepo = ({ id }: { id: string }) => {
  const hash = useRepo(id)?.info.latest_hash;
  const isCloned = (hash?.length || 0) > 0;
  const { mutate, isPending } = useExecute("CloneRepo");
  const cloning = useRead(
    "GetRepoActionState",
    { repo: id },
    { refetchInterval: 5000 }
  ).data?.cloning;
  const pending = isPending || cloning;
  return (
    <ConfirmButton
      title={isCloned ? "Reclone" : "Clone"}
      icon={
        pending ? (
          <Loader2 className="w-4 h-4 animate-spin" />
        ) : (
          <ArrowDownToLine className="w-4 h-4" />
        )
      }
      onClick={() => mutate({ repo: id })}
      disabled={pending}
      loading={pending}
    />
  );
};

export const PullRepo = ({ id }: { id: string }) => {
  
  const { mutate, isPending } = useExecute("PullRepo");
  const pulling = useRead(
    "GetRepoActionState",
    { repo: id },
    { refetchInterval: 5000 }
  ).data?.pulling;
  const hash = useRepo(id)?.info.latest_hash;
  const isCloned = (hash?.length || 0) > 0;
  if (!isCloned) return null;
  const pending = isPending || pulling;
  return (
    <ConfirmButton
      title="Pull"
      icon={
        pending ? (
          <Loader2 className="w-4 h-4 animate-spin" />
        ) : (
          <ArrowDownToDot className="w-4 h-4" />
        )
      }
      onClick={() => mutate({ repo: id })}
      disabled={pending}
      loading={pending}
    />
  );
};
