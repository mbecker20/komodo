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
import Input from "../shared/Input";
import Grid from "../shared/layout/Grid";
import Tabs from "../shared/tabs/Tabs";
import s from "./home.module.scss";
import Summary from "./Summary";
import Builds from "./Tree/Build";
import Servers from "./Tree/Servers";
import Updates from "./Updates/Updates";

const Home: Component<{}> = (p) => {
  const { width } = useAppDimensions();
  return (
    <Switch>
      <Match when={width() >= 1200}>
        <Grid class={combineClasses(s.Home)}>
          <Servers />
          <Grid style={{ height: "fit-content" }}>
            <Summary />
            <Updates />
          </Grid>
        </Grid>
      </Match>
      <Match when={width() < 1200}>
        <Grid class={s.Home}>
          {/* <Summary /> */}
          <Servers />
          <Updates />
        </Grid>
      </Match>
    </Switch>
  );
};

export default Home;
