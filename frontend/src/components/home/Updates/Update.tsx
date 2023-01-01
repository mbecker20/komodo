import { Component, For, Show } from "solid-js";
import { useAppState } from "../../../state/StateProvider";
import { Update as UpdateType } from "../../../types";
import {
  combineClasses,
  readableMonitorTimestamp,
} from "../../../util/helpers";
import { useToggle } from "../../../util/hooks";
import Icon from "../../shared/Icon";
import Flex from "../../shared/layout/Flex";
import Grid from "../../shared/layout/Grid";
import CenterMenu from "../../shared/menu/CenterMenu";
import s from "./update.module.scss";

const Update: Component<{ update: UpdateType }> = (p) => {
  const { deployments, servers, builds } = useAppState();
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
    return p.update.operation.replaceAll("_", " ");
  };
  const [showLog, toggleShowLog] = useToggle();
  return (
    <Flex
      class={combineClasses(s.Update, "shadow")}
      justifyContent="space-between"
      alignItems="center"
    >
      <Grid gap="0.5rem" placeItems="center start">
        <h2>{name()}</h2>
        <div
          style={{
            color: !p.update.success ? "rgb(182, 47, 52)" : "inherit",
          }}
        >
          {operation()}
        </div>
      </Grid>
      <Flex>
        <Grid gap="0.5rem">
          <div style={{ "place-self": "center end" }}>
            {readableMonitorTimestamp(p.update.start_ts)}
          </div>
          <Flex gap="0.5rem">
            <Icon type="user" />
            <div>{p.update.operator}</div>
          </Flex>
        </Grid>
        <CenterMenu
          title={operation()}
          show={showLog}
          toggleShow={toggleShowLog}
          target={<Icon type="console" />}
          targetStyle={{ "place-self": "center end" }}
          targetClass="blue"
          padding="1rem 2rem"
          content={
            <Grid class={s.LogContainer} gap="1rem">
              <For each={p.update.logs}>
                {(log, index) => {
                  return (
                    <Grid gap="0.5rem" class="card lightgrey shadow">
                      <Flex alignItems="center">
                        <h1>{log.stage}</h1>
                        <div style={{ opacity: 0.7 }}>
                          (stage {index() + 1} of {p.update.logs.length})
                        </div>
                      </Flex>
                      <div>command</div>
                      <pre class={combineClasses(s.Log)}>{log.command}</pre>
                      <Show when={log.stdout}>
                        <div>stdout</div>
                        <pre
                          class={combineClasses(s.Log)}
                          // style={{
                          //   "max-height": log.stderr ? "30vh" : "60vh",
                          // }}
                        >
                          {log.stdout}
                        </pre>
                      </Show>
                      <Show when={log.stderr}>
                        <div>stderr</div>
                        <pre
                          class={combineClasses(s.Log)}
                          // style={{
                          //   "max-height": log.stdout ? "30vh" : "60vh",
                          // }}
                        >
                          {log.stderr}
                        </pre>
                      </Show>
                    </Grid>
                  );
                }}
              </For>
            </Grid>
          }
        />
      </Flex>
    </Flex>
  );
};

export default Update;
