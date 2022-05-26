import { Log } from "@monitor/types";
import { Component, createSignal } from "solid-js";
import { useAppState } from "../../../../../state/StateProvider";
import { useToggle } from "../../../../../util/hooks";
import { getPm2Log } from "../../../../../util/query";
import Button from "../../../../util/Button";
import Icon from "../../../../util/Icon";
import Flex from "../../../../util/layout/Flex";
import Grid from "../../../../util/layout/Grid";
import CenterMenu from "../../../../util/menu/CenterMenu";

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
  const [log, setLog] = createSignal<Log>();
  const load = () => {
    getPm2Log(selected.id(), p.name).then((cle) => setLog(cle.log));
  };
  load();
  return (
    <Grid style={{ padding: "0.5rem" }}>
      <Flex justifyContent="space-between">
        <h1>log</h1>
        <Button class="blue" onClick={load}>
          <Icon type="refresh" />
        </Button>
      </Flex>
      <pre>{log()?.stdout}</pre>
      <pre>{log()?.stderr}</pre>
    </Grid>
  );
};

export default LogButton;
