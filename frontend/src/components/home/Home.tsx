import {
  Component,
  createMemo,
  createSignal,
  For,
  Match,
  Switch,
} from "solid-js";
import { useAppDimensions } from "../../state/DimensionProvider";
import { useAppState } from "../../state/StateProvider";
import { combineClasses } from "../../util/helpers";
import Grid from "../shared/layout/Grid";
import SimpleTabs from "../shared/tabs/SimpleTabs";
import s from "./home.module.scss";
import Summary from "./Summary";
import Groups from "./Tree/Groups";
import Servers from "./Tree/Servers";
import Updates from "./Updates/Updates";

const Home: Component<{}> = (p) => {
  const { width } = useAppDimensions();
  const { servers } = useAppState();
  return (
    <Switch>
      <Match when={width() >= 1200}>
        <Grid class={combineClasses(s.Home)}>
          <Grid style={{ height: "fit-content" }}>
            <SimpleTabs
              localStorageKey="home-groups-servers-tab-v1"
              tabs={[
                {
                  title: "groups",
                  element: () => <Groups />,
                },
                {
                  title: "servers",
                  element: () => <Servers serverIDs={servers.ids()!} showAdd />,
                },
              ]}
            />
          </Grid>
          <Grid style={{ height: "fit-content" }}>
            <Summary />
            <Updates />
          </Grid>
        </Grid>
      </Match>
      <Match when={width() < 1200}>
        <Grid class={s.Home}>
          {/* <Summary /> */}
          <SimpleTabs
            localStorageKey="home-groups-servers-tab-v1"
            tabs={[
              {
                title: "groups",
                element: () => <Groups />,
              },
              {
                title: "servers",
                element: () => <Servers serverIDs={servers.ids()!} showAdd />,
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
