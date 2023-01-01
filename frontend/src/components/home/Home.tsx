import { Component, createMemo, createSignal, For, Match, Switch } from "solid-js";
import { useAppDimensions } from "../../state/DimensionProvider";
import { useAppState } from "../../state/StateProvider";
import { combineClasses } from "../../util/helpers";
import Input from "../shared/Input";
import Grid from "../shared/layout/Grid";
import Tabs from "../shared/tabs/Tabs";
import s from "./home.module.scss"
import Summary from "./Summary";
import AddServer from "./Tree/AddServer";
import Builds from "./Tree/Builds";
import Server from "./Tree/Server";
import Updates from "./Updates/Updates";

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
        const name = servers.get(id)!.server.name;
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
        <Grid class={combineClasses(s.Home)}>
          <Tabs
            localStorageKey="home-tab"
            containerClass={s.Tabs}
            tabs={[
              {
                title: "deployments",
                element: () => (
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
                element: () => <Builds />,
              },
            ]}
          />
          <Grid>
            <Summary />
            <Updates />
          </Grid>
        </Grid>
      </Match>
      <Match when={width() < 1200}>
        <Grid class={s.Home}>
          <Summary />
          <Tabs
            localStorageKey="home-tab"
            containerClass={s.Tabs}
            tabs={[
              {
                title: "deployments",
                element: () => (
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
                element: () => <Builds />,
              },
            ]}
          />
          <Updates />
        </Grid>
      </Match>
    </Switch>
  );
}

export default Home;