import { Accessor, Component, createMemo, For, Show } from "solid-js";
import { useAppState } from "../../state/StateProvider";
import { DockerContainerState, ServerStatus } from "../../types";
import Grid from "../shared/layout/Grid";
import Flex from "../shared/layout/Flex";
import PieChart, { PieChartSection } from "../shared/PieChart";
import { COLORS } from "../../style/colors";
import { useAppDimensions } from "../../state/DimensionProvider";

const PIE_CHART_HEIGHT = 250;

const Summary: Component<{}> = (p) => {
  const { isMobile } = useAppDimensions();
  const deployentCount = useDeploymentCount();
  const serverCount = useServerCount();
  return (
    <Grid
      class="card shadow"
      gridTemplateColumns={isMobile() ? "1fr" : "1fr 1fr"}
      style={{ width: "100%", height: "100%", "box-sizing": "border-box", padding: "0.5rem" }}
      placeItems="center"
      gap="0"
    >
      <div style={{ width: "100%", height: `${PIE_CHART_HEIGHT}px` }}>
        <PieChart title="deployments" sections={deployentCount()} />
      </div>
      <div style={{ width: "100%", height: `${PIE_CHART_HEIGHT}px` }}>
        <PieChart title="servers" sections={serverCount()} />
      </div>
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

function useDeploymentCount(): Accessor<PieChartSection[]> {
  const { deployments } = useAppState();
  const count = createMemo(() => {
    const ids = deployments.ids();
    if (!ids)
      return [
        { title: "running", amount: 0, color: COLORS.textgreen },
        { title: "stopped", amount: 0, color: COLORS.textred },
        { title: "not deployed", amount: 0, color: COLORS.textblue },
        { title: "unknown", amount: 0, color: COLORS.textorange },
      ];
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
    return [
      { title: "running", amount: running, color: COLORS.textgreen },
      { title: "stopped", amount: stopped, color: COLORS.textred },
      { title: "not deployed", amount: notDeployed, color: COLORS.textblue },
      { title: "unknown", amount: unknown, color: COLORS.textorange },
    ];
  });
  return count;
}

function useServerCount(): Accessor<PieChartSection[]> {
  const { servers } = useAppState();
  const count = createMemo(() => {
    const ids = servers.ids();
    if (!ids) return [
      { title: "healthy", amount: 0, color: COLORS.textgreen },
      { title: "unhealthy", amount: 0, color: COLORS.textred },
      { title: "disabled", amount: 0, color: COLORS.textblue },
    ];
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
    return [
      { title: "healthy", amount: healthy, color: COLORS.textgreen },
      { title: "unhealthy", amount: unhealthy, color: COLORS.textred },
      { title: "disabled", amount: disabled, color: COLORS.textblue },
    ];
  });
  return count;
}
