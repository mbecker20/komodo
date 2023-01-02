import { Component } from "solid-js";
import Grid from "../../../shared/layout/Grid";
import DockerStats from "./DockerStats";
import Pm2Processes from "./pm2/Pm2Processes";
import SystemStats from "./SystemStats";

const Stats: Component<{}> = (p) => {
  return (
    <Grid class="config">
      <Grid class="config-items">
        <Pm2Processes />
        <SystemStats />
        <DockerStats />
      </Grid>
    </Grid>
  );
};

export default Stats;