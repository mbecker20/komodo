import { Component, createMemo } from "solid-js";
import { useAppState } from "../../state/StateProvider";
import { DockerContainerState, ServerStatus } from "../../types";

const Summary: Component<{}> = (p) => {
  return <div></div>;
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
