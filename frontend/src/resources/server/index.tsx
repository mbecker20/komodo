import { ResourceUpdates } from "@components/updates/resource";
import { useRead, useAddRecentlyViewed, useWrite } from "@hooks";
import { ResourceCard } from "@layouts/card";
import { Resource } from "@layouts/resource";
import { CardDescription } from "@ui/card";
import { useParams, Link } from "react-router-dom";
import { ServerStats } from "./stats";
import {
  ServerName,
  ServerStatusIcon,
  ServerSpecs,
  ServerRegion,
} from "./util";
import { useState } from "react";
import { Types } from "@monitor/client";
import { ConfigInner } from "@layouts/page";
import { DeleteServer } from "./actions";

export const ServerCard = ({ id }: { id: string }) => {
  const servers = useRead("ListServers", {}).data;
  const server = servers?.find((server) => server.id === id);
  if (!server) return null;

  return (
    <Link to={`/servers/${server.id}`} key={server.id}>
      <ResourceCard
        title={server.name}
        description={server.info.status}
        statusIcon={<ServerStatusIcon serverId={server.id} />}
        // icon={<Server className="w-4 h-4" />}
      >
        <div className="flex flex-col text-sm">
          <ServerSpecs server_id={server.id} />
          <ServerRegion serverId={server.id} />
        </div>
      </ResourceCard>
    </Link>
  );
};

const ServerConfig = ({ id }: { id: string }) => {
  const config = useRead("GetServer", { id }).data?.config;
  const [update, set] = useState<Partial<Types.ServerConfig>>({});
  const { mutate } = useWrite("UpdateServer");
  if (!config) return null;

  return (
    <ConfigInner
      config={config}
      update={update}
      set={set}
      onSave={() => mutate({ id, config: update })}
      components={{
        general: {
          general: {
            address: true,
            region: true,
            enabled: true,
            auto_prune: true,
          },
        },
        warnings: {
          cpu: {
            cpu_warning: true,
            cpu_critical: true,
          },
          memory: {
            mem_warning: true,
            mem_critical: true,
          },
          disk: {
            disk_warning: true,
            disk_critical: true,
          },
        },
      }}
    />
  );
};

export const ServerPage = () => {
  const id = useParams().serverId;
  useAddRecentlyViewed("Server", id);
  if (!id) return null;

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
      <ServerConfig id={id} />
      <div className="flex items-center justify-between w-full">
        danger zone {"B^)"}
        <DeleteServer id={id} />
      </div>
    </Resource>
  );
};
