import { useRead } from "@hooks";
import { DeploymentCard } from "@pages/dashboard";

export const Deployments = () => {
  const deployments = useRead({ type: "ListDeployments", params: {} }).data;

  return (
    <div className="flex flex-col gap-12">
      <h1 className="text-3xl">Deployments</h1>
      <div className="grid grid-cols-4 gap-8">
        {deployments?.map(({ id }) => (
          <DeploymentCard key={id} id={id} />
        ))}
      </div>
    </div>
  );
};
