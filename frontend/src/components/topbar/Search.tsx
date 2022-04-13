import { Component, createMemo, createSignal, For, Show } from "solid-js";
import { useAppState } from "../../state/StateProvider";
import { combineClasses } from "../../util/helpers";
import Input from "../util/Input";
import Menu from "../util/menu/Menu";
import s from "./topbar.module.scss";

const Search: Component<{}> = (p) => {
  const { deployments, builds, servers, selected } = useAppState();
  const [search, setSearch] = createSignal("");
  const [highlighted, setHighlighted] = createSignal(0);
  const filteredDeployments = createMemo(() =>
    search().length > 0
      ? deployments.filterArray((deployment) =>
          deployment.name.toLowerCase().includes(search().toLowerCase())
        )!
      : undefined
  );
  const filteredBuilds = createMemo(() =>
    search().length > 0
      ? builds.filterArray((build) =>
          build.name.toLowerCase().includes(search().toLowerCase())
        )!
      : undefined
  );
  const filteredServers = createMemo(() =>
    search().length > 0
      ? servers.filterArray((server) =>
          server.name.toLowerCase().includes(search().toLowerCase())
        )!
      : undefined
  );
  return (
    <Menu
      show={search().length > 0}
      close={() => setSearch("")}
      position="bottom right"
      target={
        <Input
          class={s.Search}
          placeholder="search"
          value={search()}
          onEdit={(val) => {
            setSearch(val);
            setHighlighted(0);
          }}
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
                setSearch("");
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
                setSearch("");
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
                setSearch("");
              }
            } else if (e.key === "Escape") {
              setSearch("");
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
                  setSearch("");
                }}
              >
                {deployment.name}
                <div style={{ opacity: 0.6, "font-size": "0.9rem" }}>
                  deployment
                </div>
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
                  setSearch("");
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
                  setSearch("");
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
