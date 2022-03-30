import { Component, Show } from "solid-js";
import { combineClasses } from "../../../../util/helpers";
import ConfirmButton from "../../../util/ConfirmButton";
import Icon from "../../../util/icons/Icon";
import Flex from "../../../util/layout/Flex";
import Grid from "../../../util/layout/Grid";
import Loading from "../../../util/loading/Loading";
import s from "../../build.module.css";
import { useConfig } from "../Provider";
import Git from "./Git";
import OnClone from "./OnClone";

const GitConfig: Component<{}> = (p) => {
  const { build, reset, save } = useConfig();
  return (
    <Show when={build.loaded}>
      <Grid class={s.Config}>
        <Grid class={combineClasses(s.ConfigItems, "scroller")}>
          <Git />
          <OnClone />
        </Grid>
        <Show when={build.updated}>
          <Show when={!build.saving} fallback={<button class="green">
            updating <Loading type="spinner" />
          </button>}>
            <Flex style={{ "place-self": "center", padding: "1rem" }}>
              <button onClick={reset}>
                reset
                <Icon type="reset" />
              </button>
              <ConfirmButton onConfirm={save} color="green">
                save
                <Icon type="floppy-disk" />
              </ConfirmButton>
            </Flex>
          </Show>
        </Show>
      </Grid>
    </Show>
  );
};

export default GitConfig;
