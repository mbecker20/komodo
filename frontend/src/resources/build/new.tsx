import { useState } from "react";
import { useWrite } from "@hooks";
import { Input } from "@ui/input";
import { NewResource } from "@components/new-resource";
import { useNavigate } from "react-router-dom";

export const NewBuild = ({
  open,
  set,
}: {
  open: boolean;
  set: (b: false) => void;
}) => {
  const nav = useNavigate();
  const { mutate, isLoading } = useWrite("CreateBuild", {
    onSuccess: (d) => {
      set(false);
      nav(`/builds/${d._id?.$oid}`);
    },
  });

  const [name, setName] = useState("");

  return (
    <NewResource
      type="Build"
      open={open}
      loading={isLoading}
      set={set}
      onSuccess={() => mutate({ name, config: {} })}
    >
      <div className="flex items-center justify-between">
        <div>Build Name</div>
        <Input
          className="max-w-[50%]"
          placeholder="Build Name"
          name={name}
          onChange={(e) => setName(e.target.value)}
        />
      </div>
    </NewResource>
  );
};
