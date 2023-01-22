import { A } from "@solidjs/router";
import { Component, Show } from "solid-js";
import { useAppState } from "../../../state/StateProvider";
import { Operation, Update as UpdateType, UpdateStatus } from "../../../types";
import {
  combineClasses,
  readableMonitorTimestamp,
  readableVersion,
} from "../../../util/helpers";
import Icon from "../../shared/Icon";
import Flex from "../../shared/layout/Flex";
import Grid from "../../shared/layout/Grid";
import UpdateMenu from "../../update/UpdateMenu";
import s from "./update.module.scss";

const Update: Component<{ update: UpdateType }> = (p) => {
  const { deployments, servers, builds, usernames } = useAppState();
  const name = () => {
    if (p.update.target.type === "Deployment" && deployments.loaded()) {
      return deployments.get(p.update.target.id!)?.deployment.name || "deleted";
    } else if (p.update.target.type === "Server" && servers.loaded()) {
      return servers.get(p.update.target.id)?.server.name || "deleted";
    } else if (p.update.target.type === "Build" && builds.loaded()) {
      return builds.get(p.update.target.id)?.name || "deleted";
    } else {
      return "monitor";
    }
  };
  const operation = () => {
    if (p.update.operation === Operation.BuildBuild) {
      return `build ${readableVersion(p.update.version!)}`;
    }
    return `${p.update.operation.replaceAll("_", " ")}${
      p.update.version ? " " + readableVersion(p.update.version) : ""
    }`;
  };
  const link_to = () => {
    return p.update.target.type === "System"
      ? "/"
      : `/${p.update.target.type.toLowerCase()}/${p.update.target.id}`;
  };
  return (
    <Flex
      class={combineClasses(s.Update, "shadow")}
      justifyContent="space-between"
      alignItems="center"
    >
      <Grid gap="0.5rem" placeItems="center start">
        <A style={{ padding: 0 }} href={link_to()}>
          <h2 class="text-hover">{name()}</h2>
        </A>
        <Flex gap="0.5rem">
          <div
            style={{
              color: !p.update.success ? "rgb(182, 47, 52)" : "inherit",
            }}
          >
            {operation()}
          </div>
          <Show when={p.update.status === UpdateStatus.InProgress}>
            <div style={{ opacity: 0.7 }}>(in progress)</div>
          </Show>
        </Flex>
      </Grid>
      <Flex>
        <Grid gap="0.5rem">
          <div style={{ "place-self": "center end" }}>
            {readableMonitorTimestamp(p.update.start_ts)}
          </div>
          <Flex gap="0.5rem">
            <Icon type="user" />
            <div>{usernames.get(p.update.operator)}</div>
          </Flex>
        </Grid>
        <UpdateMenu update={p.update} />
      </Flex>
    </Flex>
  );
};

export default Update;
