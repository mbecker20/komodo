import { Component, createResource, For, Show } from "solid-js";
import { client } from "../..";
import { useAppState } from "../../state/StateProvider";
import { Update as UpdateType } from "../../types";
import { combineClasses, readableDuration, readableMonitorTimestamp } from "../../util/helpers";
import { useToggle } from "../../util/hooks";
import Icon from "../shared/Icon";
import Flex from "../shared/layout/Flex";
import Grid from "../shared/layout/Grid";
import CenterMenu from "../shared/menu/CenterMenu";
import s from "./update.module.scss";

const UpdateMenu: Component<{ update: UpdateType }> = (p) => {
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
    <CenterMenu
      title={`${operation()} | ${name()}`}
      show={showLog}
      toggleShow={toggleShowLog}
      target={<Icon type="console" />}
      targetStyle={{ "place-self": "center end" }}
      targetClass="blue"
      padding="1rem 2rem"
      content={
        () => <UpdateMenuContent update={p.update} />
      }
    />
  );
};

export default UpdateMenu;

const UpdateMenuContent: Component<{ update: UpdateType }> = (p) => {
  const { usernames } = useAppState();
  return (
    <Grid class={s.LogContainer} gap="1rem">
      <Grid gap="0.5rem" class="card light shadow">
        <div>operator: {usernames.get(p.update.operator)}</div>
        <div>started at: {readableMonitorTimestamp(p.update.start_ts)}</div>
        <Show when={p.update.end_ts}>
          <div>
            duration: {readableDuration(p.update.start_ts, p.update.end_ts!)}
          </div>
        </Show>
      </Grid>
      <For each={p.update.logs}>
        {(log, index) => {
          return (
            <Grid gap="0.5rem" class="card light shadow">
              <Flex alignItems="center" class="wrap">
                <h1>{log.stage}</h1>
                <div style={{ opacity: 0.7 }}>
                  (stage {index() + 1} of {p.update.logs.length})
                </div>
                <div style={{ opacity: 0.7 }}>
                  {readableDuration(log.start_ts, log.end_ts)}
                </div>
              </Flex>
              <Show when={log.command}>
                <div>command</div>
                <pre class={combineClasses(s.Log)}>{log.command}</pre>
              </Show>
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
  );
}