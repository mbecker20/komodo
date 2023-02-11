import { useParams } from "@solidjs/router";
import {
  Component,
  createEffect,
  createMemo,
  createSignal,
  onCleanup,
  Show,
} from "solid-js";
import { client, pushNotification } from "../../../..";
import { useAppState } from "../../../../state/StateProvider";
import { DockerContainerState, Log as LogType } from "../../../../types";
import { combineClasses } from "../../../../util/helpers";
import { useBuffer, useLocalStorageToggle } from "../../../../util/hooks";
import Icon from "../../../shared/Icon";
import Flex from "../../../shared/layout/Flex";
import Grid from "../../../shared/layout/Grid";
import Selector from "../../../shared/menu/Selector";
import { useConfig } from "../config/Provider";
import s from "./log.module.scss";

const POLLING_RATE = 5000;

let interval = -1;

const Log: Component<{
  log?: LogType;
  logTail: number;
  setLogTail: (tail: number) => void;
  reload: () => Promise<void>;
  error?: boolean;
}> = (p) => {
  const { deployments } = useAppState();
  const params = useParams();
  const { userCanUpdate } = useConfig();
  const deployment = () => deployments.get(params.id);
  let ref: HTMLDivElement | undefined;
  let ignore = false;
  const [scrolled, setScrolled] = createSignal(false);
  createEffect(() => {
    if (params.id) {
      ref?.scroll({
        top: ref.scrollHeight,
      });
      setScrolled(false);
    }
  });
  createEffect(() => {
    if (!scrolled() && p.log) {
      ignore = true;
      ref?.scroll({
        top: ref.scrollHeight,
        behavior: "smooth",
      });
      setTimeout(() => {
        ignore = false;
      }, 1500);
    }
  });
  const log = () => {
    if (deployment()?.state === DockerContainerState.NotDeployed) {
      return "not deployed";
    } else {
      return (
        (p.error ? p.log?.stderr : p.log?.stdout) ||
        `no${p.error ? " error" : ""} log`
      );
    }
  };
  const buffer = useBuffer(scrolled, 250);
  const [poll, togglePoll] = useLocalStorageToggle(
    "deployment-log-polling",
    true
  );
  clearInterval(interval);
  interval = setInterval(() => {
    if (poll() && deployment()?.state === DockerContainerState.Running) {
      p.reload();
    }
  }, POLLING_RATE);
  onCleanup(() => clearInterval(interval));
  return (
    <Show when={p.log}>
      <Grid
        gap="0.5rem"
        style={{ height: "100%", "grid-template-rows": "auto 1fr" }}
      >
        <Flex
          alignItems="center"
          justifyContent="flex-end"
          style={{ margin: "0rem 0.5rem", height: "fit-content" }}
        >
          lines:
          <Selector
            targetClass="lightgrey"
            targetStyle={{ padding: "0.35rem" }}
            selected={p.logTail.toString()}
            items={["50", "100", "500", "1000"]}
            onSelect={(tail) => p.setLogTail(Number(tail))}
            position="bottom right"
            itemStyle={{ width: "4rem" }}
          />
          <Show when={userCanUpdate()}>
            <button
              class="blue"
              onClick={() =>
                client.download_container_log(
                  params.id,
                  deployment()!.deployment.name,
                  p.error
                )
              }
              style={{ padding: "0.35rem" }}
            >
              download full log
            </button>
          </Show>
          <button
            class="blue"
            onClick={async () => {
              await p.reload();
              pushNotification("good", "log refreshed");
            }}
            style={{ padding: "0.4rem" }}
          >
            <Icon type="refresh" />
          </button>
          <button class={poll() ? "green" : "red"} onClick={togglePoll}>
            {poll() ? "" : "don't "}poll
          </button>
        </Flex>
        <div style={{ position: "relative", height: "100%" }}>
          <div
            class={combineClasses(s.LogContainer, "scroller")}
            ref={ref}
            onScroll={() => {
              if (!ignore) {
                setScrolled(
                  !(
                    ref &&
                    (ref.scrollHeight - ref.scrollTop - ref.clientHeight) /
                      ref.scrollHeight <
                      0.01
                  )
                );
              }
            }}
          >
            <pre class={s.Log}>{log()}</pre>
          </div>
          <Show when={buffer()}>
            <button
              class={combineClasses(
                s.TopRight,
                "blue",
                scrolled() ? s.Enter : s.Exit
              )}
              onClick={() => setScrolled(false)}
            >
              <Icon type="arrow-down" />
            </button>
          </Show>
        </div>
      </Grid>
    </Show>
  );
};

export default Log;
