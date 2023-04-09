import { Component, For, Show, createMemo, createSignal } from "solid-js";
import Grid from "../../shared/layout/Grid";
import { useLocalStorage, useToggle } from "../../../util/hooks";
import Flex from "../../shared/layout/Flex";
import { TREE_SORTS, TreeSortType, useTreeState } from "./Provider";
import { useAppState } from "../../../state/StateProvider";
import Input from "../../shared/Input";
import Selector from "../../shared/menu/Selector";
import { NewGroup } from "../../New";
import Icon from "../../shared/Icon";
import { useAppDimensions } from "../../../state/DimensionProvider";
import ConfirmButton from "../../shared/ConfirmButton";
import Server from "./Server";
import { useWindowKeyDown } from "../../../util/hooks";
import Menu from "../../shared/menu/Menu";
import { client } from "../../..";
import { useUser } from "../../../state/UserProvider";
import { PermissionLevel } from "../../../types";

const Groups: Component<{}> = (p) => {
  const { isSemiMobile } = useAppDimensions();
  const { user, user_id } = useUser();
  const { groups } = useAppState();
  const [selected, setSelected] = useLocalStorage<string | null>(
    null,
    "home-selected-group-v1"
  );
  const [searchFilter, setSearchFilter] = createSignal("");
  const { sort, setSort, group_sorter } = useTreeState();
  const [editing, toggleEditing, setEditing] = useToggle();
  const groupIDs = createMemo(() => {
    if (groups.loaded()) {
      const filters = searchFilter()
        .split(" ")
        .filter((term) => term.length > 0)
        .map((term) => term.toLowerCase());
      return groups
        .ids()
        ?.filter((id) => {
          const name = groups.get(id)!.name;
          for (const term of filters) {
            if (!name.includes(term)) {
              return false;
            }
          }
          return true;
        })
        .sort(group_sorter());
    } else {
      return undefined;
    }
  });
  const canEdit = (group_id: string) =>
    user().admin ||
    groups.get(group_id)?.permissions?.[user_id()] === PermissionLevel.Update;
  return (
    <Grid class="full-width">
      <Grid gridTemplateColumns={selected() ? "auto 1fr auto" : "1fr auto"}>
        <Show when={selected()}>
          <Flex alignItems="center">
            <button class="grey" onClick={() => setSelected(null)}>
              <Icon type="arrow-left" />
            </button>
            <h1 style={{ margin: "0 1rem" }}>
              {selected() === "all" ? "all" : groups.get(selected()!)?.name}
            </h1>
          </Flex>
        </Show>
        <Input
          placeholder={`filter ${selected() ? "servers" : "groups"}`}
          value={searchFilter()}
          onEdit={setSearchFilter}
          style={{ width: "100%", padding: "0.5rem" }}
        />
        <Flex alignItems="center" style={{ width: "fit-content" }}>
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
          <Show when={selected()} fallback={<NewGroup />}>
            <Show when={selected() !== "all" && canEdit(selected()!)}>
              <button
                class="blue"
                onClick={toggleEditing}
                style={{ height: "100%" }}
              >
                <Icon type="edit" />
              </button>
              <AddServerToGroup groupId={selected()!} />
            </Show>
          </Show>
        </Flex>
      </Grid>
      <Show
        when={selected()}
        fallback={
          <Grid gridTemplateColumns={isSemiMobile() ? "1fr" : "1fr 1fr"}>
            <GroupButton id="all" setSelected={setSelected} />
            <For each={groupIDs()}>
              {(id) => <GroupButton id={id} setSelected={setSelected} />}
            </For>
          </Grid>
        }
      >
        <Group
          id={selected()!}
          searchFilter={searchFilter()}
          exit={() => {
            setSelected(null);
            setEditing(false);
          }}
          editing={editing()}
        />
      </Show>
    </Grid>
  );
};

export default Groups;

