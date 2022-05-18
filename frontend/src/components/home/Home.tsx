import { Component, For, Show } from "solid-js";
import { useAppState } from "../../state/StateProvider";
import { useTheme } from "../../state/ThemeProvider";
import { useUser } from "../../state/UserProvider";
import {
  combineClasses,
  deploymentStatusClass,
  inPx,
  serverStatusClass,
} from "../../util/helpers";
import Button from "../util/Button";
import Circle from "../util/Circle";
import Flex from "../util/layout/Flex";
import Grid from "../util/layout/Grid";
import s from "./home.module.scss";

const cardWidth = 300; // in px

const Home: Component<{}> = (p) => {
  const { username, permissions } = useUser();
  const { deployments, builds, servers, selected } = useAppState();
  const { themeClass } = useTheme();
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
  return (
    <Grid class={s.Home}>
      <Flex class={s.Container}>
        <Show
          when={filteredDeploymentIds() && filteredDeploymentIds()!.length > 0}
        >
          <Grid
            gap="0.5rem"
            class={combineClasses("card shadow", themeClass())}
            style={{ height: "fit-content" }}
          >
            <h1 style={{ opacity: 0.5 }}>my deployments</h1>
            <For each={filteredDeploymentIds()}>
              {(id) => (
                <Button
                  class="grey"
                  onClick={() => selected.set(id, "deployment")}
                  style={{
                    "justify-content": "space-between",
                    width: inPx(cardWidth),
                  }}
                >
                  <h2>{deployments.get(id)!.name}</h2>
                  <Flex>
                    <div style={{ opacity: 0.7 }}>{deployments.status(id)}</div>
                    <Circle
                      size={1}
                      class={deploymentStatusClass(deployments.state(id))}
                    />
                  </Flex>
                </Button>
              )}
            </For>
          </Grid>
        </Show>

        <Show when={filteredServerIds() && filteredServerIds()!.length > 0}>
          <Grid
            gap="0.5rem"
            class={combineClasses("card shadow", themeClass())}
            style={{ height: "fit-content" }}
          >
            <h1 style={{ opacity: 0.5 }}>my servers</h1>
            <For each={filteredServerIds()}>
              {(id) => (
                <Button
                  class="grey"
                  onClick={() => selected.set(id, "server")}
                  style={{
                    "justify-content": "space-between",
                    width: inPx(cardWidth),
                  }}
                >
                  <h2>{servers.get(id)!.name}</h2>
                  <Flex alignItems="center">
                    <Show when={servers.get(id)!.region}>
                      <div style={{ opacity: 0.7 }}>{servers.get(id)!.region}</div>
                    </Show>
                    <div
                      class={serverStatusClass(
                        servers.get(id)!.enabled
                          ? servers.get(id)!.status === "OK"
                            ? "OK"
                            : "NOT OK"
                          : "DISABLED",
                        themeClass
                      )}
                    >
                      {servers.get(id)!.enabled
                        ? servers.get(id)!.status === "OK"
                          ? "OK"
                          : "NOT OK"
                        : "DISABLED"}
                    </div>
                  </Flex>
                </Button>
              )}
            </For>
          </Grid>
        </Show>

        <Show when={filteredBuildIds() && filteredBuildIds()!.length > 0}>
          <Grid
            gap="0.5rem"
            class={combineClasses("card shadow", themeClass())}
            style={{ height: "fit-content" }}
          >
            <h1 style={{ opacity: 0.5 }}>my builds</h1>
            <For each={filteredBuildIds()}>
              {(id) => (
                <Button
                  class="grey"
                  onClick={() => selected.set(id, "build")}
                  style={{
                    "justify-content": "space-between",
                    width: inPx(cardWidth),
                  }}
                >
                  <h2>{builds.get(id)!.name}</h2>
                </Button>
              )}
            </For>
          </Grid>
        </Show>

        <Show when={permissions() < 1}>
          <div>you are using a view only account.</div>
        </Show>
      </Flex>
    </Grid>
  );
};

export default Home;
