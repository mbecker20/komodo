import { Component, For, JSX, onMount, Show } from "solid-js";
import s from "../topbar.module.scss";
import Input from "../../util/Input";
import Menu from "../../util/menu/Menu";
import { useSearchState } from "./Provider";
import {
  combineClasses,
  deploymentStatusClass,
  inPx,
} from "../../../util/helpers";
import { useAppState } from "../../../state/StateProvider";
import Flex from "../../util/layout/Flex";
import Icon from "../../util/Icon";
import { useWindowKeyDown } from "../../../util/hooks";
import Circle from "../../util/Circle";
import { ControlledTabs } from "../../util/tabs/Tabs";
import { useAppDimensions } from "../../../state/DimensionProvider";

const mobileStyle: JSX.CSSProperties = {
  position: "fixed",
  top: inPx(44),
  left: "1rem",
  width: "calc(100vw - 2rem)",
};

export const Search: Component<{}> = (p) => {
  const { isMobile } = useAppDimensions();
  const { search, open, input } = useSearchState();
  let inputRef: HTMLInputElement | undefined;
  useWindowKeyDown((e) => {
    if (e.key === "S" && e.shiftKey) {
      open.set(true);
      setTimeout(() => inputRef?.focus(), 200);
    }
  });
  const style = () => {
    return {
      "max-height": "80vh",
      "padding-right": "0.5rem",
      gap: "0.5rem",
      ...(isMobile() ? mobileStyle : {}),
    };
  };
  return (
    <Menu
      show={open.value()}
      close={() => open.close(inputRef)}
      position="bottom right"
      menuClass="scroller"
      menuStyle={style()}
      backgroundColor={isMobile() ? "rgba(0,0,0,0.6)" : undefined}
      target={
        <Show
          when={!isMobile()}
          fallback={
            <button class="grey" onClick={() => open.set(true)}>
              <Icon type="search" width="1.15rem" />
            </button>
          }
        >
          <Input
            ref={inputRef}
            class={s.Search}
            placeholder="search"
            value={search.value()}
            onEdit={input.onEdit}
            onFocus={() => open.set(true)}
            onKeyDown={input.onKeyDown(inputRef)}
          />
        </Show>
      }
      content={<SearchMenu close={() => open.close(inputRef)} />}
    />
  );
};

const SearchMenu: Component<{ close: () => void }> = (p) => {
  const { tab, input, search } = useSearchState();
  const { isMobile } = useAppDimensions();
  let inputRef: HTMLInputElement | undefined;
  onMount(() => {
    if (isMobile()) {
      inputRef?.focus();
    }
  });
  return (
    <>
      <Show when={isMobile()}>
        <Input
          ref={inputRef}
          class={s.Search}
          placeholder="search"
          value={search.value()}
          onEdit={input.onEdit}
          onKeyDown={input.onKeyDown(inputRef)}
          style={{ width: isMobile() ? "100%" : undefined }}
        />
      </Show>
      <ControlledTabs
        selected={tab.selected}
        set={tab.set}
        containerStyle={{ width: isMobile() ? "100%" : "350px" }}
        tabs={[
          {
            title: "deployments",
            element: <Deployments close={p.close} />,
          },
          {
            title: "builds",
            element: <Builds close={p.close} />,
          },
          {
            title: "servers",
            element: <Servers close={p.close} />,
          },
        ]}
      />
    </>
  );
};

const Deployments: Component<{ close: () => void }> = (p) => {
  const { selected, servers, deployments } = useAppState();
  const { highlighted, filteredDeployments } = useSearchState();
  return (
    <>
      <Show when={filteredDeployments()?.length === 0}>no results</Show>
      <For each={filteredDeployments()}>
        {(deployment, index) => (
          <button
            class={combineClasses(
              s.SearchItem,
              index() === highlighted.value() && "selected",
              "grey"
            )}
            onClick={() => {
              selected.set(deployment._id!, "deployment");
              p.close();
            }}
          >
            <Flex alignItems="center">
              {deployment.name}{" "}
              <Circle
                class={deploymentStatusClass(
                  deployments.state(deployment._id!)
                )}
              />
            </Flex>
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
    </>
  );
};

const Builds: Component<{ close: () => void }> = (p) => {
  const { selected } = useAppState();
  const { highlighted, filteredBuilds } = useSearchState();
  return (
    <>
      <Show when={filteredBuilds()?.length === 0}>no results</Show>
      <For each={filteredBuilds()}>
        {(build, index) => (
          <button
            class={combineClasses(
              s.SearchItem,
              index() === highlighted.value() && "selected",
              "grey"
            )}
            onClick={() => {
              selected.set(build._id!, "build");
              p.close();
            }}
          >
            {build.name}
            <div style={{ opacity: 0.6, "font-size": "0.9rem" }}>build</div>
          </button>
        )}
      </For>
    </>
  );
};

const Servers: Component<{ close: () => void }> = (p) => {
  const { selected } = useAppState();
  const { highlighted, filteredServers } = useSearchState();
  return (
    <>
      <Show when={filteredServers()?.length === 0}>no results</Show>
      <For each={filteredServers()}>
        {(server, index) => (
          <button
            class={combineClasses(
              s.SearchItem,
              index() === highlighted.value() && "selected",
              "grey"
            )}
            onClick={() => {
              selected.set(server._id!, "server");
              p.close();
            }}
          >
            {server.name}
            <div style={{ opacity: 0.6, "font-size": "0.9rem" }}>server</div>
          </button>
        )}
      </For>
    </>
  );
};
