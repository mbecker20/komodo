import { useRead } from "@hooks";
import { DeploymentCard } from "@resources/deployment/card";
import { NewDeployment } from "@resources/deployment/new";
import { Button } from "@ui/button";
import { Input } from "@ui/input";
import { PlusCircle } from "lucide-react";
import { useState } from "react";

export const Deployments = () => {
  const deployments = useRead({ type: "ListDeployments", params: {} }).data;
  const [search, setSearch] = useState("");
  const [open, setOpen] = useState(false);

  return (
    <div className="flex flex-col gap-12">
      <div className="flex justify-between">
        <h1 className="text-3xl">Deployments</h1>
        <div className="flex gap-4">
          <Input
            className="w-[300px]"
            placeholder="Search"
            value={search}
            onChange={(e) => setSearch(e.target.value)}
          />
          <Button
            className="w-[200px] flex items-center gap-2"
            variant="outline"
            intent="success"
            onClick={() => setOpen(true)}
          >
            <PlusCircle className="w-4 h-4 text-green-500" />
            New Deployment
          </Button>
          <NewDeployment open={open} set={setOpen} />
        </div>
      </div>
      <div className="grid grid-cols-3 gap-8">
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
