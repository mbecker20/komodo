import { Component, Show } from "solid-js";
import ConfirmButton from "../../../util/ConfirmButton";
import Icon from "../../../util/Icon";
import Flex from "../../../util/layout/Flex";
import Grid from "../../../util/layout/Grid";
import Address from "./Address";
import Alerts from "./Alerts";
import Enabled from "./Enabled";
import Info from "./Info";
import Networks from "./Networks";
import Passkey from "./Passkey";
import { useConfig } from "./Provider";
import ToNotify from "./ToNotify";

const Config: Component<{}> = (p) => {
  const { server, reset, save } = useConfig();
  return (
    <Show when={server.loaded}>
      <Grid class="config">
        <Grid class="config-items scroller">
          <Show when={!server.isCore}>
            <Address />
            <Enabled />
            <Passkey />
          </Show>
          <Networks />
          <Info />
          <Alerts />
          <ToNotify />
        </Grid>
        <Show when={server.updated}>
          <Flex style={{ "place-self": "center", padding: "1rem" }}>
            <button onClick={reset}>
              reset
              <Icon type="reset" />
            </button>
            <ConfirmButton onConfirm={save} color="green">
              save
              <Icon type="floppy-disk" />
            </ConfirmButton>
          </Flex>
        </Show>
      </Grid>
    </Show>
  );
};

export default Config;
