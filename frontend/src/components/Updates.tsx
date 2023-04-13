import { A } from "@solidjs/router";
import {
  Component,
  createEffect,
  createMemo,
  createSignal,
  For,
  Show,
} from "solid-js";
import { OPERATIONS } from "..";
import { useAppDimensions } from "../state/DimensionProvider";
import { useAppState } from "../state/StateProvider";
import { Operation, Update as UpdateType, UpdateStatus } from "../types";
import {
  getId,
  readableMonitorTimestamp,
  readableVersion,
} from "../util/helpers";
import Icon from "./shared/Icon";
import Input from "./shared/Input";
import Flex from "./shared/layout/Flex";
import Grid from "./shared/layout/Grid";
import Loading from "./shared/loading/Loading";
import Selector from "./shared/menu/Selector";
import UpdateMenu from "./update/UpdateMenu";

const Updates: Component<{}> = (p) => {
  const { isMobile } = useAppDimensions();
  const { updates, usernames, name_from_update_target } = useAppState();
  const [operation, setOperation] = createSignal<Operation>();
  createEffect(() => {
    if (operation()) {
      updates.load([operation()!]);
    } else {
      updates.load();
    }
  });
  const [search, setSearch] = createSignal("");
  const filtered_updates = createMemo(() => {
    return updates.collection()?.filter((u) => {
      const name = name_from_update_target(u.target);
      if (name.includes(search())) return true;
      const username = usernames.get(u.operator);
      if (username?.includes(search())) return true;
    });
  });
  const [openMenu, setOpenMenu] = createSignal<string | undefined>(undefined);
  return (
    <Grid class="full-width card shadow">
      <Flex alignItems="center" justifyContent="space-between">
        <Flex>
          <h1>updates</h1>
          <UpdateMenu
            update={openMenu() ? updates.get(openMenu()!) : undefined}
            closeMenu={() => setOpenMenu(undefined)}
          />
        </Flex>
        <Flex alignItems="center">
          <Input class="lightgrey" placeholder="search" onEdit={setSearch} />
          <Selector
            label={isMobile() ? undefined : "operation: "}
            selected={operation() ? operation()! : "all"}
            items={["all", ...OPERATIONS]}
            onSelect={(o) =>
              o === "all"
                ? setOperation(undefined)
                : setOperation(o.replaceAll(" ", "_") as Operation)
            }
            targetClass="blue"
            position="bottom right"
            searchStyle={{ width: "15rem" }}
            menuClass="scroller"
            menuStyle={{ "max-height": "50vh" }}
            useSearch
          />
        </Flex>
      </Flex>
      <Show
        when={updates.loaded()}
        fallback={
          <Flex justifyContent="center">
            <Loading type="three-dot" />
          </Flex>
        }
      >
        <For each={filtered_updates()}>
          {(update) => (
            <Update
              update={update}
              openMenu={() => setOpenMenu(getId(update))}
            />
          )}
        </For>
        <Show when={!updates.noMore()}>
          <button
            class="grey full-width"
            onClick={() =>
              operation()
                ? updates.loadMore([operation()!])
                : updates.loadMore()
            }
          >
            load more
          </button>
        </Show>
      </Show>
    </Grid>
  );
};

export default Updates;

const Update: Component<{ update: UpdateType; openMenu: () => void }> = (p) => {
  const { isMobile } = useAppDimensions();
  const { usernames, name_from_update_target } = useAppState();
  const name = () => name_from_update_target(p.update.target);
  const operation = () => {
    if (p.update.operation === Operation.BuildBuild) {
      return `build ${readableVersion(p.update.version!)}`;
    }
    return `${p.update.operation.replaceAll("_", " ")}${
      p.update.version ? " " + readableVersion(p.update.version) : ""
    }`;
  };
  const link_to = () => {
    return p.update.target.type === "System"
      ? "/"
      : `/${p.update.target.type.toLowerCase()}/${p.update.target.id}`;
  };
  return (
    <Flex
      class="card light hover shadow wrap"
      justifyContent="space-between"
      alignItems="center"
    >
      <Flex
        alignItems="center"
        justifyContent="space-between"
        style={{ width: isMobile() ? "100%" : undefined }}
      >
        <A style={{ padding: 0 }} href={link_to()}>
          <h2 class="text-hover">{name()}</h2>
        </A>
        <div
          style={{
            color: !p.update.success ? "rgb(182, 47, 52)" : "inherit",
          }}
        >
          {operation()}
        </div>
        <Show when={p.update.status === UpdateStatus.InProgress}>
          <div style={{ opacity: 0.7 }}>(in progress)</div>
        </Show>
      </Flex>
      <Flex
        alignItems="center"
        justifyContent="space-between"
        style={{ width: isMobile() ? "100%" : undefined }}
      >
        <Flex gap="0.5rem">
          <Icon type="user" />
          <div>{usernames.get(p.update.operator)}</div>
        </Flex>
        <Flex alignItems="center">
          <div style={{ "place-self": "center end" }}>
            {readableMonitorTimestamp(p.update.start_ts)}
          </div>
          <button class="blue" onClick={p.openMenu}>
            <Icon type="console" />
          </button>
        </Flex>
      </Flex>
    </Flex>
  );
};
