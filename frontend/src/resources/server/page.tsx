import { useSetRecentlyViewed } from "@hooks";
import { Resource } from "@layouts/resource";
import { useParams } from "react-router-dom";
import { ServerInfo, ServerName } from "./util";
import { Server as ServerIcon } from "lucide-react";
import { CardDescription } from "@ui/card";

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
          <ServerIcon className="w-4 h-4" />
          <CardDescription className="hidden md:block">|</CardDescription>
          <ServerInfo serverId={serverId} />
        </div>
      }
      actions=""
      tabs={[
        {
          title: "Config",
          component: <>config</>,
        },
        {
          title: "Deployments",
          component: <>server deployments</>,
        },
        {
          title: "Stats",
          component: "server stats",
        },
        {
          title: "Updates",
          component: <>updates</>,
        },
      ]}
    />
  );
};
