import { Component } from "solid-js";
import Grid from "./shared/layout/Grid";
import Loading from "./shared/loading/Loading";

const NotFound: Component<{ type: "deployment" | "server" | "build" }> = (p) => {
	return (
    <Grid
      placeItems="center"
      style={{ height: "100%", width: "100%" }}
    >
      <Grid placeItems="center" style={{ width: "fit-content", height: "fit-content" }}>
        <h2>{p.type} at id not found</h2>
        <Loading type="sonar" />
      </Grid>
    </Grid>
  );
}

export default NotFound;