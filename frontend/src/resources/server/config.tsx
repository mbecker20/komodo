import { Config } from "@components/config/Config";
import { useRead } from "@hooks";
import { Types } from "@monitor/client";
import { useState } from "react";
import { useParams } from "react-router-dom";

export const ServerConfig = () => {
  const id = useParams().serverId;
  const server = useRead("GetServer", { id });
  const [update, set] = useState<Partial<Types.ServerConfig>>({});
  if (server.data?.config) {
    return (
      <Config config={server.data?.config as any} update={update} set={set} />
    );
  } else {
    // loading
    return null;
  }
};
