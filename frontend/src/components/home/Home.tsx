import { ContainerStatus } from "@monitor/types";
import { Component, For, Show } from "solid-js";
import { useAppState } from "../../state/StateProvider";
import { useUser } from "../../state/UserProvider";
import { deploymentStatusClass, serverStatusClass } from "../../util/helpers";
import Flex from "../util/layout/Flex";
import Grid from "../util/layout/Grid";
import s from "./home.module.scss";

const Home: Component<{}> = (p) => {
  const { username, permissions } = useUser();
  const { deployments, builds, servers, selected } = useAppState();
  const filteredDeploymentIds = () =>
    deployments
      .ids()
      ?.filter(
        (id) =>
          permissions() > 1 || deployments.get(id)!.owners.includes(username()!)
      );

  const filteredBuildIds = () =>
    builds
      .ids()
      ?.filter(
        (id) =>
          permissions() > 1 || builds.get(id)!.owners.includes(username()!)
      );

  const filteredServerIds = () =>
    servers
      .ids()
      ?.filter(
        (id) =>
          permissions() > 1 || servers.get(id)!.owners.includes(username()!)
      );
  const deploymentState = (id: string) =>
    deployments.get(id)!.status === "not deployed"
      ? "not deployed"
      : (deployments.get(id)!.status as ContainerStatus).State;
  const deploymentStatus = (id: string) =>
    deployments.get(id)!.status === "not deployed"
      ? undefined
      : (deployments.get(id)!.status as ContainerStatus).Status.toLowerCase();
  return (
    <Grid class={s.Home}>
      <Grid style={{ height: "fit-content", width: "fit-content" }}>
        <Show
          when={filteredDeploymentIds() && filteredDeploymentIds()!.length > 0}
        >
          <Grid gap="0.5rem" class="card shadow">
            <h1 style={{ opacity: 0.5 }}>my deployments</h1>
            <For each={filteredDeploymentIds()}>
              {(id) => (
                <button
                  class="grey"
                  onClick={() => selected.set(id, "deployment")}
                  style={{ "justify-content": "space-between", width: "22rem" }}
                >
                  <h2>{deployments.get(id)!.name}</h2>
                  <Flex>
                    <div class={deploymentStatusClass(deploymentState(id))}>
                      {deploymentState(id)}
                    </div>
                    <div style={{ opacity: 0.7 }}>{deploymentStatus(id)}</div>
                  </Flex>
                </button>
              )}
            </For>
          </Grid>
        </Show>

        <Show when={filteredServerIds() && filteredServerIds()!.length > 0}>
          <Grid class="card shadow">
            <h1 style={{ opacity: 0.5 }}>my servers</h1>
            <For each={filteredServerIds()}>
              {(id) => (
                <button
                  class="grey"
                  onClick={() => selected.set(id, "server")}
                  style={{ "justify-content": "space-between", width: "22rem" }}
                >
                  <h2>{servers.get(id)!.name}</h2>
                  <div
                    class={serverStatusClass(
                      servers.get(id)!.enabled
                        ? servers.get(id)!.status === "OK"
                          ? "OK"
                          : "NOT OK"
                        : "DISABLED"
                    )}
                  >
                    {servers.get(id)!.enabled
                      ? servers.get(id)!.status === "OK"
                        ? "OK"
                        : "NOT OK"
                      : "DISABLED"}
                  </div>
                </button>
              )}
            </For>
          </Grid>
        </Show>

        <Show when={filteredBuildIds() && filteredBuildIds()!.length > 0}>
          <Grid class="card shadow">
            <h1 style={{ opacity: 0.5 }}>my builds</h1>
            <For each={filteredBuildIds()}>
              {(id) => (
                <button
                  class="grey"
                  onClick={() => selected.set(id, "build")}
                  style={{ "justify-content": "space-between", width: "22rem" }}
                >
                  <h2>{builds.get(id)!.name}</h2>
                </button>
              )}
            </For>
          </Grid>
        </Show>

        <Show when={permissions() < 1}>
          <div>you are using a view only account.</div>
        </Show>
      </Grid>
    </Grid>
  );
};

export default Home;
