import { Component, createMemo, For, Show } from "solid-js";
import { useAppDimensions } from "../../state/DimensionProvider";
import { combineClasses, getId } from "../../util/helpers";
import Grid from "../shared/layout/Grid";
import SimpleTabs from "../shared/tabs/SimpleTabs";
import s from "./serverchildren.module.scss";
import { useUser } from "../../state/UserProvider";
import { PermissionLevel } from "../../types";
import { NewBuild, NewDeployment } from "../New";
import Deployment from "./Deployment";
import Build from "./Build";
import { useAppState } from "../../state/StateProvider";

const ServerChildren: Component<{ id: string }> = (p) => {
  const { user } = useUser();
  const { isSemiMobile } = useAppDimensions();
  const { servers, deployments, builds } = useAppState();
  const server = () => servers.get(p.id);
  const deploymentIDs = createMemo(() => {
    return (deployments.loaded() &&
      deployments
        .ids()!
        .filter(
          (id) => deployments.get(id)?.deployment.server_id === p.id
        )) as string[];
  });
  const buildIDs = createMemo(() => {
    return (builds.loaded() &&
      builds
        .ids()!
        .filter((id) => builds.get(id)?.server_id === p.id)) as string[];
  });
  return (
    <SimpleTabs
      containerClass="card shadow"
      localStorageKey={`${p.id}-home-tab`}
      tabs={[
        {
          title: "deployments",
          element: () => (
            <Grid
              gap=".5rem"
              class={combineClasses(s.Deployments)}
              gridTemplateColumns={isSemiMobile() ? "1fr" : "1fr 1fr"}
            >
              <For each={deploymentIDs()}>{(id) => <Deployment id={id} />}</For>
              <Show
                when={
                  user().admin ||
                  server()?.server.permissions![getId(user())] ===
                    PermissionLevel.Update
                }
              >
                <NewDeployment serverID={p.id} />
              </Show>
            </Grid>
          ),
        },
        {
          title: "builds",
          element: () => (
            <Grid
              gap=".5rem"
              class={combineClasses(s.Deployments)}
              gridTemplateColumns={isSemiMobile() ? "1fr" : "1fr 1fr"}
            >
              <For each={buildIDs()}>{(id) => <Build id={id} />}</For>
              <Show
                when={
                  user().admin ||
                  server()?.server.permissions![getId(user())] ===
                    PermissionLevel.Update
                }
              >
                <NewBuild serverID={p.id} />
              </Show>
            </Grid>
          ),
        },
      ]}
    />
  );
};

export default ServerChildren;
