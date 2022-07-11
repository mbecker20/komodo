import { Component, For, Show } from "solid-js";
import { useAppState } from "../../state/StateProvider";
import { useTheme } from "../../state/ThemeProvider";
import { useUser } from "../../state/UserProvider";
import Grid from "../util/layout/Grid";
import Tabs from "../util/tabs/Tabs";
import Server from "./Server";
import Builds from "./Builds";
import s from "./home.module.scss";
import AddServer from "./AddServer";

const Home: Component<{}> = (p) => {
  const { username, permissions } = useUser();
  const { deployments, builds, servers, selected } = useAppState();
  const { themeClass } = useTheme();
  const serverIDs = () => servers.loaded() && servers.ids();
  return (
    <Grid class={s.Home} placeItems="start center">
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
