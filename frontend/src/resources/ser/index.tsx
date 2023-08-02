import { ResourceUpdates } from "@components/updates/resource";
import { Resource } from "@layouts/resource";
import { ServerConfig } from "@resources/server/config";
import { ServerStatsPage } from "@resources/server/stats";
import {
  ServerName,
  ServerStatusIcon,
  ServerStats,
} from "@resources/server/util";
import { CardDescription } from "@ui/card";
import { useParams } from "react-router-dom";

export const ServerPage = () => {
  const id = useParams().serverId;
  if (!id) return null;

  return (
    <Resource
      title={<ServerName serverId={id} />}
      info={
        <div className="flex items-center gap-4">
          <ServerStatusIcon serverId={id} />
          <CardDescription className="hidden md:block">|</CardDescription>
          <ServerStats server_id={id} />
        </div>
      }
      actions={null}
    >
      <ResourceUpdates type="Server" id={id} />
      <ServerStatsPage />
      <ServerConfig />
    </Resource>
  );
};
