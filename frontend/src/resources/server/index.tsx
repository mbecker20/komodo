import { ResourceUpdates } from "@components/updates/resource";
import { useRead, useAddRecentlyViewed } from "@hooks";
import { ResourceCard } from "@layouts/card";
import { Resource } from "@layouts/resource";
import { CardDescription } from "@ui/card";
import { useParams, Link } from "react-router-dom";
import { ServerConfig } from "./config";
import { ServerStats } from "./stats";
import {
  ServerName,
  ServerStatusIcon,
  ServerSpecs,
  ServerRegion,
} from "./util";

export const ServerPage = () => {
  const id = useParams().serverId;

  if (!id) return null;
  useAddRecentlyViewed("Server", id);

  return (
    <Resource
      title={<ServerName serverId={id} />}
      info={
        <div className="flex items-center gap-4">
          <ServerStatusIcon serverId={id} />
          <CardDescription className="hidden md:block">|</CardDescription>
          <ServerSpecs server_id={id} />
        </div>
      }
      actions={null}
    >
      <ResourceUpdates type="Server" id={id} />
      <ServerStats />
      <ServerConfig />
    </Resource>
  );
};

export const ServerCard = ({ id }: { id: string }) => {
  const servers = useRead("ListServers", {}).data;
  const server = servers?.find((server) => server.id === id);
  if (!server) return null;

  return (
    <Link to={`/servers/${server.id}`} key={server.id}>
      <ResourceCard
        title={server.name}
        description={server.status}
        statusIcon={<ServerStatusIcon serverId={server.id} />}
        // icon={<Server className="w-4 h-4" />}
      >
        <div className="flex flex-col text-sm">
          <ServerSpecs server_id={server.id} />
          <ServerRegion />
        </div>
      </ResourceCard>
    </Link>
  );
};
