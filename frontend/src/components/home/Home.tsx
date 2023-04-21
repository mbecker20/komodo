import {
  Component, Match, Show, Switch,
} from "solid-js";
import { useAppDimensions } from "../../state/DimensionProvider";
import Grid from "../shared/layout/Grid";
import { ControlledSimpleTabs } from "../shared/tabs/SimpleTabs";
import Summary from "./Summary";
import Builds from "./Tree/Builds";
import Groups from "./Tree/Groups";
import { TreeProvider } from "./Tree/Provider";
import Updates from "./Updates/Updates";
import { useLocalStorage } from "../../util/hooks";
import BuildSummary from "./BuildSummary";

const Home: Component<{}> = (p) => {
  const { isSemiMobile } = useAppDimensions();
  const [selectedTab, setTab] = useLocalStorage<"servers" | "builds">(
    "servers",
    "home-groups-servers-tab-v2"
  );
  return (
    <>
      <Grid
        style={{ width: "100%" }}
        gridTemplateColumns={isSemiMobile() ? "1fr" : "1fr 1fr"}
      >
        <Switch>
          <Match when={selectedTab() === "servers"}>
            <Summary />
          </Match>
          <Match when={selectedTab() === "builds"}>
            <BuildSummary />
          </Match>
        </Switch>
        <Updates />
      </Grid>
      <TreeProvider>
        <ControlledSimpleTabs
          selected={selectedTab}
          set={setTab as any}
          containerStyle={{ width: "100%" }}
          tabs={[
            {
              title: "servers",
              element: () => <Groups />,
            },
            {
              title: "builds",
              element: () => <Builds />,
            },
          ]}
        />
      </TreeProvider>
    </>
  );
};

export default Home;
