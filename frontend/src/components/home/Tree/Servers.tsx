import { Component, createMemo, createSignal, For, Show } from "solid-js";
import { useAppState } from "../../../state/StateProvider";
import { useUser } from "../../../state/UserProvider";
import Input from "../../shared/Input";
import Grid from "../../shared/layout/Grid";
import Selector from "../../shared/menu/Selector";
import AddServer from "../../topbar/AddServer";
import { TreeSortType, TREE_SORTS, useTreeState } from "./Provider";
import Server from "./Server";

const Servers: Component<{ serverIDs: string[]; showAdd?: boolean }> = (p) => {
  const { user } = useUser();
  const { servers } = useAppState();
  const { sort, setSort, server_sorter } = useTreeState();
  const [serverFilter, setServerFilter] = createSignal("");
  const serverIDs = createMemo(() => {
    if (servers.loaded()) {
      const filters = serverFilter()
        .split(" ")
        .filter((term) => term.length > 0)
        .map((term) => term.toLowerCase());
      return p.serverIDs.filter((id) => {
        const name = servers.get(id)!.server.name;
        for (const term of filters) {
          if (!name.includes(term)) {
            return false;
          }
        }
        return true;
      })
      .sort(server_sorter());
    } else {
      return undefined;
    }
  });
  return (
    <Grid style={{ height: "fit-content" }}>
      <Grid gridTemplateColumns="1fr auto auto">
        <Input
          placeholder="filter servers"
          value={serverFilter()}
          onEdit={setServerFilter}
          style={{ width: "100%", padding: "0.5rem" }}
        />
        <Selector
          label={<div class="dimmed">sort by:</div>}
          selected={sort()}
          items={TREE_SORTS as any as string[]}
          onSelect={(mode) => setSort(mode as TreeSortType)}
          position="bottom right"
          targetClass="blue"
          targetStyle={{ height: "100%" }}
          containerStyle={{ height: "100%" }}
        />
        <Show
          when={p.showAdd && (user().admin || user().create_server_permissions)}
        >
          <AddServer />
        </Show>
      </Grid>
      <For each={serverIDs()}>{(id) => <Server id={id} />}</For>
    </Grid>
  );
};

export default Servers;
