import { Component, For, Show } from "solid-js";
import { useAppState } from "../../state/StateProvider";
import { Operation, Update as UpdateType } from "../../types";
import {
  combineClasses,
  readableDuration,
  readableMonitorTimestamp,
  readableVersion,
} from "../../util/helpers";
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
    if (p.update.operation === Operation.BuildBuild) {
      return "build";
    }
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
      content={() => <UpdateMenuContent update={p.update} />}
    />
  );
};

export default UpdateMenu;

const UpdateMenuContent: Component<{ update: UpdateType }> = (p) => {
  return (
    <Grid class={combineClasses(s.LogContainer, "scroller")}>
      <UpdateSummary update={p.update} />
      <UpdateLogs update={p.update} />
    </Grid>
  );
};

const UpdateSummary: Component<{ update: UpdateType }> = (p) => {
  const { usernames } = useAppState();
  return (
    <Grid
      gap="0.5rem"
      class="card light shadow"
      gridTemplateColumns="1fr 1fr"
      style={{ width: "40rem", "max-width": "90vw" }}
    >
      <Flex gap="0.5rem" alignItems="center">
        status: <h2>{p.update.status.replaceAll("_", " ")}</h2>
      </Flex>
      <h2 style={{ "place-self": "center end" }}>
        {readableMonitorTimestamp(p.update.start_ts)}
      </h2>
      <Flex gap="0.5rem" alignItems="center">
        duration:
        <h2>
          {p.update.end_ts
            ? readableDuration(p.update.start_ts, p.update.end_ts)
            : "unknown"}
        </h2>
      </Flex>
      <Flex
        gap="0.5rem"
        alignItems="center"
        style={{ "place-self": "center end" }}
      >
        <Icon type="user" width="1.25rem" />
        <h2>{usernames.get(p.update.operator)}</h2>
      </Flex>
      <Show when={p.update.version}>
        <Flex gap="0.5rem" alignItems="center">
          version:
          <h2>{readableVersion(p.update.version!)}</h2>
        </Flex>
      </Show>
    </Grid>
  );
}

const UpdateLogs: Component<{ update: UpdateType }> = (p) => {
  return (
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
  );
};
