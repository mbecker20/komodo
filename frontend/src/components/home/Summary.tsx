import { Component, createMemo, For, Show } from "solid-js";
import { useAppState } from "../../state/StateProvider";
import { DockerContainerState, ServerStatus } from "../../types";
import Grid from "../shared/layout/Grid";
import Flex from "../shared/layout/Flex";

const Summary: Component<{}> = (p) => {
  return (
    <Grid class="card shadow" gridTemplateRows="auto 1fr 1fr 1fr">
      <h1>summary</h1>
      <DeploymentsSummary />
      <ServersSummary />
      <BuildsSummary />
    </Grid>
  );
};

export default Summary;

const SummaryItem: Component<{
  title: string;
  metrics: Array<{ title: string; class: string; count?: number }>;
}> = (p) => {
  return (
    <Flex
      class="card light shadow wrap"
      justifyContent="space-between"
      alignItems="center"
    >
      <h2>{p.title}</h2>
      <Flex class="wrap">
        <For each={p.metrics}>
          {(metric) => (
            <Show when={metric?.count && metric.count > 0}>
              <Flex gap="0.4rem" alignItems="center">
                <div>{metric.title}</div>
                <h2 class={metric.class}>{metric.count}</h2>
              </Flex>
            </Show>
          )}
        </For>
      </Flex>
    </Flex>
  );
};

const BuildsSummary = () => {
  const { builds } = useAppState();
  return (
    <SummaryItem
      title="builds"
      metrics={[
        { title: "total", class: "text-green", count: builds.ids()?.length },
      ]}
    />
  );
};

const DeploymentsSummary = () => {
  const deployentCount = useDeploymentCount();
  return (
    <SummaryItem
      title="deployments"
      metrics={[
        {
          title: "total",
          class: "text-green",
          count: deployentCount().total,
        },
        {
          title: "running",
          class: "text-green",
          count: deployentCount().running,
        },
        {
          title: "stopped",
          class: "text-red",
          count: deployentCount().stopped,
        },
        {
          title: "not deployed",
          class: "text-blue",
          count: deployentCount().notDeployed,
        },
        {
          title: "unknown",
          class: "text-blue",
          count: deployentCount().unknown,
        },
      ]}
    />
  );
};

const ServersSummary = () => {
  const serverCount = useServerCount();
  return (
    <SummaryItem
      title="servers"
      metrics={[
        { title: "total", class: "text-green", count: serverCount().total },
        { title: "healthy", class: "text-green", count: serverCount().healthy },
        {
          title: "unhealthy",
          class: "text-red",
          count: serverCount().unhealthy,
        },
        {
          title: "disabled",
          class: "text-blue",
          count: serverCount().disabled,
        },
      ]}
    />
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
