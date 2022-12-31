import { Component, createMemo, Show } from "solid-js";
import { useAppState } from "../../state/StateProvider";
import { DockerContainerState, ServerStatus } from "../../types";
import { combineClasses } from "../../util/helpers";
import Grid from "../shared/layout/Grid";
import s from "./home.module.scss";
import Flex from "../shared/layout/Flex";

const Summary: Component<{}> = (p) => {
  const deployentCount = useDeploymentCount();
  const serverCount = useServerCount();
  const { builds } = useAppState();
  return (
    <Grid class={combineClasses(s.Summary, "card shadow")}>
      <h1>summary</h1>
      <Flex justifyContent="space-between">
        <Flex gap="0.2rem">
          deployments: <h2>{deployentCount().total}</h2>
        </Flex>
        <Flex gap="0.2rem">
          <h2 class="text-green">{deployentCount().running}</h2> running
          <Show when={deployentCount().stopped > 0}>
            {", "}
            <h2 class="text-red">{deployentCount().stopped}</h2> stopped
          </Show>
          <Show when={deployentCount().notDeployed > 0}>
            {", "}
            <h2 class="text-blue">{deployentCount().notDeployed}</h2> not
            deployed
          </Show>
          <Show when={deployentCount().unknown > 0}>
            {", "}
            <h2 class="text-orange">{deployentCount().unknown}</h2> unknown
          </Show>
        </Flex>
      </Flex>
      <Flex justifyContent="space-between">
        <Flex gap="0.2rem" alignItems="center">
          servers: <h2>{serverCount().total}</h2>
        </Flex>
        <Flex gap="0.2rem" alignItems="center">
          <h2 class="text-green">{serverCount().healthy}</h2> healthy
          <Show when={serverCount().unhealthy > 0}>
            {", "}
            <h2 class="text-red">{serverCount().unhealthy}</h2> unhealthy
          </Show>
          <Show when={serverCount().disabled > 0}>
            {", "}
            <h2 class="text-blue">{serverCount().disabled}</h2> disabled
          </Show>
        </Flex>
      </Flex>
      <div>builds: {builds.ids()?.length}</div>
    </Grid>
  );
};

export default Summary;

function useDeploymentCount() {
  const { deployments } = useAppState();
  const count = createMemo(() => {
    const ids = deployments.ids();
    if (!ids)
      return { total: 0, running: 0, stopped: 0, notDeployed: 0, unknown: 0 };
    let running = 0;
    let stopped = 0;
    let notDeployed = 0;
    let unknown = 0;
    for (const id of ids) {
      const state = deployments.get(id)!.state;
      if (state === DockerContainerState.NotDeployed) {
        notDeployed++
      } else if (state === DockerContainerState.Running) {
        running++
      } else if (state === DockerContainerState.Exited) {
        stopped++
      } else if (state === DockerContainerState.Unknown) {
        unknown++
      }
    }
    return { total: ids.length, running, stopped, notDeployed, unknown };
  });
  return count;
}

function useServerCount() {
  const { servers } = useAppState();
  const count = createMemo(() => {
    const ids = servers.ids();
    if (!ids) return { total: 0, healthy: 0, unhealthy: 0, disabled: 0 };
    let healthy = 0;
    let unhealthy = 0;
    let disabled = 0;
    for (const id of ids) {
      const server = servers.get(id)!;
      if (server.status === ServerStatus.Disabled) {
        disabled++;
      } else if (server.status === ServerStatus.Ok) {
        healthy++;
      } else if (server.status === ServerStatus.NotOk) {
        unhealthy++;
      }
    }
    return { total: ids.length, healthy, unhealthy, disabled };
  });
  return count;
}
