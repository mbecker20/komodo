import { A } from "@solidjs/router";
import { Component, createResource, Show } from "solid-js";
import { client } from "../..";
import { useAppState } from "../../state/StateProvider";
import { DockerContainerState } from "../../types";
import {
  deploymentStateClass,
  readableVersion,
} from "../../util/helpers";
import Circle from "../shared/Circle";
import Flex from "../shared/layout/Flex";
import Grid from "../shared/layout/Grid";

const Deployment: Component<{ id: string }> = (p) => {
  const { deployments, builds } = useAppState();
  const deployment = () => deployments.get(p.id)!;
  const [deployed_version] = createResource(() =>
    client.get_deployment_deployed_version(p.id)
  );
  const derived_image = () => {
    if (deployment().deployment.build_id) {
      const build = builds.get(deployment().deployment.build_id!);
      if (build === undefined) return "unknown";
      const version =
        deployment().state === DockerContainerState.NotDeployed
          ? deployment().deployment.build_version
            ? readableVersion(
                deployment().deployment.build_version!
              ).replaceAll("v", "")
            : "latest"
          : deployed_version() || "unknown";
      return `${build.name}:${version}`;
    } else {
      return deployment().deployment.docker_run_args.image || "unknown";
    }
  };
  const image = () => {
    if (deployment().state === DockerContainerState.NotDeployed) {
      derived_image();
    } else if (deployment().container?.image) {
      if (deployment().container!.image.includes("sha256:")) {
        derived_image();
      }
      let [account, image] = deployment().container!.image.split("/");
      return image ? image : account;
    } else {
      return "unknown";
    }
  };
  return (
    <Show when={deployment()}>
      <A
        href={`/deployment/${p.id}`}
        class="card hoverable"
        style={{
          width: "100%",
          "justify-content": "space-between",
          padding: "0.5rem",
        }}
      >
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
