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
import Grid from "../../util/layout/Grid";
import Button from "../../util/Button";

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
  return (
    <Menu
      show={open.value()}
      close={() => open.close(inputRef)}
      position="bottom center"
      menuStyle={{
        gap: "0.5rem",
        ...(isMobile() ? mobileStyle : {}),
      }}
      backgroundColor={isMobile() ? "rgba(0,0,0,0.6)" : undefined}
      target={
        <Show
          when={!isMobile()}
          fallback={
            <Button class="grey" onClick={() => open.set(true)}>
              <Icon type="search" width="1.15rem" />
            </Button>
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
        containerStyle={{ width: isMobile() ? "100%" : "30rem", gap: "0.5rem" }}
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
    <Grid
      class="scroller"
      gap="0.5rem"
      style={{ "max-height": "70vh", "padding-right": "0.5rem" }}
    >
      <Show when={filteredDeployments()?.length === 0}>no results</Show>
      <For each={filteredDeployments()}>
        {(deployment, index) => (
          <Button
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
            <Grid gap="0rem">
              <Flex alignItems="center">{deployment.name} </Flex>
              <Flex
                alignItems="center"
                gap="0.2rem"
                style={{ opacity: 0.6, "font-size": "0.9rem" }}
              >
                {servers.get(deployment.serverID!)?.name}
                <Icon type="caret-right" width="0.7rem" />
                deployment
              </Flex>
            </Grid>
            <Circle
              class={deploymentStatusClass(deployments.state(deployment._id!))}
              size={1.25}
            />
          </Button>
        )}
      </For>
    </Grid>
  );
};

const Builds: Component<{ close: () => void }> = (p) => {
  const { selected } = useAppState();
  const { highlighted, filteredBuilds } = useSearchState();
  return (
    <Grid
      class="scroller"
      gap="0.5rem"
      style={{ "max-height": "70vh", "padding-right": "0.5rem" }}
    >
      <Show when={filteredBuilds()?.length === 0}>no results</Show>
      <For each={filteredBuilds()}>
        {(build, index) => (
          <Button
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
            <Grid gap="0rem">
              {build.name}
              <div style={{ opacity: 0.6, "font-size": "0.9rem" }}>build</div>
            </Grid>
          </Button>
        )}
      </For>
    </Grid>
  );
};

const Servers: Component<{ close: () => void }> = (p) => {
  const { selected } = useAppState();
  const { highlighted, filteredServers } = useSearchState();
  return (
    <Grid
      class="scroller"
      gap="0.5rem"
      style={{ "max-height": "70vh", "padding-right": "0.5rem" }}
    >
      <Show when={filteredServers()?.length === 0}>no results</Show>
      <For each={filteredServers()}>
        {(server, index) => (
          <Button
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
            <Grid gap="0rem">
              <Flex alignItems="center">
                <div>{server.name}</div>
              </Flex>
              <Flex
                alignItems="center"
                gap="0.2rem"
                style={{ opacity: 0.6, "font-size": "0.9rem" }}
              >
                server
                <Show when={server.region}>
                  <Icon type="caret-right" width="0.7rem" />
                  {server.region}
                </Show>
              </Flex>
            </Grid>
            <div
              class={server.status === "OK" ? "green" : "red"}
              style={{
                padding: "0.25rem",
                "border-radius": ".35rem",
                transition: "background-color 125ms ease-in-out",
                "font-size": "0.8rem",
              }}
            >
              {server.status === "OK" ? "OK" : "NOT OK"}
            </div>
          </Button>
        )}
      </For>
    </Grid>
  );
};
