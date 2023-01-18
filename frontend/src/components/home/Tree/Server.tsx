import {
  Component,
  createMemo,
  createSignal,
  For,
  Show,
} from "solid-js";
import { useAppState } from "../../../state/StateProvider";
import { useUser } from "../../../state/UserProvider";
import { combineClasses, getId, readableStorageAmount } from "../../../util/helpers";
import { useLocalStorageToggle } from "../../../util/hooks";
import Icon from "../../shared/Icon";
import Flex from "../../shared/layout/Flex";
import Grid from "../../shared/layout/Grid";
import Deployment from "./Deployment";
import s from "../home.module.scss";
import { NewBuild, NewDeployment } from "./New";
import Loading from "../../shared/loading/Loading";
import { A } from "@solidjs/router";
import { PermissionLevel, ServerStatus } from "../../../types";
import { useAppDimensions } from "../../../state/DimensionProvider";
import Build from "./Build";
import SimpleTabs from "../../shared/tabs/SimpleTabs";
// import StatGraphs from "../../server/StatGraphs/StatGraphs";

const Server: Component<{ id: string }> = (p) => {
  const { servers, serverStats, deployments, builds } = useAppState();
  const { isSemiMobile } = useAppDimensions();
  const { user } = useUser();
  const [open, toggleOpen] = useLocalStorageToggle(p.id + "-homeopen");
  const server = () => servers.get(p.id);
  const deploymentIDs = createMemo(() => {
    return (deployments.loaded() &&
      deployments
        .ids()!
        .filter(
          (id) => deployments.get(id)?.deployment.server_id === p.id
        )) as string[];
  });
  const buildIDs = createMemo(() => {
    return (builds.loaded() &&
      builds
        .ids()!
        .filter((id) => builds.get(id)?.server_id === p.id)) as string[];
  });
  return (
    <Show when={server()}>
      <div class={combineClasses(s.Server, "shadow")}>
        <button
          class={combineClasses(s.ServerButton, "shadow")}
          onClick={toggleOpen}
        >
          <Flex>
            <Icon type={open() ? "chevron-up" : "chevron-down"} width="1rem" />
            <h1 style={{ "font-size": "1.25rem" }}>{server()?.server.name}</h1>
          </Flex>
          <Flex alignItems="center">
            <ServerStats id={p.id} />
            <A
              href={`/server/${p.id}`}
              class={
                server()?.server.enabled
                  ? server()?.status === ServerStatus.Ok
                    ? "green"
                    : "red"
                  : "blue"
              }
              style={{
                padding: "0.25rem",
                "border-radius": ".35rem",
                transition: "background-color 125ms ease-in-out",
              }}
              onClick={(e) => {
                e.stopPropagation();
              }}
            >
              {server()?.status.replaceAll("_", " ").toUpperCase()}
            </A>
          </Flex>
        </button>
        <Show when={open()}>
          <SimpleTabs
            containerClass="card shadow"
            localStorageKey={`${p.id}-home-tab`}
            tabs={[
              {
                title: "deployments",
                element: () => (
                  <Grid
                    gap=".5rem"
                    class={combineClasses(
                      s.Deployments,
                      open() ? s.Enter : s.Exit
                    )}
                    gridTemplateColumns={isSemiMobile() ? "1fr" : "1fr 1fr"}
                  >
                    <For each={deploymentIDs()}>
                      {(id) => <Deployment id={id} />}
                    </For>
                    <Show
                      when={
                        user().admin ||
                        server()?.server.permissions![getId(user())] ===
                          PermissionLevel.Update
                      }
                    >
                      <NewDeployment serverID={p.id} />
                    </Show>
                  </Grid>
                ),
              },
              {
                title: "builds",
                element: () => (
                  <Grid
                    gap=".5rem"
                    class={combineClasses(
                      s.Deployments,
                      open() ? s.Enter : s.Exit
                    )}
                    gridTemplateColumns={isSemiMobile() ? "1fr" : "1fr 1fr"}
                  >
                    <For each={buildIDs()}>{(id) => <Build id={id} />}</For>
                    <Show
                      when={
                        user().admin ||
                        server()?.server.permissions![getId(user())] ===
                          PermissionLevel.Update
                      }
                    >
                      <NewBuild serverID={p.id} />
                    </Show>
                  </Grid>
                ),
              },
            ]}
          />
        </Show>
      </div>
    </Show>
  );
};

export default Server;

const ServerStats: Component<{ id: string }> = (p) => {
  const { servers, serverStats } = useAppState();
  const { isMobile, isSemiMobile } = useAppDimensions();
  const server = () => servers.get(p.id);
  const [reloading, setReloading] = createSignal(false);
  const stats = () => serverStats.get(p.id);
  const reloadStats = async () => {
    setReloading(true);
    await serverStats.load(p.id);
    setReloading(false);
  };
  return (
    <Show when={!isMobile() && server()?.status === ServerStatus.Ok}>
      <Show when={stats()} fallback={<Loading type="three-dot" />}>
        <Grid
          // gap="0.25rem"
          placeItems="center"
          gridTemplateColumns={
            isSemiMobile() ? "100px 100px 100px" : "90px 160px 160px"
          }
        >
          <Flex
            style={{
              width: "100%",
              "box-sizing": "border-box",
            }}
            gap="0.5rem"
            alignItems="center"
            justifyContent="center"
          >
            <div style={{ opacity: 0.7 }}>cpu:</div>
            <h2>{stats()!.cpu_perc.toFixed(1)}%</h2>
          </Flex>
          <Flex
            style={{
              width: "100%",
              "box-sizing": "border-box",
            }}
            gap="0.5rem"
            alignItems="center"
            justifyContent="center"
          >
            <div style={{ opacity: 0.7 }}>mem:</div>
            <h2>
              {((100 * stats()!.mem_used_gb) / stats()!.mem_total_gb).toFixed(
                1
              )}
              %
            </h2>
            <Show when={!isSemiMobile()}>
              <div>{stats()!.mem_total_gb.toFixed()} GiB</div>
            </Show>
          </Flex>
          <Flex
            style={{
              width: "100%",
              "box-sizing": "border-box",
            }}
            gap="0.5rem"
            alignItems="center"
            justifyContent="center"
          >
            <div style={{ opacity: 0.7 }}>disk:</div>
            <h2>
              {((100 * stats()!.disk.used_gb) / stats()!.disk.total_gb).toFixed(
                1
              )}
              %
            </h2>
            <Show when={!isSemiMobile()}>
              <div>{readableStorageAmount(stats()!.disk.total_gb)}</div>
            </Show>
          </Flex>
        </Grid>
        <Flex gap=".5rem" alignItems="center">
          <Show
            when={!reloading()}
            fallback={
              <button class="blue" style={{ height: "fit-content" }}>
                <Loading type="spinner" scale={0.2} />
              </button>
            }
          >
            <button
              class="blue"
              style={{ height: "fit-content" }}
              onClick={(e) => {
                e.stopPropagation();
                reloadStats();
              }}
            >
              <Icon type="refresh" width="0.85rem" />
            </button>
          </Show>
          <A
            href={`/server/${p.id}/stats`}
            class="blue"
            onClick={(e) => e.stopPropagation()}
          >
            <Icon type="timeline-line-chart" />
          </A>
        </Flex>
      </Show>
    </Show>
  );
}
