import { Component, Show } from "solid-js";
import Grid from "../../../util/layout/Grid";
import Image from "./Image";
import Network from "./Network";
import Mounts from "./Volumes";
import Env from "./Env";
import Ports from "./Ports";
import { useConfig } from "./Provider";
import Flex from "../../../util/layout/Flex";
import Icon from "../../../util/icons/Icon";
import ConfirmButton from "../../../util/ConfirmButton";
import Restart from "./Restart";
import DockerAccount from "./DockerAccount";

const Config: Component<{}> = (p) => {
  const { deployment, reset, save } = useConfig();
  return (
    <Show when={deployment.loaded}>
      <Grid class="config">
        <Grid class="config-items scroller">
          <Image />
          <Show when={deployment.image}>
            <DockerAccount />
          </Show>
          <Network />
          <Restart />
          <Ports />
          <Mounts />
          <Env />
        </Grid>
        <Show when={deployment.updated}>
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
