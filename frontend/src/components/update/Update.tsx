import { Component, For, Show } from "solid-js";
import { useAppState } from "../../state/StateProvider";
import { Update } from "../../types";
import {
  combineClasses,
  readableMonitorTimestamp,
} from "../../util/helpers";
import { useToggle } from "../../util/hooks";
import Icon from "../shared/Icon";
import Flex from "../shared/layout/Flex";
import Grid from "../shared/layout/Grid";
import CenterMenu from "../shared/menu/CenterMenu";
import s from "./update.module.scss";

const Update: Component<{ update: Update; showName: boolean }> = (p) => {
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
    return p.update.operation.replaceAll("_", " ")
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
            color: !p.update.success ? "rgb(182, 47, 52)" : "inherit",
          }}
        >
          {operation()}
        </div>
        <div style={{ "place-self": "center end" }}>
          {readableMonitorTimestamp(p.update.start_ts)}
        </div>
        <Flex alignItems="center">
          <Icon type="user" />
          <div>{p.update.operator}</div>
        </Flex>
        <CenterMenu
          title={operation()}
          show={showLog}
          toggleShow={toggleShowLog}
          target={<Icon type="console" />}
          targetStyle={{ "place-self": "center end" }}
          targetClass="blue"
          content={
            <Grid
              class={s.LogContainer}
              gap="0.25rem"
            >
              <For each={p.update.logs}>
                {(log, index) => {
                  return (
                    <>
                      <div>stage: {log.stage}</div>
                      <div>command</div>
                      <pre class={combineClasses(s.Log, "scroller")}>
                        {log.command}
                      </pre>
                      <Show when={log.stdout}>
                        <div>stdout</div>
                        <pre
                          class={combineClasses(s.Log, "scroller")}
                          style={{
                            "max-height": log.stderr ? "30vh" : "60vh",
                          }}
                        >
                          {log.stdout}
                        </pre>
                      </Show>
                      <Show when={log.stderr}>
                        <div>stderr</div>
                        <pre
                          class={combineClasses(s.Log, "scroller")}
                          style={{
                            "max-height": log.stdout ? "30vh" : "60vh",
                          }}
                        >
                          {log.stderr}
                        </pre>
                      </Show>
                    </>
                  );
                }}
              </For>
            </Grid>
          }
        />
      </Grid>
    </Grid>
  );
};

export default Update;
