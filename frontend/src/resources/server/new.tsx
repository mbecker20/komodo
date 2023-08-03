import { useState } from "react";
import { useWrite } from "@hooks";
import { Input } from "@ui/input";
import { NewResource } from "@components/new-resource";
import { useNavigate } from "react-router-dom";

export const NewServer = ({
  open,
  set,
}: {
  open: boolean;
  set: (b: false) => void;
}) => {
  const nav = useNavigate();
  const { mutate, isLoading } = useWrite("CreateServer", {
    onSuccess: (d) => {
      set(false);
      nav(`/servers/${d._id?.$oid}`);
    },
  });

  const [name, setName] = useState("");

  return (
    <NewResource
      type="Server"
      open={open}
      loading={isLoading}
      set={set}
      onSuccess={() => mutate({ name, config: {} })}
    >
      <div className="flex items-center justify-between">
        <div>Server Name</div>
        <Input
          className="max-w-[50%]"
          placeholder="Server Name"
          name={name}
          onChange={(e) => setName(e.target.value)}
        />
      </div>
    </NewResource>
  );
};
