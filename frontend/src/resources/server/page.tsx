import { useSetRecentlyViewed } from "@hooks";
import { Resource } from "@layouts/resource";
import { Link, useParams } from "react-router-dom";
import { ServerName, ServerStats } from "./util";
import { ServerStatusIcon } from "./util";
import { CardDescription } from "@ui/card";
import { Button } from "@ui/button";
import { Settings } from "lucide-react";

export const Server = () => {
  const { serverId } = useParams();
  const push = useSetRecentlyViewed();

  // if (!serverId) return null;
  // push("Server", serverId!);
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
          <Link to={`/servers/${serverId}/config`}>
            <Button variant="outline">
              <Settings className="w-4 h-4" />
            </Button>
          </Link>
        </div>
      }
    />
  );
};
