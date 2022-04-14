import {
  Component,
  createMemo,
  createSignal,
  For,
  onCleanup,
  Show,
} from "solid-js";
import { useAppState } from "../../state/StateProvider";
import { combineClasses } from "../../util/helpers";
import { useWindowKeyDown } from "../../util/hooks";
import Icon from "../util/Icon";
import Input from "../util/Input";
import Flex from "../util/layout/Flex";
import Menu from "../util/menu/Menu";
import s from "./topbar.module.scss";

const Search: Component<{}> = (p) => {
  const { deployments, builds, servers, selected } = useAppState();
  const [search, setSearch] = createSignal("");
  const [open, setOpen] = createSignal(false);
  const close = () => {
    inputRef?.blur();
    setSearch("");
    setOpen(false);
  };
  const [highlighted, setHighlighted] = createSignal(0);
  const filteredDeployments = createMemo(
    () =>
      deployments.filterArray((deployment) =>
        deployment.name.toLowerCase().includes(search().toLowerCase())
      )!
  );
  const filteredBuilds = createMemo(
    () =>
      builds.filterArray((build) =>
        build.name.toLowerCase().includes(search().toLowerCase())
      )!
  );
  const filteredServers = createMemo(
    () =>
      servers.filterArray((server) =>
        server.name.toLowerCase().includes(search().toLowerCase())
      )!
  );
  let inputRef: HTMLInputElement | undefined;

  useWindowKeyDown((e) => {
    if (e.key === "S" && e.shiftKey) {
      setOpen(true);
      setTimeout(() => inputRef?.focus(), 200);
    }
  });
  return (
    <Menu
      show={open()}
      close={close}
      position="bottom right"
      menuClass="scroller"
      menuStyle={{
        "max-height": "80vh",
      }}
      target={
        <Input
          ref={inputRef}
          class={s.Search}
          placeholder="search"
          value={search()}
          onEdit={(val) => {
            setSearch(val);
            setHighlighted(0);
          }}
          onFocus={() => setOpen(true)}
          onBlur={close}
          onKeyDown={(e: any) => {
            if (e.key === "ArrowDown") {
              e.preventDefault();
              setHighlighted((h) =>
                Math.min(
                  h + 1,
                  (filteredDeployments()?.length || 0) +
                    (filteredBuilds()?.length || 0) +
                    (filteredServers()?.length || 0) -
                    1
                )
              );
            } else if (e.key === "ArrowUp") {
              e.preventDefault();
              setHighlighted((h) => Math.max(0, h - 1));
            } else if (e.key === "Enter") {
              if (highlighted() < (filteredDeployments()?.length || 0)) {
                selected.set(
                  filteredDeployments()![highlighted()]._id!,
                  "deployment"
                );
                close();
              } else if (
                highlighted() <
                (filteredDeployments()?.length || 0) +
                  (filteredBuilds()?.length || 0)
              ) {
                selected.set(
                  filteredBuilds()![
                    highlighted() - (filteredDeployments()?.length || 0)
                  ]._id!,
                  "build"
                );
                close();
              } else if (
                highlighted() <
                (filteredDeployments()?.length || 0) +
                  (filteredBuilds()?.length || 0) +
                  (filteredServers()?.length || 0)
              ) {
                selected.set(
                  filteredServers()![
                    highlighted() -
                      (filteredDeployments()?.length || 0) -
                      (filteredBuilds()?.length || 0)
                  ]._id!,
                  "server"
                );
                close();
              }
            } else if (e.key === "Escape") {
              close();
            }
          }}
        />
      }
      content={
        <>
          <Show
            when={
              filteredDeployments()?.length === 0 &&
              filteredBuilds()?.length === 0 &&
              filteredServers()?.length === 0
            }
          >
            no results
          </Show>
          <For each={filteredDeployments()}>
            {(deployment, index) => (
              <button
                class={combineClasses(
                  s.SearchItem,
                  index() === highlighted() && "selected",
                  "grey"
                )}
                onClick={() => {
                  selected.set(deployment._id!, "deployment");
                  close();
                }}
              >
                {deployment.name}
                <Flex
                  alignItems="center"
                  gap="0.2rem"
                  style={{ opacity: 0.6, "font-size": "0.9rem" }}
                >
                  {servers.get(deployment.serverID!)?.name}
                  <Icon type="caret-right" width="0.7rem" />
                  deployment
                </Flex>
              </button>
            )}
          </For>
          {/* <div class="divider" /> */}
          <For each={filteredBuilds()}>
            {(build, index) => (
              <button
                class={combineClasses(
                  s.SearchItem,
                  index() + (filteredDeployments()?.length || 0) ===
                    highlighted() && "selected",
                  "grey"
                )}
                onClick={() => {
                  selected.set(build._id!, "build");
                  close();
                }}
              >
                {build.name}
                <div style={{ opacity: 0.6, "font-size": "0.9rem" }}>build</div>
              </button>
            )}
          </For>
          {/* <div class="divider" /> */}
          <For each={filteredServers()}>
            {(server, index) => (
              <button
                class={combineClasses(
                  s.SearchItem,
                  index() +
                    (filteredDeployments()?.length || 0) +
                    (filteredBuilds()?.length || 0) ===
                    highlighted() && "selected",
                  "grey"
                )}
                onClick={() => {
                  selected.set(server._id!, "server");
                  close();
                }}
              >
                {server.name}
                <div style={{ opacity: 0.6, "font-size": "0.9rem" }}>
                  server
                </div>
              </button>
            )}
          </For>
        </>
      }
    />
  );
};

export default Search;
