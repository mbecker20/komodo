import { Log as LogType } from "@monitor/types";
import { Component, createEffect, createSignal, Show } from "solid-js";
import { pushNotification } from "../../../..";
import { useAppState } from "../../../../state/StateProvider";
import { combineClasses } from "../../../../util/helpers";
import { useBuffer } from "../../../../util/hooks";
import { downloadDeploymentLog } from "../../../../util/query";
import Icon from "../../../util/Icon";
import Flex from "../../../util/layout/Flex";
import Grid from "../../../util/layout/Grid";
import Selector from "../../../util/menu/Selector";
import s from "./log.module.scss";

const Log: Component<{
  log: LogType;
  logTail: number;
  setLogTail: (tail: number) => void;
  reload: () => Promise<void>;
  error?: boolean;
}> = (p) => {
  const { selected, deployments } = useAppState();
  const deployment = () => deployments.get(selected.id());
  let ref: HTMLDivElement | undefined;
  let ignore = false;
  const [scrolled, setScrolled] = createSignal(false);
  createEffect(() => {
    if (selected.id()) {
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
    if (deployment()?.status === "not deployed") {
      return "not deployed";
    } else {
      return (p.error ? p.log?.stderr : p.log?.stdout) || "no log";
    }
  };
  const buffer = useBuffer(scrolled, 250);
  return (
    <Show when={p.log}>
      <Flex
        alignItems="center"
        justifyContent="space-between"
        style={{ margin: "0rem 0.5rem" }}
      >
        <Flex alignItems="center">
          number of lines:
          <Selector
            targetClass="lightgrey"
            targetStyle={{ padding: "0.35rem" }}
            selected={p.logTail.toString()}
            items={["50", "100", "500", "1000"]}
            onSelect={(tail) => p.setLogTail(Number(tail))}
            position="bottom right"
            itemStyle={{ width: "4rem" }}
          />
        </Flex>
        <button
          class="blue"
          onClick={() =>
            downloadDeploymentLog(selected.id(), deployment()!.name)
          }
          style={{ padding: "0.35rem" }}
        >
          download full log
        </button>
      </Flex>
      <div style={{ position: "relative" }}>
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
        <Grid class={s.TopRight}>
          <button
            class="blue"
            onClick={async () => {
              await p.reload();
              pushNotification("good", "log refreshed");
            }}
          >
            <Icon type="refresh" />
          </button>
          <Show when={buffer()}>
            <button
              class={combineClasses("blue", scrolled() ? s.Enter : s.Exit)}
              onClick={() => setScrolled(false)}
            >
              <Icon type="arrow-down" />
            </button>
          </Show>
        </Grid>
      </div>
    </Show>
  );
};

export default Log;
