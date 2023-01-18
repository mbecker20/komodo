import { Component, For, Show } from "solid-js";
import Icon from "../../../shared/Icon";
import Input from "../../../shared/Input";
import Flex from "../../../shared/layout/Flex";
import Grid from "../../../shared/layout/Grid";
import { useConfig } from "./Provider";
import { combineClasses } from "../../../../util/helpers";

const ToNotify: Component<{}> = (p) => {
  const { server, setServer, userCanUpdate } = useConfig();
  return (
    <Grid class={combineClasses("config-item shadow")}>
      <Flex alignItems="center" justifyContent="space-between">
        <h1>notify</h1>
        <Show when={userCanUpdate()}>
          <button
            class="green"
            onClick={() =>
              setServer("to_notify", (toNotify) =>
                toNotify ? [...toNotify, ""] : [""]
              )
            }
          >
            <Icon type="plus" />
          </button>
        </Show>
      </Flex>
      <For each={server.to_notify}>
        {(user, index) => (
          <Flex justifyContent="space-between" alignItems="center">
            <Input
              placeholder="slack user id"
              value={user}
              onEdit={(user) => setServer("to_notify", index(), user)}
              disabled={!userCanUpdate()}
            />
            <Show when={userCanUpdate()}>
              <button
                class="red"
                onClick={() =>
                  setServer("to_notify", (toNotify) =>
                    toNotify!.filter((_, i) => i !== index())
                  )
                }
              >
                <Icon type="trash" />
              </button>
            </Show>
          </Flex>
        )}
      </For>
      <Show when={server.to_notify?.length === 0}>
        <div>no slack users to notify</div>
      </Show>
    </Grid>
  );
};

export default ToNotify;
