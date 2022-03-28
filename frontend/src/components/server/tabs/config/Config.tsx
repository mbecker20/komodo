import { Component, Show } from "solid-js";
import { combineClasses } from "../../../../util/helpers";
import ConfirmButton from "../../../util/ConfirmButton";
import Icon from "../../../util/icons/Icon";
import Flex from "../../../util/layout/Flex";
import Grid from "../../../util/layout/Grid";
import s from "../../server.module.css";
import Address from "./Address";
import Enabled from "./Enabled";
import Networks from "./Networks";
import Passkey from "./Passkey";
import { useConfig } from "./Provider";

const Config: Component<{}> = (p) => {
	const { server, reset, save } = useConfig();
	return (
    <Show when={server.loaded}>
      <Grid class={s.Config}>
        <Grid class={combineClasses(s.ConfigItems, "scroller")}>
          <Show when={!server.isCore}>
            <Address />
            <Passkey />
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