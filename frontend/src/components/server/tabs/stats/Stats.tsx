import { Component } from "solid-js";
import Grid from "../../../util/layout/Grid";
import DockerStats from "./DockerStats";
import Pm2Processes from "./Pm2Processes";
import SystemStats from "./SystemStats";

const Stats: Component<{}> = (p) => {
  return (
    <Grid class="config">
      <Grid class="config-items">
        <SystemStats />
        <Pm2Processes />
        <DockerStats />
      </Grid>
    </Grid>
  );
};

export default Stats;