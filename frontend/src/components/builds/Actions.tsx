import { BuildActionState } from "@monitor/types";
import { Component, createEffect, onCleanup, Show } from "solid-js";
import { createStore } from "solid-js/store";
import { BUILD, CLONE_BUILD_REPO } from "../../state/actions";
import { useAppState } from "../../state/StateProvider";
import { combineClasses } from "../../util/helpers";
import { getBuildActionState } from "../../util/query";
import ConfirmButton from "../util/ConfirmButton";
import Icon from "../util/icons/Icon";
import Flex from "../util/layout/Flex";
import Grid from "../util/layout/Grid";
import Loading from "../util/loading/Loading";

const Actions: Component<{}> = (p) => {
  const { builds, selected, ws } = useAppState();
  const build = () => builds.get(selected.id())!;
  const [actions, setActions] = createStore<BuildActionState>({
    pulling: false,
    building: false,
    cloning: false,
    updating: false,
  });
  createEffect(() => {
    getBuildActionState(selected.id()).then(setActions);
  });
  onCleanup(
    ws.subscribe([BUILD], ({ complete, buildID }) => {
      if (buildID === selected.id()) {
        setActions("building", !complete);
      }
    })
  );
  onCleanup(
    ws.subscribe([CLONE_BUILD_REPO], ({ complete, buildID }) => {
      if (buildID === selected.id()) {
        setActions("cloning", !complete);
      }
    })
  );
  return (
    <Show when={build()}>
      <Grid class="card shadow">
        <h1>actions</h1>
        <Flex class="action shadow">
          build{" "}
          <Show
            when={!actions.building}
            fallback={
              <button class="green">
                <Loading type="spinner" />
              </button>
            }
          >
            <ConfirmButton
              color="green"
              onConfirm={() => {
                ws.send(BUILD, { buildID: selected.id() });
              }}
            >
              <Icon type="build" />
            </ConfirmButton>
          </Show>
        </Flex>
        <Flex class="action shadow">
          reclone{" "}
          <Show
            when={!actions.cloning}
            fallback={
              <button class="orange">
                <Loading type="spinner" />
              </button>
            }
          >
            <ConfirmButton
              color="orange"
              onConfirm={() => {
                ws.send(CLONE_BUILD_REPO, { buildID: selected.id() });
              }}
            >
              <Icon type="arrow-down" />
            </ConfirmButton>
          </Show>
        </Flex>
      </Grid>
    </Show>
  );
};

export default Actions;
