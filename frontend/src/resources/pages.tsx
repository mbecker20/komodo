import { useRead } from "@hooks";
import { Resources } from "@layouts/resources";
import { DeploymentCard } from "./deployment/card";
import { BuildCard } from "./build/card";
import { ServerCard } from "./server/card";
import { Hammer, Rocket, Server } from "lucide-react";

export const Deployments = () => {
  const deployments = useRead({ type: "ListDeployments", params: {} }).data;
  return (
    <Resources
      type="Deployment"
      info={`${deployments?.length} Total, 1 Running, 3 Stopped`}
      icon={<Rocket className="w-6 h-6" />}
      components={(search) => (
        <>
          {deployments
            ?.filter((d) => d.name.includes(search) || search.includes(d.name))
            .map(({ id }) => (
              <DeploymentCard key={id} id={id} />
            ))}
        </>
      )}
    />
  );
};

export const Builds = () => {
  const builds = useRead({ type: "ListBuilds", params: {} }).data;
  return (
    <Resources
      type="Build"
      info={`${builds?.length} Total`}
      icon={<Hammer className="w-6 h-6" />}
      components={(search) => (
        <>
          {builds
            ?.filter((d) => d.name.includes(search) || search.includes(d.name))
            .map(({ id }) => (
              <BuildCard key={id} id={id} />
            ))}
        </>
      )}
    />
  );
};

export const Servers = () => {
  const servers = useRead({ type: "ListServers", params: {} }).data;
  return (
    <Resources
      type="Server"
      info={`${servers?.length} Total, 1 Healthy, 0 Unhealthy`}
      icon={<Server className="w-6 h-6" />}
      components={(search) => (
        <>
          {servers
            ?.filter((d) => d.name.includes(search) || search.includes(d.name))
            .map(({ id }) => (
              <ServerCard key={id} id={id} />
            ))}
        </>
      )}
    />
  );
};
