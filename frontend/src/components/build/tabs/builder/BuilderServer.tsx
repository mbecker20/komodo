import { Component } from "solid-js";
import { useAppState } from "../../../../state/StateProvider";
import { PermissionLevel } from "../../../../types";
import { getId } from "../../../../util/helpers";
import Flex from "../../../shared/layout/Flex";
import Grid from "../../../shared/layout/Grid";
import Selector from "../../../shared/menu/Selector";
import { useConfig } from "../Provider";

const BuilderServer: Component<{}> = (p) => {
  const { servers, getPermissionOnServer } = useAppState();
	const { setBuild, server, userCanUpdate } = useConfig();
  const availableServers = () => {
    if (!servers.loaded()) return [];
    return servers
      .ids()!
      .filter((id) => {
        return getPermissionOnServer(id) === PermissionLevel.Update;
      });
  };
  return (
    <Flex
      class="config-item shadow"
      alignItems="center"
      justifyContent="space-between"
    >
      <h1>builder server</h1>
      <Selector
        targetClass="blue"
        selected={server()?.server ? getId(server()!.server) : "select server"}
        items={availableServers()}
        onSelect={(server_id) => setBuild("server_id", server_id)}
        itemMap={(server_id) =>
          server_id === "select server"
            ? "select server"
            : servers.get(server_id)!.server.name
        }
        disabled={!userCanUpdate()}
        position="bottom right"
        useSearch
      />
    </Flex>
  );
};

export default BuilderServer;
