import { Component } from "solid-js";
import { createStore } from "solid-js/store";
import { useTheme } from "../../../../state/ThemeProvider";
import { combineClasses, validatePercentage } from "../../../../util/helpers";
import Input from "../../../util/Input";
import Flex from "../../../util/layout/Flex";
import Grid from "../../../util/layout/Grid";
import { useConfig } from "./Provider";

const Alerts: Component<{}> = (p) => {
  const { server, setServer } = useConfig();
  const { themeClass } = useTheme();
  const [alerts, setAlerts] = createStore({
    cpu: server.cpuAlert?.toString(),
    mem: server.memAlert?.toString(),
    disk: server.diskAlert?.toString(),
  });
  return (
    <Grid class={combineClasses("config-item shadow", themeClass())}>
      <h1>alerts</h1>
      <Flex justifyContent="space-between">
        <div>cpu</div>
        <Flex alignItems="center">
          <Input
            placeholder="%"
            value={alerts.cpu || server.cpuAlert?.toString()}
            onEdit={(val) => setAlerts("cpu", val)}
            onConfirm={(val) => {
              if (validatePercentage(val)) {
                setServer("cpuAlert", Number(val));
              } else {
                setAlerts("cpu", server.cpuAlert?.toString());
              }
            }}
          />
          <div>%</div>
        </Flex>
      </Flex>
      <Flex justifyContent="space-between">
        <div>mem</div>
        <Flex alignItems="center">
          <Input
            placeholder="%"
            value={alerts.mem || server.memAlert?.toString()}
            onEdit={(val) => setAlerts("mem", val)}
            onConfirm={(val) => {
              if (validatePercentage(val)) {
                setServer("memAlert", Number(val));
              } else {
                setAlerts("mem", server.memAlert?.toString());
              }
            }}
          />
          <div>%</div>
        </Flex>
      </Flex>
      <Flex justifyContent="space-between">
        <div>disk</div>
        <Flex alignItems="center">
          <Input
            placeholder="%"
            value={alerts.disk || server.diskAlert?.toString()}
            onEdit={(val) => setAlerts("disk", val)}
            onConfirm={(val) => {
              if (validatePercentage(val)) {
                setServer("diskAlert", Number(val));
              } else {
                setAlerts("disk", server.diskAlert?.toString());
              }
            }}
          />
          <div>%</div>
        </Flex>
      </Flex>
    </Grid>
  );
};

export default Alerts;
