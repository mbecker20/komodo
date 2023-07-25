import { Factory, Hammer, Rocket, Server } from "lucide-react";
import { useRead } from "@hooks";
import { Resources } from "@layouts/resources";
import { DeploymentCard } from "./deployment/card";
import { BuildCard } from "./build/card";
import { ServerCard } from "./server/card";
import { BuilderCard } from "./builder/card";

export const Deployments = () => {
  const deployments = useRead("ListDeployments", {}).data;
  const summary = useRead("GetDeploymentsSummary", {}).data;

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

export const Servers = () => {
  const servers = useRead("ListServers", {}).data;
  const summary = useRead("GetServersSummary", {}).data;
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

export const Builds = () => {
  const builds = useRead("ListBuilds", {}).data;
  const summary = useRead("GetBuildsSummary", {}).data;

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

export const Builders = () => {
  const builders = useRead("ListBuilders", {}).data;
  const summary = useRead("GetBuildersSummary", {}).data;

  return (
    <Resources
      type="Builder"
      info={`${summary?.total} Total`}
      icon={<Factory className="w-6 h-6" />}
      components={(search) => (
        <>
          {builders
            ?.filter((d) => d.name.includes(search) || search.includes(d.name))
            .map(({ _id }) => (
              <BuilderCard key={_id?.$oid} id={_id?.$oid!} />
            ))}
        </>
      )}
    />
  );
};
