import { Network as NetworkType } from "@monitor/types";
import { Component, createSignal, For } from "solid-js";
import { pushNotification } from "../../../..";
import { CREATE_NETWORK, DELETE_NETWORK } from "../../../../state/actions";
import { useAppState } from "../../../../state/StateProvider";
import ConfirmButton from "../../../util/ConfirmButton";
import Icon from "../../../util/icons/Icon";
import Input from "../../../util/Input";
import Flex from "../../../util/layout/Flex";
import Grid from "../../../util/layout/Grid";
import { useConfig } from "./Provider";

const BASE_NETWORKS = ["bridge", "host", "none"];

const Networks: Component<{}> = (p) => {
  const { ws, selected } = useAppState();
  const { networks } = useConfig();
  const filteredNetworks = () => {
    return networks().filter(
      (network) => !BASE_NETWORKS.includes(network.name)
    );
  };
  const [name, setName] = createSignal("");
  return (
    <Grid class="config-item shadow">
      <Flex alignItems="center" justifyContent="space-between">
        <h1>networks</h1>
        <Flex alignItems="center">
          <Input
            placeholder="new network name"
            value={name()}
            onEdit={setName}
          />
          <ConfirmButton
            onConfirm={() => {
              if (name().length > 0) {
                ws.send(CREATE_NETWORK, {
                  serverID: selected.id(),
                  name: name(),
                });
                setName("");
              } else {
                pushNotification("bad", "please enter a name");
              }
            }}
          >
            create
          </ConfirmButton>
        </Flex>
      </Flex>
      <For each={filteredNetworks()}>
        {(network) => <Network network={network} />}
      </For>
    </Grid>
  );
};

const Network: Component<{ network: NetworkType }> = (p) => {
  const { selected, ws } = useAppState();
  return (
    <Flex
      class="grey-no-hover"
      alignItems="center"
      justifyContent="space-between"
      style={{
        padding: "0.5rem",
        transition: "background-color 125ms ease-in-out",
      }}
    >
      <div>{p.network.name}</div>
      <ConfirmButton
        color="red"
        onConfirm={() => {
          ws.send(DELETE_NETWORK, {
            serverID: selected.id(),
            name: p.network.name,
          });
        }}
      >
        <Icon type="trash" />
      </ConfirmButton>
    </Flex>
  );
};

export default Networks;
