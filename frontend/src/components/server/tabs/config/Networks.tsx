import { Network as NetworkType } from "@monitor/types";
import { Component, For } from "solid-js";
import { useAppState } from "../../../../state/StateProvider";
import { combineClasses } from "../../../../util/helpers";
import ConfirmButton from "../../../util/ConfirmButton";
import Icon from "../../../util/icons/Icon";
import Input from "../../../util/Input";
import Flex from "../../../util/layout/Flex";
import Grid from "../../../util/layout/Grid";
import s from "../../server.module.css";
import { useConfig } from "./Provider";

const BASE_NETWORKS = ["bridge", "host", "none"];

const Networks: Component<{}> = (p) => {
  const { server, setServer, networks } = useConfig();
  const filteredNetworks = () => {
    return networks().filter(
      (network) => !BASE_NETWORKS.includes(network.name)
    );
  };
  return (
    <Grid class={combineClasses(s.ConfigItem, "shadow")}>
      <Flex alignItems="center" justifyContent="space-between">
        <h1>networks</h1>
        <Flex alignItems="center">
          <Input />
          <ConfirmButton>create</ConfirmButton>
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
    <Flex class={s.Network} alignItems="center" justifyContent="space-between">
      <div>{p.network.name}</div>
      <ConfirmButton color="red">
        <Icon type="trash" />
      </ConfirmButton>
    </Flex>
  );
};

export default Networks;
