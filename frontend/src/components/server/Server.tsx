import { useParams } from "@solidjs/router";
import { Component, Show } from "solid-js";
import { useAppDimensions } from "../../state/DimensionProvider";
import { useAppState } from "../../state/StateProvider";
import NotFound from "../NotFound";
import Grid from "../shared/layout/Grid";
import Actions from "./Actions";
import { ActionStateProvider } from "./ActionStateProvider";
import Header from "./Header";
import ServerTabs from "./tabs/Tabs";
import Updates from "./Updates";

const Server: Component<{}> = (p) => {
  const { servers } = useAppState();
  const params = useParams();
  const server = () => servers.get(params.id)!;
  const { isSemiMobile } = useAppDimensions();
  // const userCanUpdate = () =>
  //   user().admin ||
  //   server()!.server.permissions![getId(user())] === PermissionLevel.Update;
  return (
    <Show when={server()} fallback={<NotFound type="server" />}>
      <ActionStateProvider>
        <Grid
          style={{
            width: "100%",
            "box-sizing": "border-box",
          }}
        >
          <Grid
            style={{ width: "100%" }}
            gridTemplateColumns={isSemiMobile() ? "1fr" : "1fr 1fr"}
          >
            <Grid>
              <Header />
              <Actions />
            </Grid>
            <Show when={!isSemiMobile()}>
              <Updates />
            </Show>
          </Grid>
          <ServerTabs />
        </Grid>
      </ActionStateProvider>
    </Show>
  );
};

export default Server;
