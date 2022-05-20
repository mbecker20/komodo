import { Component, For, Show } from "solid-js";
import Icon from "../../../util/Icon";
import Input from "../../../util/Input";
import Flex from "../../../util/layout/Flex";
import Grid from "../../../util/layout/Grid";
import { useConfig } from "./Provider";
import { useTheme } from "../../../../state/ThemeProvider";
import { combineClasses } from "../../../../util/helpers";
import Button from "../../../util/Button";

const ToNotify: Component<{}> = (p) => {
  const { server, setServer } = useConfig();
  const { themeClass } = useTheme();
  return (
    <Grid class={combineClasses("config-item shadow", themeClass())}>
      <Flex alignItems="center" justifyContent="space-between">
        <h1>notify</h1>
        <Button
          class="green"
          onClick={() => setServer("toNotify", (toNotify) => [...toNotify, ""])}
        >
          <Icon type="plus" />
        </Button>
      </Flex>
      <For each={server.toNotify}>
        {(user, index) => (
          <Flex justifyContent="space-between" alignItems="center">
            <Input
              placeholder="slack user id"
              value={user}
              onEdit={(user) => setServer("toNotify", index(), user)}
            />
            <Button
              class="red"
              onClick={() =>
                setServer("toNotify", (toNotify) =>
                  toNotify.filter((_, i) => i !== index())
                )
              }
            >
              <Icon type="trash" />
            </Button>
          </Flex>
        )}
      </For>
			<Show when={server.toNotify.length === 0}>
				<div>no slack users to notify</div>
			</Show>
    </Grid>
  );
};

export default ToNotify;
