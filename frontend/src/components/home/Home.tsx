import { Component, For, Show } from "solid-js";
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

const Home: Component<{}> = (p) => {
  const { servers } = useAppState();
  const serverIDs = () => servers.loaded() && servers.ids();
  return (
    <Grid class={s.Home} placeItems="start center">
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
    </Grid>
  );
};

export default Home;
