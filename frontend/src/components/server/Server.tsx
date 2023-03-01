import { useParams } from "@solidjs/router";
import { Component, Show } from "solid-js";
import { useAppDimensions } from "../../state/DimensionProvider";
import { useAppState } from "../../state/StateProvider";
import { useUser } from "../../state/UserProvider";
import { PermissionLevel } from "../../types";
import Description from "../Description";
import NotFound from "../NotFound";
import ServerChildren from "../server_children/ServerChildren";
import Grid from "../shared/layout/Grid";
import Actions from "./Actions";
import { ActionStateProvider } from "./ActionStateProvider";
import Header from "./Header";
import ServerTabs from "./tabs/Tabs";
import Updates from "./Updates";

const Server: Component<{}> = (p) => {
  const { user, user_id } = useUser();
  const { servers } = useAppState();
  const params = useParams();
  const server = () => servers.get(params.id)!;
  const { isSemiMobile } = useAppDimensions();
  const userCanUpdate = () =>
    user().admin ||
    server()?.server.permissions![user_id()] === PermissionLevel.Update;
  return (
    <Show when={server()} fallback={<NotFound type="server" loaded={servers.loaded()} />}>
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
            <Grid style={{ "flex-grow": 1, "grid-auto-rows": "auto auto 1fr" }}>
              <Header />
              <Description
                target={{ type: "Server", id: params.id }}
                name={server().server.name}
                description={server().server.description}
                userCanUpdate={userCanUpdate()}
              />
              <Actions />
            </Grid>
            <Show when={!isSemiMobile()}>
              <Updates />
            </Show>
          </Grid>
          <ServerChildren id={params.id} />
          <ServerTabs />
        </Grid>
      </ActionStateProvider>
    </Show>
  );
};

export default Server;
