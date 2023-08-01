import { useSetRecentlyViewed } from "@hooks";
import { Resource } from "@layouts/resource";
import { useParams } from "react-router-dom";
import { ServerName, ServerStats } from "./util";
import { ServerStatusIcon } from "./util";
import { CardDescription } from "@ui/card";
import { ResourceUpdates } from "@components/updates/resource";
import { ServerStatsPage } from "./stats";
import { ServerConfig } from "./config";

export const Server = () => {
  const { serverId } = useParams();
  const push = useSetRecentlyViewed();
  if (!serverId) return null;
  push("Server", serverId);

  return (
    <Resource
      title={<ServerName serverId={serverId} />}
      info={
        <div className="flex items-center gap-4">
          <ServerStatusIcon serverId={serverId} />
          <CardDescription className="hidden md:block">|</CardDescription>
          <ServerStats server_id={serverId} />
        </div>
      }
      actions={
        <div className="flex gap-4">
          {/* <Link to={`/servers/${serverId}/config`}>
            <Button variant="outline">
              <Settings className="w-4 h-4" />
            </Button>
          </Link> */}
        </div>
      }
    />
  );
};

export const ServerContent = () => {
  const id = useParams().serverId;
  if (!id) return null;
  return (
    <div className="flex flex-col gap-12">
      <ResourceUpdates type="Server" id={id} />
      <ServerStatsPage />
      <ServerConfig />
    </div>
  );
};
