import { useNavigate, useParams } from "@solidjs/router";
import { Component, createEffect, onCleanup, Show } from "solid-js";
import { useAppDimensions } from "../../state/DimensionProvider";
import { useAppState } from "../../state/StateProvider";
import { useUser } from "../../state/UserProvider";
import { Operation, PermissionLevel } from "../../types";
import Description from "../Description";
import NotFound from "../NotFound";
import Grid from "../shared/layout/Grid";
import Actions from "./Actions";
import { ActionStateProvider } from "./ActionStateProvider";
import Header from "./Header";
import BuildTabs from "./tabs/Tabs";
import Updates from "./Updates";

const Build: Component<{}> = (p) => {
  const { user, user_id } = useUser();
  const { builds, ws } = useAppState();
  const navigate = useNavigate();
  const params = useParams();
  const build = () => builds.get(params.id)!;
  const { isSemiMobile } = useAppDimensions();
  // const { user } = useUser();
  // const userCanUpdate = () =>
  //   user().admin ||
  //   build().permissions![getId(user())] === PermissionLevel.Update;
  let unsub = () => {};
  createEffect(() => {
    unsub();
    unsub = ws.subscribe([Operation.DeleteBuild], (update) => {
      if (update.target.id === params.id) {
        navigate("/");
      }
    });
  });
  onCleanup(() => unsub());
  const userCanUpdate = () =>
    user().admin ||
    build()?.permissions![user_id()] === PermissionLevel.Update;
  return (
    <Show
      when={build()}
      fallback={<NotFound type="build" loaded={builds.loaded()} />}
    >
      <ActionStateProvider build_id={params.id}>
        <Grid
          style={{
            width: "100%",
            "box-sizing": "border-box",
          }}
        >
          <Header />
          <Grid
            style={{ width: "100%" }}
            gridTemplateColumns={isSemiMobile() ? "1fr" : "1fr 1fr"}
          >
            <Grid gridTemplateRows="auto 1fr" style={{ "flex-grow": 1 }}>
              <Description
                target={{ type: "Build", id: params.id }}
                name={build()?.name!}
                description={build()?.description}
                userCanUpdate={userCanUpdate()}
              />
              <Actions />
            </Grid>
            <Show when={!isSemiMobile()}>
              <Updates />
            </Show>
          </Grid>
          <BuildTabs />
        </Grid>
      </ActionStateProvider>
    </Show>
  );
};

export default Build;
