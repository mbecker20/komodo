import { Log as LogType } from "@monitor/types";
import { Component, createEffect, createSignal, Show } from "solid-js";
import { pushNotification } from "../../..";
import { useAppState } from "../../../state/StateProvider";
import { combineClasses } from "../../../util/helpers";
import Icon from "../../util/icons/Icon";
import s from "../deployment.module.css";

const Log: Component<{
  log: LogType;
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
    if (deployment()?.status === "not created") {
      return "not created";
    } else {
      return (p.error ? p.log?.stderr : p.log?.stdout) || "no log";
    }
  };
  return (
    <Show when={p.log}>
      <div style={{ position: "relative" }}>
        <div
          class={s.LogContainer}
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
        <button
          class={combineClasses(s.RefreshButton, "blue")}
          onClick={async () => {
            await p.reload();
            pushNotification("good", "log refreshed");
          }}
        >
          <Icon type="refresh" />
        </button>
        <Show when={scrolled()}>
          <button
            class={combineClasses(s.ReturnButton, "blue")}
            onClick={() => setScrolled(false)}
          >
            <Icon type="arrow-down" />
          </button>
        </Show>
      </div>
    </Show>
  );
};

export default Log;
