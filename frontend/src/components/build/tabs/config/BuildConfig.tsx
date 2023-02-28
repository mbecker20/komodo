import { Component, Show } from "solid-js";
import ConfirmButton from "../../../shared/ConfirmButton";
import Icon from "../../../shared/Icon";
import Flex from "../../../shared/layout/Flex";
import Grid from "../../../shared/layout/Grid";
import CliBuild from "./CliBuild";
import Docker from "./Docker";
import { useConfig } from "../Provider";
import Loading from "../../../shared/loading/Loading";
import BuildArgs from "./BuildArgs";
import Version from "./Version";
import Repo from "./Repo";
import WebhookUrl from "./WebhookUrl";

const BuildConfig: Component<{}> = (p) => {
  const { build, reset, save, userCanUpdate } = useConfig();
  return (
    <Show when={build.loaded}>
      <Grid class="config">
        <Grid class="config-items scroller">
          <Version />
          <Repo />
          <Docker />
          <CliBuild />
          <BuildArgs />
          <Show when={userCanUpdate()}>
            <WebhookUrl />
          </Show>
        </Grid>
        <Show when={userCanUpdate() && build.updated}>
          <Show
            when={!build.saving}
            fallback={
              <button class="green">
                updating <Loading type="spinner" />
              </button>
            }
          >
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
        </Show>
      </Grid>
    </Show>
  );
};

export default BuildConfig;
