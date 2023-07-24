import { useRead } from "@hooks";
import { DeploymentCard } from "@pages/dashboard";
import { Button } from "@ui/button";
import { Input } from "@ui/input";
import { PlusCircle } from "lucide-react";
import { useState } from "react";

export const Deployments = () => {
  const deployments = useRead({ type: "ListDeployments", params: {} }).data;
  const [search, set] = useState("");

  return (
    <div className="flex flex-col gap-12">
      <div className="flex justify-between">
        <h1 className="text-3xl">Deployments</h1>
        <div className="flex gap-4">
          <Input
            className="w-[300px]"
            placeholder="Search"
            value={search}
            onChange={(e) => set(e.target.value)}
          />
          <Button
            className="w-[200px] flex items-center gap-2"
            variant="outline"
            intent="success"
          >
            <PlusCircle className="w-4 h-4 text-green-500" />
            New Deployment
          </Button>
        </div>
      </div>
      <div className="grid grid-cols-4 gap-8">
        {deployments?.map(
          ({ id, name }) =>
            (search.includes(name) || name.includes(search)) && (
              <DeploymentCard key={id} id={id} />
            )
        )}
      </div>
    </div>
  );
};
