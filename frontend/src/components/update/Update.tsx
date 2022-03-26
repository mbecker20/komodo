import { Update as UpdateType } from "@monitor/types";
import { Component, Show } from "solid-js";
import { useAppState } from "../../state/StateProvider";
import { combineClasses, readableOperation, readableTimestamp } from "../../util/helpers";
import Icon from "../util/icons/Icon";
import Flex from "../util/layout/Flex";
import Grid from "../util/layout/Grid";
import s from "./update.module.css";

const Update: Component<{ update: UpdateType; showName: boolean }> = (p) => {
  const { deployments, servers, builds } = useAppState();
  const name = () => {
    if (p.update.deploymentID && deployments.loaded()) {
      return deployments.get(p.update.deploymentID)?.name || "loading...";
    } else if (p.update.serverID && servers.loaded()) {
      return servers.get(p.update.serverID)?.name || "loading...";
    } else if (p.update.buildID && builds.loaded()) {
      return builds.get(p.update.buildID)?.name || "loading...";
    } else {
      return "Monitor System";
    }
  };
  const operation = () => {
    const op = readableOperation(p.update.operation);
    if (!p.showName) {
      if (p.update.deploymentID) {
        return op.replaceAll(" deployment", "");
      } else if (p.update.buildID) {
        return op.replaceAll(" build", "");
      } else if (p.update.serverID) {
        return op.replaceAll(" server", "");
      }
    } else {
      return op;
    }
  }
  return (
    <Grid gap="0.5rem" class={combineClasses(s.Update, "shadow")}>
      <Show when={p.showName}>
        <div>{name()}</div>
      </Show>
      <Grid
        gap="0.5rem"
        style={{
          "grid-template-columns": "1fr 1fr",
          "grid-template-rows": "1fr 1fr",
        }}
      >
        <div>{operation()}</div>
        <div style={{ "place-self": "center end" }}>
          {readableTimestamp(p.update.timestamp)}
        </div>
        <Flex alignItems="center">
          <Icon type="user" />
          <div>{p.update.operator}</div>
        </Flex>
        <Flex justifyContent="space-between" alignItems="center">
          {/* show command */}
          <Icon type="arrow-down" />
          <Show when={p.update.note}>
            <Icon type="arrow-down" />
          </Show>
          {/* show log */}
          <Icon type="arrow-down" />
        </Flex>
      </Grid>
    </Grid>
  );
};

export default Update;
