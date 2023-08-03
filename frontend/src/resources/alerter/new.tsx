import { useState } from "react";
import { useWrite } from "@hooks";
import { Input } from "@ui/input";
import { NewResource } from "@components/new-resource";
import { useNavigate } from "react-router-dom";

export const NewAlerter = ({
  open,
  set,
}: {
  open: boolean;
  set: (b: false) => void;
}) => {
  const nav = useNavigate();
  const { mutate, isLoading } = useWrite("CreateAlerter", {
    onSuccess: (d) => {
      set(false);
      nav(`/alerters/${d._id?.$oid}`);
    },
  });

  const [name, setName] = useState("");

  return (
    <NewResource
      type="Alerter"
      open={open}
      loading={isLoading}
      set={set}
      onSuccess={() => mutate({ name, config: { type: "Custom", params: {} } })}
    >
      <div className="flex items-center justify-between">
        <div>Alerter Name</div>
        <Input
          className="max-w-[50%]"
          placeholder="Alerter Name"
          name={name}
          onChange={(e) => setName(e.target.value)}
        />
      </div>
    </NewResource>
  );
};
