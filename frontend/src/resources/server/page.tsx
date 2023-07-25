import { useSetRecentlyViewed } from "@hooks";
import { Resource } from "@layouts/resource";
import { useParams } from "react-router-dom";
import { ServerName, ServerStats } from "./util";
import { ServerStatusIcon } from "./util";
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
          <ServerStatusIcon serverId={serverId} />
          <CardDescription className="hidden md:block">|</CardDescription>
          <ServerStats server_id={serverId} />
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
