import { Component, Show } from "solid-js";
import { useAppState } from "../../state/StateProvider";
import { useUser } from "../../state/UserProvider";
import ConfirmButton from "../shared/ConfirmButton";
import Icon from "../shared/Icon";
import Flex from "../shared/layout/Flex";
import Grid from "../shared/layout/Grid";
import Loading from "../shared/loading/Loading";
import { useActionStates } from "./ActionStateProvider";
import { client } from "../..";
import { combineClasses, getId } from "../../util/helpers";
import { useParams } from "@solidjs/router";
import { PermissionLevel } from "../../types";

const Actions: Component<{}> = (p) => {
  const { user } = useUser();
  const params = useParams() as { id: string };
  const { builds, ws } = useAppState();
  const build = () => builds.get(params.id)!;
  const actions = useActionStates();
  const userCanExecute = () =>
    user().admin ||
    build().permissions![getId(user())] === PermissionLevel.Execute ||
    build().permissions![getId(user())] === PermissionLevel.Execute;
  return (
    <Show when={userCanExecute()}>
      <Grid class={combineClasses("card shadow")} gridTemplateRows="auto 1fr">
        <h1>actions</h1>
        <Grid style={{ height: "fit-content" }}>
          <Flex class={combineClasses("action shadow")}>
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
                class="green"
                onConfirm={() => {
                  client.build(params.id);
                }}
              >
                <Icon type="build" />
              </ConfirmButton>
            </Show>
          </Flex>
          <Flex class={combineClasses("action shadow")}>
            reclone{" "}
            <Show
              when={!actions.recloning}
              fallback={
                <button class="orange">
                  <Loading type="spinner" />
                </button>
              }
            >
              <ConfirmButton
                class="orange"
                onConfirm={() => {
                  client.reclone_build(params.id);
                }}
              >
                <Icon type="reset" />
              </ConfirmButton>
            </Show>
          </Flex>
        </Grid>
      </Grid>
    </Show>
  );
};

export default Actions;
