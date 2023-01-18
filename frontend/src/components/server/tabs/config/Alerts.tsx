import { Component } from "solid-js";
import { createStore } from "solid-js/store";
import { combineClasses, validatePercentage } from "../../../../util/helpers";
import Input from "../../../shared/Input";
import Flex from "../../../shared/layout/Flex";
import Grid from "../../../shared/layout/Grid";
import { useConfig } from "./Provider";

const Alerts: Component<{}> = (p) => {
  const { server, setServer, userCanUpdate } = useConfig();
  const [alerts, setAlerts] = createStore({
    cpu: server.cpu_alert?.toString(),
    mem: server.mem_alert?.toString(),
    disk: server.disk_alert?.toString(),
  });
  return (
    <Grid class={combineClasses("config-item shadow")}>
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
                  setServer("cpu_alert", Number(val));
                } else {
                  setAlerts("cpu", server.cpu_alert?.toString());
                }
              }}
              style={{ width: "4rem" }}
              disabled={!userCanUpdate()}
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
                  setServer("mem_alert", Number(val));
                } else {
                  setAlerts("mem", server.mem_alert?.toString());
                }
              }}
              style={{ width: "4rem" }}
              disabled={!userCanUpdate()}
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
                  setServer("disk_alert", Number(val));
                } else {
                  setAlerts("disk", server.disk_alert?.toString());
                }
              }}
              style={{ width: "4rem" }}
              disabled={!userCanUpdate()}
            />
            <div>%</div>
          </Flex>
        </Flex>
      </Grid>
    </Grid>
  );
};

export default Alerts;
