import { A } from "@solidjs/router";
import { Component, createResource, Show } from "solid-js";
import { client } from "../..";
import { useAppState } from "../../state/StateProvider";
import { DockerContainerState } from "../../types";
import {
  combineClasses,
  deploymentStateClass,
  getId,
  readableVersion,
} from "../../util/helpers";
import Circle from "../shared/Circle";
import Flex from "../shared/layout/Flex";
import Grid from "../shared/layout/Grid";
import s from "./serverchildren.module.scss";

const Deployment: Component<{ id: string }> = (p) => {
  const { deployments, builds } = useAppState();
  const deployment = () => deployments.get(p.id)!;
  const [deployed_version] = createResource(() =>
    client.get_deployment_deployed_version(p.id)
  );
  const image = () => {
    return deployment().container?.image || "unknown";
    // if (deployment().deployment.build_id) {
    //   const build = builds.get(deployment().deployment.build_id!);
    //   if (build === undefined) return "unknown"
    //   if (deployment().state === DockerContainerState.NotDeployed) {
    //     const version = deployment().deployment.build_version
    //       ? readableVersion(deployment().deployment.build_version!).replaceAll(
    //           "v",
    //           ""
    //         )
    //       : "latest";
    //     return `${build.name}:${version}`;
    //   } else {
    //     return deployed_version() && `${build.name}:${deployed_version()}`;
    //   }
    // } else {
    //   return deployment().deployment.docker_run_args.image || "unknown";
    // }
  };
  return (
    <Show when={deployment()}>
      <A href={`/deployment/${p.id}`} class={combineClasses(s.DropdownItem)}>
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
