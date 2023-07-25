import { useRead } from "@hooks";
import { Resources } from "@layouts/resources";
import { DeploymentCard } from "./deployment/card";
import { BuildCard } from "./build/card";
import { ServerCard } from "./server/card";
import { Hammer, Rocket, Server } from "lucide-react";

export const Deployments = () => {
  const deployments = useRead({ type: "ListDeployments", params: {} }).data;
  const summary = useRead({ type: "GetDeploymentsSummary", params: {} }).data;

  return (
    <Resources
      type="Deployment"
      info={`${summary?.total} Total, ${summary?.running} Running, ${summary?.stopped} Stopped, ${summary?.unknown} Unknown`}
      icon={<Rocket className="w-4 h-4" />}
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
  const summary = useRead({ type: "GetBuildsSummary", params: {} }).data;

  return (
    <Resources
      type="Build"
      info={`${summary?.total} Total`}
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
  const summary = useRead({ type: "GetServersSummary", params: {} }).data;
  return (
    <Resources
      type="Server"
      info={`${summary?.total} Total, ${summary?.healthy} Healthy, ${summary?.unhealthy} Unhealthy`}
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
