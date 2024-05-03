import { ConfirmButton } from "@components/util";
import { useExecute, useRead } from "@lib/hooks";
import { ArrowDownToDot, ArrowDownToLine, Loader2 } from "lucide-react";

export const CloneRepo = ({ id }: { id: string }) => {
  const { mutate, isPending } = useExecute("CloneRepo");
  const cloning = useRead(
    "GetRepoActionState",
    { repo: id },
    { refetchInterval: 5000 }
  ).data?.cloning;
  const pending = isPending || cloning;
  return (
    <ConfirmButton
      title="Clone"
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
