import { useRead } from "@hooks";
import { Link } from "react-router-dom";
import { ServerStatusIcon, ServerSpecs, ServerRegion } from "./util";
import { ResourceCard } from "@layouts/card";

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
