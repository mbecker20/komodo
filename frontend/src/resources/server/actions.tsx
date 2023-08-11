import { ActionWithDialog } from "@components/util";
import { useRead, useWrite } from "@hooks";
import { Trash } from "lucide-react";
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
