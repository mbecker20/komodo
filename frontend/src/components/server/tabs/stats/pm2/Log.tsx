import { Log as LogType } from "@monitor/types";
import { Component, createEffect, createSignal } from "solid-js";
import { useAppState } from "../../../../../state/StateProvider";
import { useTheme } from "../../../../../state/ThemeProvider";
import { combineClasses } from "../../../../../util/helpers";
import { useToggle } from "../../../../../util/hooks";
import { getPm2Log } from "../../../../../util/query";
import Button from "../../../../util/Button";
import Icon from "../../../../util/Icon";
import Flex from "../../../../util/layout/Flex";
import Grid from "../../../../util/layout/Grid";
import CenterMenu from "../../../../util/menu/CenterMenu";
import Selector from "../../../../util/menu/Selector";
import s from "../stats.module.scss";

const LogButton: Component<{ name: string }> = (p) => {
  const [show, toggleShow] = useToggle();
  return (
    <CenterMenu
      show={show}
      toggleShow={toggleShow}
      target="show log"
      targetClass="blue"
      content={<Log name={p.name} />}
    />
  );
};

const Log: Component<{ name: string }> = (p) => {
  const { selected } = useAppState();
  const [log, setLog] = createSignal<LogType>();
  const [lines, setLines] = createSignal(50);
  const load = () => {
    getPm2Log(selected.id(), p.name, lines()).then((cle) => setLog(cle.log));
  };
  createEffect(load);
  const { themeClass } = useTheme();
  return (
    <Grid
      gap="0.2rem"
      style={{ padding: "0.5rem", width: "80vw", height: "90vh" }}
    >
      <Flex justifyContent="space-between" alignItems="center">
        <h1>log</h1>
        <Flex alignItems="center">
          lines:
          <Selector
            targetClass="lightgrey"
            targetStyle={{ padding: "0.35rem" }}
            selected={lines().toString()}
            items={["50", "100", "500", "1000"]}
            onSelect={(lines) => setLines(Number(lines))}
            position="bottom right"
            itemStyle={{ width: "4rem" }}
          />
          <Button class="blue" onClick={load}>
            <Icon type="refresh" />
          </Button>
        </Flex>
      </Flex>
      <pre class={combineClasses(s.Pm2Log, "scroller", themeClass())}>
        {log()?.stdout}
      </pre>
    </Grid>
  );
};

export default LogButton;
