import { ContainerStatus, Deployment } from "@monitor/types";
import { Component, Match, Switch } from "solid-js";
import { combineClasses } from "../../util/helpers";
import Icon from "../util/icons/Icon";
import Flex from "../util/layout/Flex";
import Grid from "../util/layout/Grid";
import s from "./deployment.module.css";

const Actions: Component<{ deployment: Deployment }> = (p) => {
  return (
    <Grid class={combineClasses(s.Actions, "shadow")}>
      <div class={s.ItemHeader}>actions</div>
      <Switch>
        <Match
          when={(p.deployment.status as ContainerStatus)?.State === "running"}
        >
          <Flex class={combineClasses(s.Action, "shadow")}>
            deploy{" "}
            <Flex>
              <button>
                <Icon type="reset" />
              </button>
              <button>
                <Icon type="trash" />
              </button>
            </Flex>
          </Flex>
          <Flex class={combineClasses(s.Action, "shadow")}>
            container{" "}
            <button>
              <Icon type="pause" />
            </button>
          </Flex>
        </Match>

        <Match
          when={(p.deployment.status as ContainerStatus).State === "exited"}
        >
          <Flex class={combineClasses(s.Action, "shadow")}>
            deploy{" "}
            <button>
              <Icon type="reset" />
            </button>
            <button>
              <Icon type="trash" />
            </button>
          </Flex>
          <Flex class={combineClasses(s.Action, "shadow")}>
            container{" "}
            <button>
              <Icon type="play" />
            </button>
          </Flex>
        </Match>

        <Match when={p.deployment.status === "not created"}>
          <Flex class={combineClasses(s.Action, "shadow")}>
            deploy{" "}
            <button>
              <Icon type="play" />
            </button>
          </Flex>
        </Match>
      </Switch>
    </Grid>
  );
};

export default Actions;
