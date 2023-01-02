import { Log as LogType } from "@monitor/types";
import {
  Accessor,
  Component,
  createEffect,
  createSignal,
} from "solid-js";
import { pushNotification } from "../../../../..";
import { useAppState } from "../../../../../state/StateProvider";
import { useTheme } from "../../../../../state/ThemeProvider";
import { combineClasses } from "../../../../../util/helpers";
import { useToggle } from "../../../../../util/hooks";
import { getPm2Log } from "../../../../../util/query";
import Button from "../../../../util/Button";
import Icon from "../../../../util/Icon";
import Grid from "../../../../util/layout/Grid";
import CenterMenu from "../../../../util/menu/CenterMenu";
import Selector from "../../../../util/menu/Selector";
import s from "../stats.module.scss";

const LogButton: Component<{ name: string }> = (p) => {
  const { selected } = useAppState();
  const [show, toggleShow] = useToggle();
  const [log, setLog] = createSignal<LogType>();
  const [lines, setLines] = createSignal(50);
  const load = () => {
    getPm2Log(selected.id(), p.name, lines()).then((cle) => setLog(cle.log));
  };
  return (
    <CenterMenu
      show={show}
      toggleShow={toggleShow}
      title={`${p.name} log`}
      target="show log"
      targetClass="blue"
      leftOfX={
        <>
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
          <Button class="blue" onClick={async () => {
            const cle = await getPm2Log(selected.id(), p.name, lines());
            setLog(cle.log);
            pushNotification("good", "log reloaded");
          }}>
            <Icon type="refresh" />
          </Button>
        </>
      }
      content={<Log name={p.name} log={log} setLog={setLog} load={load} />}
    />
  );
};

const Log: Component<{
  name: string;
  log: Accessor<LogType | undefined>;
  setLog: (log: LogType) => void;
  load: () => void;
}> = (p) => {
  createEffect(p.load);
  const { themeClass } = useTheme();
  return (
    <Grid
      gap="0.2rem"
      style={{ padding: "0.5rem", width: "80vw", height: "90vh" }}
    >
      <pre class={combineClasses(s.Pm2Log, "scroller", themeClass())}>
        {p.log()?.stdout}
      </pre>
    </Grid>
  );
};

export default LogButton;
