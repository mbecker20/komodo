import { useRead } from "@hooks";
import { Resources } from "@layouts/resources";
import { DeploymentCard } from "./deployment/card";
import { BuildCard } from "./build/card";
import { ServerCard } from "./server/card";

export const Deployments = () => {
  const deployments = useRead({ type: "ListDeployments", params: {} }).data;
  return (
    <Resources
      title="Deployments"
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
      title="Builds"
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
      title="Servers"
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
