import { Component, Show } from "solid-js";
import ConfirmButton from "../../../shared/ConfirmButton";
import Icon from "../../../shared/Icon";
import Flex from "../../../shared/layout/Flex";
import Grid from "../../../shared/layout/Grid";
import Address from "./Address";
import Alerts from "./Alerts";
import Enabled from "./Enabled";
import Info from "./Info";
import Networks from "./Networks";
import { useConfig } from "./Provider";
import ToNotify from "./ToNotify";

const Config: Component<{}> = (p) => {
  const { server, reset, save } = useConfig();
  return (
    <Show when={server.loaded}>
      <Grid class="config">
        <Grid class="config-items scroller">
          <Address />
          <Enabled />
          {/* <Networks /> */}
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
            <ConfirmButton onConfirm={save} class="green">
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