const GroupButton: Component<{
  id: string;
  setSelected: (s: string) => void;
}> = (p) => {
  const { isSemiMobile } = useAppDimensions();
  const { user, user_id } = useUser();
  const { groups, servers } = useAppState();
  const isAll = () => p.id === "all";
  const name = () => {
    if (isAll()) {
      return "all";
    }
    return groups.get(p.id)?.name;
  };
  const serverCount = () => {
    if (isAll()) {
      return servers.ids()?.length || 0;
    }
    return (
      groups.get(p.id)?.servers.filter((server_id) => servers.get(server_id))
        .length || 0
    );
  };
  const canEdit = () =>
    user().admin ||
    groups.get(p.id)?.permissions?.[user_id()] === PermissionLevel.Update;
  return (
    <Flex
      class="card light hover shadow"
      style={{
        "grid-column": isAll() && !isSemiMobile() ? "span 2" : undefined,
        "justify-content": "space-between",
        cursor: "pointer",
      }}
      onClick={() => p.setSelected(p.id)}
    >
      <h1>{name()}</h1>
      <Flex alignItems="center">
        <Flex gap="0.4rem">
          <div>{serverCount()}</div>
          <div class="dimmed">{`server${serverCount() > 1 ? "s" : ""}`}</div>
        </Flex>
        <Show when={canEdit() && !isAll()}>
          <ConfirmButton
            class="red"
            onConfirm={() => client.delete_group(p.id)}
          >
            <Icon type="trash" />
          </ConfirmButton>
        </Show>
      </Flex>
    </Flex>
  );
};

const Group: Component<{
  id: string;
  searchFilter: string;
  editing: boolean;
  exit: () => void;
}> = (p) => {
  const { user, user_id } = useUser();
  const { groups, servers } = useAppState();
  const { server_sorter } = useTreeState();
  const group = () => groups.get(p.id);
  const canEdit = () =>
    user().admin ||
    group()?.permissions?.[user_id()] === PermissionLevel.Update;
  const serverIDs = createMemo(() => {
    if (servers.loaded()) {
      const filters = p.searchFilter
        .split(" ")
        .filter((term) => term.length > 0)
        .map((term) => term.toLowerCase());
      const serverIds = (
        p.id === "all"
          ? servers.ids()
          : groups
              .get(p.id)
              ?.servers.filter((server_id) => servers.get(server_id))
      )?.sort(server_sorter());
      return serverIds
        ?.filter((id) => {
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
  useWindowKeyDown((e) => {
    if (e.key === "ArrowLeft" || e.key === "Escape") {
      p.exit();
    }
  });
  return (
    <For each={serverIDs()}>
      {(id) => (
        <Flex alignItems="center">
          <Server id={id} />
          <Show when={canEdit() && p.editing}>
            <ConfirmButton
              class="red"
              onConfirm={() => {
                client.update_group({
                  ...group()!,
                  servers: group()!.servers.filter((member) => member !== id),
                });
              }}
            >
              <Icon type="minus" />
            </ConfirmButton>
          </Show>
        </Flex>
      )}
    </For>
  );
};

const AddServerToGroup: Component<{ groupId: string }> = (p) => {
  const { user, user_id } = useUser();
  const { groups, servers, ungroupedServerIds } = useAppState();
  const [showAdd, setShowAdd] = createSignal(false);
  const group = () => groups.get(p.groupId);
  const canEdit = () =>
    user().admin ||
    group()?.permissions?.[user_id()] === PermissionLevel.Update;
  return (
    <Show
      when={canEdit() && ((group() && ungroupedServerIds()?.length) || 0 > 0)}
    >
      <Menu
        show={showAdd()}
        close={(e) => {
          e.stopPropagation();
          setShowAdd(false);
        }}
        position="bottom right"
        target={
          <button
            class="green"
            onClick={(e) => {
              e.stopPropagation();
              setShowAdd(true);
            }}
          >
            <Icon type="plus" />
          </button>
        }
        menuStyle={{ gap: "0.5rem" }}
        content={
          <>
            {/* <Input placeholder="search" style={{ width: "10rem" }} /> */}
            <For each={ungroupedServerIds()!}>
              {(server_id) => {
                const server = () => servers.get(server_id)!;
                return (
                  <ConfirmButton
                    class="lightgrey"
                    style={{ width: "100%" }}
                    onConfirm={() =>
                      client.update_group({
                        ...group()!,
                        servers: [...group()!.servers, server_id],
                      })
                    }
                  >
                    {server().server.name}
                  </ConfirmButton>
                );
              }}
            </For>
          </>
        }
      />
    </Show>
  );
};
