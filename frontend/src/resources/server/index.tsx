import { ResourceUpdates } from "@components/updates/resource";
import { useRead, useAddRecentlyViewed, useWrite } from "@hooks";
import { ResourceCard } from "@layouts/card";
import { Resource } from "@layouts/resource";
import { CardDescription } from "@ui/card";
import { useParams, Link } from "react-router-dom";
import { SerCon } from "./config";
import { ServerStats } from "./stats";
import {
  ServerName,
  ServerStatusIcon,
  ServerSpecs,
  ServerRegion,
} from "./util";
import { ServerConfig } from "@monitor/client/dist/types";
import { useState } from "react";
import { Types } from "@monitor/client";
import { ConfigLayout } from "@layouts/page";
import { Button } from "@ui/button";
import { ConfigAgain } from "@components/config/again";
import { ConfigInput } from "@components/config/util";

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

const ServerConfigInner = ({
  id,
  config,
}: {
  id: string;
  config: Types.ServerConfig;
}) => {
  const [update, set] = useState<Partial<Types.ServerConfig>>({});
  const [show, setShow] = useState("general");
  const { mutate } = useWrite("UpdateServer");

  <ConfigLayout
    content={update}
    onConfirm={() => mutate({ id, config: update })}
    onReset={() => set({})}
  >
    <div className="flex gap-4">
      <div className="flex flex-col gap-4 w-[300px]">
        <Button>General</Button>
      </div>
      {show === "general" && (
        <ConfigAgain
          config={config}
          update={update}
          components={{
            address: (value) => (
              <ConfigInput
                label="Address"
                value={value}
                onChange={(address) => set((p) => ({ ...p, address }))}
              />
            ),
            region: (value) => (
              <ConfigInput
                label="region"
                value={value}
                onChange={(region) => set((p) => ({ ...p, region }))}
              />
            ),
            address: (value) => (
              <ConfigInput
                label="Address"
                value={value}
                onChange={(address) => set((p) => ({ ...p, address }))}
              />
            ),
            address: (value) => (
              <ConfigInput
                label="Address"
                value={value}
                onChange={(address) => set((p) => ({ ...p, address }))}
              />
            ),
          }}
        />
      )}
    </div>
  </ConfigLayout>;
};

const ServerConfig = ({ id }: { id: string }) => {
  const config = useRead("GetServer", { id }).data?.config;
  if (!config) return null;
  return <div></div>;
};

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
      <SerCon id={id} />
    </Resource>
  );
};
