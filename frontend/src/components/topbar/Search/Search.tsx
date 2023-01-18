import { Component, For, JSX, onMount, Show } from "solid-js";
import s from "../topbar.module.scss";
import Input from "../../shared/Input";
import Menu from "../../shared/menu/Menu";
import { useSearchState } from "./Provider";
import {
  combineClasses,
  deploymentStateClass,
  getId,
} from "../../../util/helpers";
import { useAppState } from "../../../state/StateProvider";
import Flex from "../../shared/layout/Flex";
import Icon from "../../shared/Icon";
import { useWindowKeyDown } from "../../../util/hooks";
import Circle from "../../shared/Circle";
import { ControlledTabs } from "../../shared/tabs/Tabs";
import { useAppDimensions } from "../../../state/DimensionProvider";
import Grid from "../../shared/layout/Grid";
import { A, useNavigate } from "@solidjs/router";
import { ServerStatus } from "../../../types";

const mobileStyle: JSX.CSSProperties = {
  // position: "fixed",
  // top: inPx(44),
  // left: "1rem",
  width: "calc(100vw - 2rem)",
};

export const Search: Component<{}> = (p) => {
  const { isSemiMobile } = useAppDimensions();
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
      menuClass={s.SearchMenu}
      menuStyle={{
        gap: "0.5rem",
        ...(isSemiMobile() ? mobileStyle : {}),
      }}
      backgroundColor={isSemiMobile() ? "rgba(0,0,0,0.6)" : "rgba(0,0,0,0.4)"}
      target={
        <Show
          when={!isSemiMobile()}
          fallback={
            <button class="grey" onClick={() => open.set(true)}>
              <Icon type="search" width="1.15rem" />
            </button>
          }
        >
          <Input
            ref={inputRef}
            class={s.SearchInput}
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
  const { isSemiMobile } = useAppDimensions();
  let inputRef: HTMLInputElement | undefined;
  onMount(() => {
    if (isSemiMobile()) {
      inputRef?.focus();
    }
  });
  return (
    <>
      <Show when={isSemiMobile()}>
        <Input
          ref={() => inputRef}
          class={s.SearchInput}
          placeholder="search"
          value={search.value()}
          onEdit={input.onEdit}
          onKeyDown={input.onKeyDown(inputRef)}
          style={{ width: isSemiMobile() ? "100%" : undefined }}
        />
      </Show>
      <ControlledTabs
        selected={tab.selected}
        set={tab.set}
        containerStyle={{ width: isSemiMobile() ? "100%" : "30rem", gap: "0.5rem" }}
        tabs={[
          {
            title: "deployments",
            element: () => <Deployments close={p.close} />,
          },
          {
            title: "builds",
            element: () => <Builds close={p.close} />,
          },
          {
            title: "servers",
            element: () => <Servers close={p.close} />,
          },
        ]}
      />
    </>
  );
};

const Deployments: Component<{ close: () => void }> = (p) => {
  const { servers, deployments } = useAppState();
  const { highlighted, filteredDeployments } = useSearchState();
  return (
    <Grid class="scroller" gap="0.5rem" style={{ "max-height": "70vh" }}>
      <Show when={filteredDeployments()?.length === 0}>
        <Flex alignItems="center" justifyContent="center">
          no results
        </Flex>
      </Show>
      <For each={filteredDeployments()}>
        {(deployment, index) => (
          <A
            href={`/deployment/${getId(deployment.deployment)}`}
            class={combineClasses(
              s.SearchItem,
              index() === highlighted.value() && "selected",
              "grey"
            )}
            onClick={() => p.close()}
          >
            <Grid gap="0rem">
              <Flex alignItems="center">{deployment.deployment.name} </Flex>
              <Flex
                alignItems="center"
                gap="0.2rem"
                style={{ opacity: 0.6, "font-size": "0.9rem" }}
              >
                {servers.get(deployment.deployment.server_id)?.server.name}
                <Icon type="caret-right" width="0.7rem" />
                deployment
              </Flex>
            </Grid>
            <Circle
              class={deploymentStateClass(
                deployments.state(getId(deployment.deployment))
              )}
              size={1.25}
            />
          </A>
        )}
      </For>
    </Grid>
  );
};

const Builds: Component<{ close: () => void }> = (p) => {
  const { servers } = useAppState();
  const { highlighted, filteredBuilds } = useSearchState();
  return (
    <Grid class="scroller" gap="0.5rem" style={{ "max-height": "70vh" }}>
      <Show when={filteredBuilds()?.length === 0}>
        <Flex alignItems="center" justifyContent="center">
          no results
        </Flex>
      </Show>
      <For each={filteredBuilds()}>
        {(build, index) => (
          <A
            href={`/build/${getId(build)}`}
            class={combineClasses(
              s.SearchItem,
              index() === highlighted.value() && "selected",
              "grey"
            )}
            onClick={() => p.close()}
          >
            <Grid gap="0rem">
              <Flex alignItems="center">{build.name} </Flex>
              <Flex
                alignItems="center"
                gap="0.2rem"
                style={{ opacity: 0.6, "font-size": "0.9rem" }}
              >
                {servers.get(build.server_id)?.server.name}
                <Icon type="caret-right" width="0.7rem" />
                build
              </Flex>
            </Grid>
          </A>
        )}
      </For>
    </Grid>
  );
};

const Servers: Component<{ close: () => void }> = (p) => {
  // const navigate = useNavigate();
  const { highlighted, filteredServers } = useSearchState();
  return (
    <Grid
      class="scroller"
      gap="0.5rem"
      style={{ "max-height": "70vh" }}
    >
      <Show when={filteredServers()?.length === 0}>
        <Flex alignItems="center" justifyContent="center">
          no results
        </Flex>
      </Show>
      <For each={filteredServers()}>
        {(server, index) => (
          <A
            href={`/server/${getId(server.server)}`}
            class={combineClasses(
              s.SearchItem,
              index() === highlighted.value() && "selected",
              "grey"
            )}
            onClick={() => {
              // navigate(`/server/${getId(server.server)}`);
              p.close();
            }}
          >
            <Grid gap="0rem">
              <Flex alignItems="center">{server.server.name}</Flex>
              <Flex
                alignItems="center"
                gap="0.2rem"
                style={{ opacity: 0.6, "font-size": "0.9rem" }}
              >
                server
                <Show when={server.server.region}>
                  <Icon type="caret-right" width="0.7rem" />
                  {server.server.region}
                </Show>
              </Flex>
            </Grid>
            <div
              class={
                server.status === ServerStatus.Ok
                  ? "green"
                  : server.status === ServerStatus.NotOk
                  ? "red"
                  : "blue"
              }
              style={{
                padding: "0.25rem",
                "border-radius": ".35rem",
                transition: "background-color 125ms ease-in-out",
                "font-size": "0.8rem",
              }}
            >
              {server.status.replaceAll("_", " ").toUpperCase()}
            </div>
          </A>
        )}
      </For>
    </Grid>
  );
};
