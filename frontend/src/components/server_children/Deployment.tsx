import { A } from "@solidjs/router";
import { Component, Show } from "solid-js";
import { useAppState } from "../../state/StateProvider";
import { DockerContainerState } from "../../types";
import {
  combineClasses,
  deploymentStateClass,
  readableVersion,
} from "../../util/helpers";
import Circle from "../shared/Circle";
import Flex from "../shared/layout/Flex";
import Grid from "../shared/layout/Grid";
import s from "./serverchildren.module.scss";

const Deployment: Component<{ id: string }> = (p) => {
  const { deployments, builds } = useAppState();
  const deployment = () => deployments.get(p.id)!;
  const image = () => {
    if (deployment().state === DockerContainerState.NotDeployed) {
      if (deployment().deployment.build_id) {
        const build = builds.get(deployment().deployment.build_id!);
        if (build === undefined) return "unknown"
        const version = deployment().deployment.build_version
          ? readableVersion(deployment().deployment.build_version!).replaceAll(
              "v",
              ""
            )
          : "latest";
        return `${build.name}:${version}`;
      } else {
        return deployment().deployment.docker_run_args.image || "unknown";
      }
    } else if (deployment().container?.image) {
      let [account, image] = deployment().container!.image.split("/");
      return image ? image : account;
    } else {
      return "unknown";
    }
  };
  return (
    <Show when={deployment()}>
      <A href={`/deployment/${p.id}`} class="card hoverable" style={{ width: "100%", "justify-content": "space-between", padding: "0.5rem" }}>
        <Grid gap="0">
          <h2>{deployment().deployment.name}</h2>
          <div style={{ opacity: 0.7 }}>{image()}</div>
        </Grid>
        <Flex alignItems="center">
          <div style={{ opacity: 0.7 }}>{deployments.status(p.id)}</div>
          <Circle
            size={1}
            class={deploymentStateClass(deployments.state(p.id))}
          />
        </Flex>
      </A>
    </Show>
  );
};

export default Deployment;
