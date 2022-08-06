import { Update as UpdateType } from "@monitor/types";
import { Component, Show } from "solid-js";
import { useAppState } from "../../../state/StateProvider";
import { useTheme } from "../../../state/ThemeProvider";
import { combineClasses, readableOperation, readableTimestamp } from "../../../util/helpers";
import { useToggle } from "../../../util/hooks";
import Icon from "../../util/Icon";
import Flex from "../../util/layout/Flex";
import Grid from "../../util/layout/Grid";
import CenterMenu from "../../util/menu/CenterMenu";
import s from "./update.module.scss";

const Update: Component<{ update: UpdateType }> = (p) => {
  const { deployments, servers, builds } = useAppState();
  const { themeClass } = useTheme();
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
    return readableOperation(p.update.operation);
  };
  const [showLog, toggleShowLog] = useToggle();
  return (
    <Flex
      class={combineClasses(s.Update, "shadow", themeClass())}
      justifyContent="space-between"
      alignItems="center"
    >
      <Grid gap="0.5rem" placeItems="center start">
        <h2>{name()}</h2>
        <div
          style={{
            color: p.update.isError ? "rgb(182, 47, 52)" : "inherit",
          }}
        >
          {operation()}
        </div>
      </Grid>
      <Flex>
        <Grid gap="0.5rem">
          <div style={{ "place-self": "center end" }}>
            {readableTimestamp(p.update.timestamp)}
          </div>
          <Flex gap="0.5rem">
            <Icon type="user" />
            <div>{p.update.operator}</div>
          </Flex>
        </Grid>
        <CenterMenu
          title={readableOperation(p.update.operation)}
          show={showLog}
          toggleShow={toggleShowLog}
          target={<Icon type="console" />}
          targetStyle={{ "place-self": "center end" }}
          targetClass="blue"
          content={
            <Grid class={s.LogContainer} gap="0.25rem">
              <Show when={p.update.note}>
                <pre>note: {p.update.note}</pre>
              </Show>
              <div>command</div>
              <pre class={combineClasses(s.Log, "scroller", themeClass())}>
                {p.update.command}
              </pre>
              <Show when={p.update.log.stdout}>
                <div>stdout</div>
                <pre
                  class={combineClasses(s.Log, "scroller", themeClass())}
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
                  class={combineClasses(s.Log, "scroller", themeClass())}
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
      </Flex>
    </Flex>
  );
};

export default Update;
