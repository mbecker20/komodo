import { Component, Show } from "solid-js";
import Grid from "./shared/layout/Grid";
import Loading from "./shared/loading/Loading";

const NotFound: Component<{
  type: "deployment" | "server" | "build";
  loaded: boolean;
}> = (p) => {
  return (
    <Grid placeItems="center" style={{ height: "100%", width: "100%" }}>
      <Grid
        placeItems="center"
        style={{ width: "fit-content", height: "fit-content" }}
      >
        <Show when={p.loaded} fallback={<h2>loading {p.type}...</h2>}>
          <h2>{p.type} at id not found</h2>
        </Show>
        <Loading type="sonar" />
      </Grid>
    </Grid>
  );
};

export default NotFound;
