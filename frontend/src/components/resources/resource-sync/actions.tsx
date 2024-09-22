import { ActionButton, ActionWithDialog } from "@components/util";
import { useExecute, useInvalidate, useRead, useWrite } from "@lib/hooks";
import { sync_no_changes } from "@lib/utils";
import { useEditPermissions } from "@pages/resource";
import { NotebookPen, RefreshCcw, SquarePlay } from "lucide-react";

export const RefreshSync = ({ id }: { id: string }) => {
  const inv = useInvalidate();
  const { mutate, isPending } = useWrite("RefreshResourceSyncPending", {
    onSuccess: () => inv(["GetResourceSync", { sync: id }]),
  });
  const pending = isPending;
  return (
    <ActionButton
      title="Refresh"
      icon={<RefreshCcw className="w-4 h-4" />}
      onClick={() => mutate({ sync: id })}
      disabled={pending}
      loading={pending}
    />
  );
};

export const ExecuteSync = ({ id }: { id: string }) => {
  const { mutate, isPending } = useExecute("RunSync");
  const syncing = useRead(
    "GetResourceSyncActionState",
    { sync: id },
    { refetchInterval: 5000 }
  ).data?.syncing;
  const sync = useRead("GetResourceSync", { sync: id }).data;

  if (!sync || sync_no_changes(sync)) return null;

  const pending = isPending || syncing;

  return (
    <ActionWithDialog
      name={sync.name}
      title="Execute Sync"
      icon={<SquarePlay className="w-4 h-4" />}
      onClick={() => mutate({ sync: id })}
      disabled={pending}
      loading={pending}
    />
  );
};

export const CommitSync = ({ id }: { id: string }) => {
  const { mutate, isPending } = useWrite("CommitSync");
  const sync = useRead("GetResourceSync", { sync: id }).data;
  const { canWrite } = useEditPermissions({ type: "ResourceSync", id });

  if (!canWrite) return null;

  if (!sync || !sync.config?.managed || sync_no_changes(sync)) return null;

  return (
    <ActionWithDialog
      name={sync.name}
      title="Commit Sync"
      icon={<NotebookPen className="w-4 h-4" />}
      onClick={() => mutate({ sync: id })}
      disabled={isPending}
      loading={isPending}
    />
  );
};
