import { Component, Show } from "solid-js";
import ConfirmButton from "../../../util/ConfirmButton";
import Icon from "../../../util/Icon";
import Flex from "../../../util/layout/Flex";
import Grid from "../../../util/layout/Grid";
import Address from "./Address";
import Enabled from "./Enabled";
import Networks from "./Networks";
import { useConfig } from "./Provider";

const Config: Component<{}> = (p) => {
	const { server, reset, save } = useConfig();
	return (
    <Show when={server.loaded}>
      <Grid class="config">
        <Grid class="config-items scroller">
          <Show when={!server.isCore}>
            <Address />
            <Enabled />
          </Show>
          <Networks />
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
}

export default Config;