import { Update as UpdateType } from "@monitor/types";
import { Component, Show } from "solid-js";
import { useAppState } from "../../state/StateProvider";
import {
  combineClasses,
  readableOperation,
  readableTimestamp,
} from "../../util/helpers";
import { useToggle } from "../../util/hooks";
import Icon from "../util/Icon";
import Flex from "../util/layout/Flex";
import Grid from "../util/layout/Grid";
import CenterMenu from "../util/menu/CenterMenu";
import s from "./update.module.scss";

const Update: Component<{ update: UpdateType; showName: boolean }> = (p) => {
  const { deployments, servers, builds } = useAppState();
  const name = () => {
    if (p.update.deploymentID && deployments.loaded()) {
      return deployments.get(p.update.deploymentID)?.name || "deleted";
    } else if (p.update.serverID && servers.loaded()) {
      return servers.get(p.update.serverID)?.name || "deleted";
    } else if (p.update.buildID && builds.loaded()) {
      return builds.get(p.update.buildID)?.name || "deleted";
    } else {
      return "monitor";
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
  };
  const [showLog, toggleShowLog] = useToggle();
  return (
    <Grid
      gap="0.25rem"
      class={combineClasses(s.Update, !p.showName && s.NoName, "shadow")}
    >
      <Show when={p.showName}>
        <h2 style={{ "place-self": "center" }}>{name()}</h2>
      </Show>
      <Grid
        gap="0.25rem"
        style={{
          "grid-template-columns": "1fr 1fr",
          "grid-template-rows": "1fr 1fr",
        }}
        placeItems="center start"
      >
        <div
          style={{
            color: p.update.isError ? "rgb(182, 47, 52)" : "inherit",
          }}
        >
          {operation()}
        </div>
        <div style={{ "place-self": "center end" }}>
          {readableTimestamp(p.update.timestamp)}
        </div>
        <Flex alignItems="center">
          <Icon type="user" />
          <div>{p.update.operator}</div>
        </Flex>
        <CenterMenu
          title={readableOperation(p.update.operation)}
          show={showLog}
          toggleShow={toggleShowLog}
          target={<Icon type="console" />}
          targetStyle={{ "place-self": "center end" }}
          targetClass="blue"
          content={
            <Grid
              class={combineClasses(s.LogContainer, "scroller")}
              gap="0.25rem"
            >
              <Show when={p.update.note}>
                <pre>note: {p.update.note}</pre>
              </Show>
              <div>command</div>
              <pre class={combineClasses(s.Log, "scroller")}>
                {p.update.command}
              </pre>
              <Show when={p.update.log.stdout}>
                <div>stdout</div>
                <pre
                  class={combineClasses(s.Log, "scroller")}
                  style={{
                    "max-height": p.update.log.stderr ? "30vh" : "60vh",
                  }}
                >
                  {p.update.log.stdout}
                </pre>
              </Show>
              <Show when={p.update.log.stderr}>
                <div>stderr</div>
                <pre
                  class={combineClasses(s.Log, "scroller")}
                  style={{
                    "max-height": p.update.log.stdout ? "30vh" : "60vh",
                  }}
                >
                  {p.update.log.stderr}
                </pre>
              </Show>
            </Grid>
          }
        />
      </Grid>
    </Grid>
  );
};

export default Update;
