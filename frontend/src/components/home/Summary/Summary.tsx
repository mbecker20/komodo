import { ContainerStatus } from "@monitor/types";
import { Component, createMemo } from "solid-js";
import { useAppState } from "../../../state/StateProvider";
import { useTheme } from "../../../state/ThemeProvider";
import { combineClasses } from "../../../util/helpers";
import Grid from "../../util/layout/Grid";
import s from "../home.module.scss";

const Summary: Component<{}> = (p) => {
  const deployentCount = useDeploymentCount();
  const serverCount = useServerCount();
  const { builds } = useAppState();
	const { themeClass } = useTheme();
  return (
    <Grid class={combineClasses(s.Summary, "card shadow", themeClass())}>
			<h1>summary</h1>
      <div>
        servers: {serverCount().total} ({serverCount().healthy} healthy,{" "}
        {serverCount().unhealthy} unhealthy, {serverCount().disabled} disabled)
      </div>
      <div>
        deployments: {deployentCount().total} ({deployentCount().running}{" "}
        running, {deployentCount().stopped} stopped,{" "}
        {deployentCount().notDeployed} not deployed, {deployentCount().unknown}{" "}
        unknown)
      </div>
			<div>
				builds: {builds.ids()?.length}
			</div>
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
      const deployment = deployments.get(id)!;
      if (deployment.status === "not deployed") {
        notDeployed++;
      } else if (deployment.status === "unknown") {
        unknown++;
      } else if (deployment.status?.State === "running") {
        running++;
      } else if (deployment.status?.State === "exited") {
        stopped++;
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
      if (!server.enabled) {
        disabled++;
      } else if (server.status === "OK") {
        healthy++;
      } else if (server.status === "Could Not Be Reached") {
        unhealthy++;
      }
    }
    return { total: ids.length, healthy, unhealthy, disabled };
  });
  return count;
}
