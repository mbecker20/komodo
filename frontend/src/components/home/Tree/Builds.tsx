import { A } from "@solidjs/router";
import { Component, createMemo, createSignal, For, Show } from "solid-js";
import { client } from "../../..";
import { useAppDimensions } from "../../../state/DimensionProvider";
import { useAppState } from "../../../state/StateProvider";
import { useUser } from "../../../state/UserProvider";
import { PermissionLevel } from "../../../types";
import { getId, readableMonitorTimestamp } from "../../../util/helpers";
import {
  ActionStateProvider,
  useActionStates,
} from "../../build/ActionStateProvider";
import { NewBuild } from "../../New";
import ConfirmButton from "../../shared/ConfirmButton";
import Icon from "../../shared/Icon";
import Input from "../../shared/Input";
import Flex from "../../shared/layout/Flex";
import Grid from "../../shared/layout/Grid";
import Loading from "../../shared/loading/Loading";
import Selector from "../../shared/menu/Selector";
import { TreeSortType, TREE_SORTS, useTreeState } from "./Provider";

const Builds: Component<{}> = (p) => {
  const { isSemiMobile } = useAppDimensions();
  const { user } = useUser();
  const { builds } = useAppState();
  const { sort, setSort, build_sorter } = useTreeState();
  const [buildFilter, setBuildFilter] = createSignal("");
  const buildIDs = createMemo(() => {
    if (builds.loaded()) {
      const filters = buildFilter()
        .split(" ")
        .filter((term) => term.length > 0)
        .map((term) => term.toLowerCase());
      return builds
        .ids()!
        .filter((id) => {
          const name = builds.get(id)!.name;
          for (const term of filters) {
            if (!name.includes(term)) {
              return false;
            }
          }
          return true;
        })
        .sort(build_sorter());
    } else {
      return undefined;
    }
  });
  return (
    <Grid>
      <Grid gridTemplateColumns="1fr auto auto">
        <Input
          placeholder="filter builds"
          value={buildFilter()}
          onEdit={setBuildFilter}
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
        <Show when={user().admin || user().create_build_permissions}>
          <NewBuild />
        </Show>
      </Grid>
      <Grid gridTemplateColumns={isSemiMobile() ? "1fr" : "1fr 1fr"}>
        <For each={buildIDs()}>
          {(id) => (
            <ActionStateProvider build_id={id}>
              <Build id={id} />
            </ActionStateProvider>
          )}
        </For>
      </Grid>
    </Grid>
  );
};

const Build: Component<{ id: string }> = (p) => {
  const { isMobile } = useAppDimensions();
  const { user } = useUser();
  const { builds } = useAppState();
  const build = () => builds.get(p.id)!;
  // const server = () =>
  //   build().server_id ? servers.get(build().server_id!) : undefined;
  const version = () => {
    return `v${build().version.major}.${build().version.minor}.${
      build().version.patch
    }`;
  };
  const lastBuiltAt = () => {
    if (
      build().last_built_at === undefined ||
      build().last_built_at?.length === 0 ||
      build().last_built_at === "never"
    ) {
      return "not built";
    } else {
      return readableMonitorTimestamp(build().last_built_at!);
    }
  };
  const actions = useActionStates();
  const userCanExecute = () =>
    user().admin ||
    build().permissions![getId(user())] === PermissionLevel.Execute ||
    build().permissions![getId(user())] === PermissionLevel.Update;
  // const isAwsBuild = () => build().aws_config ? true : false;
  return (
    <A
      href={`/build/${p.id}`}
      class="card light hover shadow full-width"
      style={{
        "justify-content": "space-between",
      }}
    >
      <h1 style={{ "font-size": "1.25rem" }}>{build().name}</h1>
      <Flex alignItems="center">
        {/* <Show when={server()}>
          <A
            href={`/server/${build().server_id!}`}
            style={{ padding: 0, opacity: 0.7 }}
          >
            <div class="text-hover">{server()?.server.name}</div>
          </A>
        </Show>
        <Show when={isAwsBuild()}>
          <div style={{ opacity: 0.7 }}>aws build</div>
        </Show> */}
        <h2>{version()}</h2>
        <Show when={!isMobile()}>
          <div style={{ opacity: 0.7 }}>{lastBuiltAt()}</div>
        </Show>
        <Show when={userCanExecute()}>
          <Show
            when={!actions.building}
            fallback={
              <button
                class="green"
                onClick={(e) => {
                  e.stopPropagation();
                  e.preventDefault();
                }}
              >
                <Loading type="spinner" />
              </button>
            }
          >
            <ConfirmButton
              class="green"
              onConfirm={() => {
                client.build(p.id);
              }}
            >
              <Icon type="build" width="0.9rem" />
            </ConfirmButton>
          </Show>
        </Show>
      </Flex>
    </A>
  );
};

export default Builds;
