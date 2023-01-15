import { Component, createMemo, Show } from "solid-js";
import { useAppState } from "../../state/StateProvider";
import { DockerContainerState, ServerStatus } from "../../types";
import { combineClasses } from "../../util/helpers";
import Grid from "../shared/layout/Grid";
import s from "./home.module.scss";
import Flex from "../shared/layout/Flex";

const Summary: Component<{}> = (p) => {
  const { builds } = useAppState();
  return (
    <Flex
      justifyContent="space-between"
      class={combineClasses(s.Summary, "card shadow wrap")}
      style={{ width: "100%", "box-sizing": "border-box" }}
    >
      <h1>summary</h1>
      <Flex gap="1rem" justifyContent="flex-end" class="wrap">
        <ServersSummary />
        <DeploymentsSummary />
        <BuildsSummary />
      </Flex>
    </Flex>
  );
};

export default Summary;

const BuildsSummary = () => {
  const { builds } = useAppState();
  return (
    <Grid
      placeItems="start center"
      class={combineClasses(s.SummaryItem, "shadow")}
      gap="0.5rem"
    >
      <h2>builds</h2>
      <Grid gap="0.5rem" style={{ width: "100%", height: "100%" }}>
        <Flex
          gap="0.4rem"
          justifyContent="space-between"
          style={{ width: "100%" }}
        >
          <div>total</div>
          <h2 class="text-green">{builds.ids()?.length}</h2>
        </Flex>
      </Grid>
    </Grid>
  );
};

const DeploymentsSummary = () => {
  const deployentCount = useDeploymentCount();
  return (
    <Grid
      placeItems="start center"
      class={combineClasses(s.SummaryItem, "shadow")}
      gap="0.5rem"
    >
      <h2>deployments</h2>
      <Grid gap="0.5rem" style={{ width: "100%", height: "100%" }}>
        <Flex
          gap="0.4rem"
          justifyContent="space-between"
          style={{ width: "100%" }}
        >
          <div>total</div>
          <h2 class="text-green">{deployentCount().total}</h2>
        </Flex>
        <Flex
          gap="0.4rem"
          justifyContent="space-between"
          style={{ width: "100%" }}
        >
          <div>running</div>
          <h2 class="text-green">{deployentCount().running}</h2>
        </Flex>
        <Show when={deployentCount().stopped > 0}>
          <Flex
            gap="0.4rem"
            justifyContent="space-between"
            style={{ width: "100%" }}
          >
            <div>stopped</div>
            <h2 class="text-red">{deployentCount().stopped}</h2>
          </Flex>
        </Show>
        <Show when={deployentCount().notDeployed > 0}>
          <Flex
            gap="0.4rem"
            justifyContent="space-between"
            style={{ width: "100%" }}
          >
            <div>not deployed</div>
            <h2 class="text-blue">{deployentCount().notDeployed}</h2>
          </Flex>
        </Show>
        <Show when={deployentCount().unknown > 0}>
          <Flex
            gap="0.4rem"
            justifyContent="space-between"
            style={{ width: "100%" }}
          >
            <div>unknown</div>
            <h2 class="text-orange">{deployentCount().unknown}</h2>
          </Flex>
        </Show>
      </Grid>
    </Grid>
  );
};

const ServersSummary = () => {
  const serverCount = useServerCount();
  return (
    <Grid
      placeItems="start center"
      class={combineClasses(s.SummaryItem, "shadow")}
      gap="0.5rem"
    >
      <h2>servers</h2>
      <Grid gap="0.5rem" style={{ width: "100%", height: "100%" }}>
        <Flex
          gap="0.4rem"
          justifyContent="space-between"
          style={{ width: "100%" }}
        >
          <div>total</div>
          <h2 class="text-green">{serverCount().total}</h2>
        </Flex>
        <Flex
          gap="0.4rem"
          justifyContent="space-between"
          style={{ width: "100%" }}
        >
          <div>healthy</div>
          <h2 class="text-green">{serverCount().healthy}</h2>
        </Flex>
        <Show when={serverCount().unhealthy > 0}>
          <Flex
            gap="0.4rem"
            justifyContent="space-between"
            style={{ width: "100%" }}
          >
            <div>unhealthy</div>
            <h2 class="text-red">{serverCount().unhealthy}</h2>
          </Flex>
        </Show>
        <Show when={serverCount().disabled > 0}>
          <Flex
            gap="0.4rem"
            justifyContent="space-between"
            style={{ width: "100%" }}
          >
            <div>disabled</div>
            <h2 class="text-blue">{serverCount().disabled}</h2>
          </Flex>
        </Show>
      </Grid>
    </Grid>
  );
};

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
        notDeployed++;
      } else if (state === DockerContainerState.Running) {
        running++;
      } else if (state === DockerContainerState.Exited) {
        stopped++;
      } else if (state === DockerContainerState.Unknown) {
        unknown++;
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
