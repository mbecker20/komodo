import {
  Component,
  createMemo,
  createSignal,
  For,
  Match,
  Show,
  Switch,
} from "solid-js";
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
import Input from "../util/Input";

const Home: Component<{}> = (p) => {
  const { servers } = useAppState();
  const { width } = useAppDimensions();
  const [serverFilter, setServerFilter] = createSignal("");
  const serverIDs = createMemo(() => {
    if (servers.loaded()) {
      const filters = serverFilter()
        .split(" ")
        .filter((term) => term.length > 0)
        .map((term) => term.toLowerCase());
      return servers.ids()?.filter((id) => {
        const name = servers.get(id)!.name;
        for (const term of filters) {
          if (!name.includes(term)) {
            return false;
          }
        }
        return true;
      });
    } else {
      return undefined;
    }
  });
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
                  <Grid gap="0.5rem">
                    <Input
                      placeholder="filter servers"
                      value={serverFilter()}
                      onEdit={setServerFilter}
                      style={{ width: "100%", padding: "0.5rem" }}
                    />
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
                  <Grid gap="0.5rem">
                    <Input
                      placeholder="filter servers"
                      value={serverFilter()}
                      onEdit={setServerFilter}
                      style={{ width: "100%", padding: "0.5rem" }}
                    />
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
