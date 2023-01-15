import { Component, createMemo, createSignal, For, Show } from "solid-js";
import { useAppState } from "../../../state/StateProvider";
import { useUser } from "../../../state/UserProvider";
import Input from "../../shared/Input";
import Grid from "../../shared/layout/Grid";
import AddServer from "./AddServer";
import Server from "./Server";

const Servers: Component<{ serverIDs: string[]; showAdd?: boolean }> = (p) => {
  const { user } = useUser();
  const { servers } = useAppState();
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
      });
    } else {
      return undefined;
    }
  });
  return (
    <Grid style={{ height: "fit-content" }}>
      <Input
        placeholder="filter servers"
        value={serverFilter()}
        onEdit={setServerFilter}
        style={{ width: "100%", padding: "0.5rem" }}
      />
      <For each={serverIDs()}>{(id) => <Server id={id} />}</For>
      <Show
        when={p.showAdd && (user().admin || user().create_server_permissions)}
      >
        <AddServer />
      </Show>
    </Grid>
  );
};

export default Servers;
