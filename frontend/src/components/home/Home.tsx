import {
  Component,
} from "solid-js";
import { useAppDimensions } from "../../state/DimensionProvider";
import Grid from "../shared/layout/Grid";
import SimpleTabs from "../shared/tabs/SimpleTabs";
import Summary from "./Summary";
import Builds from "./Tree/Builds";
import Groups from "./Tree/Groups";
import { TreeProvider } from "./Tree/Provider";
import Updates from "./Updates/Updates";

const Home: Component<{}> = (p) => {
  const { isSemiMobile } = useAppDimensions();
  return (
    <>
      <Grid
        style={{ width: "100%" }}
        gridTemplateColumns={isSemiMobile() ? "1fr" : "1fr 1fr"}
      >
        <Summary />
        <Updates />
      </Grid>
      <TreeProvider>
        <SimpleTabs
          containerStyle={{ width: "100%" }}
          localStorageKey="home-groups-servers-tab-v1"
          tabs={[
            {
              title: "servers",
              element: () => <Groups />,
            },
            {
              title: "builds",
              element: () => <Builds />
            }
          ]}
        />
      </TreeProvider>
    </>
  );
};

export default Home;
