import { NewLayout } from "@components/layouts";
import { useInvalidate, useWrite } from "@lib/hooks";
import { Input } from "@ui/input";
import { useToast } from "@ui/use-toast";
import { useState } from "react";

export const NewUserGroup = () => {
  const { toast } = useToast();
  const inv = useInvalidate();
  const { mutateAsync } = useWrite("CreateUserGroup", {
    onSuccess: () => {
      inv(["ListUserGroups"]);
      toast({ title: "Created User Group" });
    },
  });
  const [name, setName] = useState("");
  return (
    <NewLayout
      entityType="User Group"
      onConfirm={() => mutateAsync({ name })}
      enabled={!!name}
      onOpenChange={() => setName("")}
    >
      <div className="grid md:grid-cols-2">
        Name
        <Input
          placeholder="user-group-name"
          value={name}
          onChange={(e) => setName(e.target.value)}
        />
      </div>
    </NewLayout>
  );
};

export const NewServiceUser = () => {
  const { toast } = useToast();
  const inv = useInvalidate();
  const { mutateAsync } = useWrite("CreateServiceUser", {
    onSuccess: () => {
      inv(["ListUsers"]);
      toast({ title: "Created Service User" });
    },
  });
  const [username, setUsername] = useState("");
  return (
    <NewLayout
      entityType="Service User"
      onConfirm={() => mutateAsync({ username, description: "" })}
      enabled={!!username}
      onOpenChange={() => setUsername("")}
    >
      <div className="grid md:grid-cols-2">
        Username
        <Input
          placeholder="username"
          value={username}
          onChange={(e) => setUsername(e.target.value)}
        />
      </div>
    </NewLayout>
  );
};
