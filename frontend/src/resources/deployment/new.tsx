import { useState } from "react";
import { useWrite } from "@hooks";
import { Input } from "@ui/input";
import { NewResource } from "@components/new-resource";
import { useNavigate } from "react-router-dom";

export const NewDeployment = ({
  open,
  set,
}: {
  open: boolean;
  set: (b: false) => void;
}) => {
  const nav = useNavigate();
  const { mutate, isLoading } = useWrite("CreateDeployment", {
    onSuccess: (d) => {
      set(false);
      nav(`/deployments/${d._id?.$oid}`);
    },
  });

  const [name, setName] = useState("");

  return (
    <NewResource
      type="Deployment"
      open={open}
      loading={isLoading}
      set={set}
      onSuccess={() => mutate({ name, config: {} })}
    >
      <div className="flex items-center justify-between">
        <div>Deployment Name</div>
        <Input
          className="max-w-[50%]"
          placeholder="Deployment Name"
          name={name}
          onChange={(e) => setName(e.target.value)}
        />
      </div>
    </NewResource>
  );
};
