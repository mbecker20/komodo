import { Component, Show } from "solid-js";
import { pushNotification, URL } from "../../../..";
import { combineClasses, copyToClipboard } from "../../../../util/helpers";
import ConfirmButton from "../../../shared/ConfirmButton";
import Icon from "../../../shared/Icon";
import Flex from "../../../shared/layout/Flex";
import Grid from "../../../shared/layout/Grid";
import Loading from "../../../shared/loading/Loading";
import { useConfig } from "../Provider";
import Git from "./Git";
import OnClone from "./OnClone";

const GitConfig: Component<{}> = (p) => {
  const { build, reset, save, userCanUpdate } = useConfig();
  const listenerUrl = () => `${URL}/api/listener/build/${build._id}`;
  return (
    <Show when={build.loaded}>
      <Grid class="config">
        <Grid class="config-items scroller">
          <Git />
          <OnClone />
          <Show when={userCanUpdate()}>
            <Grid class={combineClasses("config-item shadow")}>
              <h1>webhook url</h1>
              <Flex justifyContent="space-between" alignItems="center">
                <div class="ellipsis" style={{ width: "250px" }}>
                  {listenerUrl()}
                </div>
                <ConfirmButton
                  color="blue"
                  onFirstClick={() => {
                    copyToClipboard(listenerUrl());
                    pushNotification("good", "copied url to clipboard");
                  }}
                  confirm={<Icon type="check" />}
                >
                  <Icon type="clipboard" />
                </ConfirmButton>
              </Flex>
            </Grid>
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
              <ConfirmButton onConfirm={save} color="green">
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

export default GitConfig;
