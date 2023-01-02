import { useNavigate, useParams } from "@solidjs/router";
import { Component, createEffect, onCleanup, Show } from "solid-js";
import { useAppDimensions } from "../../state/DimensionProvider";
import { useAppState } from "../../state/StateProvider";
import { useUser } from "../../state/UserProvider";
import { Operation, PermissionLevel } from "../../types";
import { combineClasses, getId } from "../../util/helpers";
import NotFound from "../NotFound";
import Grid from "../shared/layout/Grid";
import Actions from "./Actions";
import { ActionStateProvider } from "./ActionStateProvider";
import Header from "./Header";
import BuildTabs from "./tabs/Tabs";
import Updates from "./Updates";

const Build: Component<{}> = (p) => {
  const { builds, ws } = useAppState();
  const navigate = useNavigate();
  const params = useParams();
  const build = () => builds.get(params.id)!;
  const { isSemiMobile } = useAppDimensions();
  const { user } = useUser();
  const userCanUpdate = () =>
    user().admin ||
    build().permissions[getId(user())] === PermissionLevel.Update;
  let unsub = () => {};
  createEffect(() => {
    unsub();
    unsub = ws.subscribe([Operation.DeleteBuild], (update) => {
      if (update.target.id === params.id) {
        navigate("/");
      }
    });
    onCleanup(unsub);
  });
  return (
    <Show when={build()} fallback={<NotFound type="build" />}>
      <ActionStateProvider>
        <Grid class={combineClasses("content")}>
          {/* left / actions */}
          <Grid class="left-content">
            <Header />
            <Actions />
            <Show when={!isSemiMobile() && userCanUpdate()}>
              <Updates />
            </Show>
          </Grid>
          {/* right / tabs */}
          <BuildTabs />
        </Grid>
      </ActionStateProvider>
    </Show>
  );
};

export default Build;
