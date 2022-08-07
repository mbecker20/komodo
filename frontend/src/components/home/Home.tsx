import { Component, For, Match, Show, Switch } from "solid-js";
import { useAppState } from "../../state/StateProvider";
import { useTheme } from "../../state/ThemeProvider";
import { useUser } from "../../state/UserProvider";
import Grid from "../util/layout/Grid";
import Tabs from "../util/tabs/Tabs";
import Server from "./Tree/Server";
import Builds from "./Tree/Builds";
import s from "./home.module.scss";
import AddServer from "./Tree/AddServer";
import Summary from "./Summary/Summary";
import Updates from "./Updates/Updates";
import { useAppDimensions } from "../../state/DimensionProvider";

const Home: Component<{}> = (p) => {
  const { servers } = useAppState();
  const { width, isMobile } = useAppDimensions();
  const serverIDs = () => servers.loaded() && servers.ids();
  return (
    <Switch>
      <Match when={width() >= 1200}>
        <Grid gap="0rem" class={s.Home} placeItems="start center">
          <Tabs
            localStorageKey="home-tab"
            containerClass={s.Tabs}
            tabs={[
              {
                title: "deployments",
                element: (
                  <Grid>
                    <For each={serverIDs()}>{(id) => <Server id={id} />}</For>
                    <AddServer />
                  </Grid>
                ),
              },
              {
                title: "builds",
                element: <Builds />,
              },
            ]}
          />
          <Grid gap="0rem" style={{ width: "80%" }}>
            <Summary />
            <Updates />
          </Grid>
        </Grid>
      </Match>
      <Match when={width() < 1200}>
        <Grid gap="0rem" class={s.Home} placeItems="start center">
          <Summary />
          <Tabs
            localStorageKey="home-tab"
            containerClass={s.Tabs}
            tabs={[
              {
                title: "deployments",
                element: (
                  <Grid>
                    <For each={serverIDs()}>{(id) => <Server id={id} />}</For>
                    <AddServer />
                  </Grid>
                ),
              },
              {
                title: "builds",
                element: <Builds />,
              },
            ]}
          />
          <Updates />
        </Grid>
      </Match>
    </Switch>
  );
};

export default Home;
