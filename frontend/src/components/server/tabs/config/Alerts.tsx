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
      <Grid style={{ padding: "0.5rem" }}>
        <Flex justifyContent="space-between" alignItems="center">
          <div>cpu</div>
          <Flex alignItems="center" gap="0.5rem">
            <Input
              placeholder="%"
              value={alerts.cpu}
              onEdit={(val) => setAlerts("cpu", val)}
              onConfirm={(val) => {
                if (validatePercentage(val)) {
                  setServer("cpuAlert", Number(val));
                } else {
                  setAlerts("cpu", server.cpuAlert?.toString());
                }
              }}
              style={{ width: "4rem" }}
            />
            <div>%</div>
          </Flex>
        </Flex>
        <Flex justifyContent="space-between" alignItems="center">
          <div>mem</div>
          <Flex alignItems="center" gap="0.5rem">
            <Input
              placeholder="%"
              value={alerts.mem}
              onEdit={(val) => setAlerts("mem", val)}
              onConfirm={(val) => {
                if (validatePercentage(val)) {
                  setServer("memAlert", Number(val));
                } else {
                  setAlerts("mem", server.memAlert?.toString());
                }
              }}
              style={{ width: "4rem" }}
            />
            <div>%</div>
          </Flex>
        </Flex>
        <Flex justifyContent="space-between" alignItems="center">
          <div>disk</div>
          <Flex alignItems="center" gap="0.5rem">
            <Input
              placeholder="%"
              value={alerts.disk}
              onEdit={(val) => setAlerts("disk", val)}
              onConfirm={(val) => {
                if (validatePercentage(val)) {
                  setServer("diskAlert", Number(val));
                } else {
                  setAlerts("disk", server.diskAlert?.toString());
                }
              }}
              style={{ width: "4rem" }}
            />
            <div>%</div>
          </Flex>
        </Flex>
      </Grid>
    </Grid>
  );
};

export default Alerts;
