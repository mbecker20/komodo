import { Component, createMemo, createSignal, For, Show } from "solid-js";
import { useAppState } from "../../../state/StateProvider";
import { useLocalStorageToggle } from "../../../util/hooks";
import Icon from "../../shared/Icon";
import Input from "../../shared/Input";
import Flex from "../../shared/layout/Flex";
import Grid from "../../shared/layout/Grid";
import { NewGroup } from "../../New";
import s from "../home.module.scss";
import { combineClasses } from "../../../util/helpers";
import Server from "./Server";
import Menu from "../../shared/menu/Menu";
import { client } from "../../..";
import ConfirmButton from "../../shared/ConfirmButton";
import { TreeSortType, TREE_SORTS, useTreeState } from "./Provider";
import Selector from "../../shared/menu/Selector";

const Groups: Component<{}> = (p) => {
  const { groups } = useAppState();
  const [groupFilter, setGroupFilter] = createSignal("");
  const { sort, setSort, group_sorter } = useTreeState();
  const groupIDs = createMemo(() => {
    if (groups.loaded()) {
      const filters = groupFilter()
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
  return (
    <Grid style={{ height: "fit-content" }}>
      <Grid gridTemplateColumns="1fr auto auto">
        <Input
          placeholder="filter groups"
          value={groupFilter()}
          onEdit={setGroupFilter}
          style={{ width: "100%", padding: "0.5rem" }}
        />
        <Selector
          selected={sort()}
          items={TREE_SORTS as any as string[]}
          onSelect={(mode) => setSort(mode as TreeSortType)}
          position="bottom right"
          targetClass="blue"
          targetStyle={{ height: "100%" }}
          containerStyle={{ height: "100%" }}
        />
        <NewGroup />
      </Grid>
      <For each={groupIDs()}>{(id) => <Group id={id} />}</For>
    </Grid>
  );
};

export default Groups;

const Group: Component<{ id: string }> = (p) => {
  const { groups, servers, ungroupedServerIds } = useAppState();
  const { server_sorter } = useTreeState();
  const group = () => groups.get(p.id);
  const serverIDs = () => group()?.servers.sort(server_sorter());
  const [open, toggleOpen] = useLocalStorageToggle(p.id + "-group-homeopen-v1");
  const [showAdd, setShowAdd] = createSignal(false);
  const [edit, setEdit] = createSignal(false);
  return (
    <Show when={group()}>
      <button
        class={combineClasses(s.ServerButton, "shadow")}
        onClick={toggleOpen}
      >
        <Flex alignItems="center">
          <Icon type={open() ? "chevron-up" : "chevron-down"} width="1rem" />
          <h1 style={{ "font-size": "1.25rem" }}>{group()?.name}</h1>
        </Flex>
        <Flex alignItems="center">
          <h2>
            {serverIDs()!.length} server{serverIDs()!.length > 1 ? "s" : ""}
          </h2>
          <Show when={open()}>
            <button
              class="blue"
              onClick={(e) => {
                e.stopPropagation();
                setEdit((edit) => !edit);
              }}
            >
              <Icon type="edit" />
            </button>
            <Show when={ungroupedServerIds()?.length || 0 > 0}>
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
          </Show>
          <ConfirmButton
            class="red"
            onConfirm={() => client.delete_group(p.id)}
          >
            <Icon type="trash" />
          </ConfirmButton>
        </Flex>
      </button>
      <Show when={serverIDs() && open()}>
        <Grid
          placeItems="center"
          gridTemplateColumns="1fr auto 1fr"
          style={{ width: "100%" }}
        >
          <div
            class="lightgrey"
            style={{ opacity: 0.7, width: "100%", height: "3px" }}
          />
          <div style={{ opacity: 0.7 }}>servers</div>
          <div
            class="lightgrey"
            style={{ opacity: 0.7, width: "100%", height: "3px" }}
          />
        </Grid>
        <For each={serverIDs()}>
          {(id) => {
            return (
              <Flex alignItems="center">
                <Server id={id} />
                <Show when={edit()}>
                  <ConfirmButton
                    class="red"
                    onConfirm={() => {
                      client.update_group({
                        ...group()!,
                        servers: group()!.servers.filter(
                          (member) => member !== id
                        ),
                      });
                    }}
                  >
                    <Icon type="minus" />
                  </ConfirmButton>
                </Show>
              </Flex>
            );
          }}
        </For>
        <Grid
          placeItems="center"
          gridTemplateColumns="1fr auto 1fr"
          style={{ width: "100%" }}
        >
          <div
            class="lightgrey"
            style={{ opacity: 0.7, width: "100%", height: "3px" }}
          />
          <div style={{ opacity: 0.7 }}>end</div>
          <div
            class="lightgrey"
            style={{ opacity: 0.7, width: "100%", height: "3px" }}
          />
        </Grid>
      </Show>
    </Show>
  );
};
