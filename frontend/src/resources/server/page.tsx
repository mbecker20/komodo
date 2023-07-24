import { useSetRecentlyViewed } from "@hooks";
import { Resource } from "@layouts/resource";
import { useParams } from "react-router-dom";
import { ServerInfo, ServerName } from "./util";

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
      info={<ServerInfo serverId={serverId} />}
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
