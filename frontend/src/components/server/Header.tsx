import { Component, createResource, Show } from "solid-js";
import { useAppState } from "../../state/StateProvider";
import { useUser } from "../../state/UserProvider";
import { combineClasses, getId, serverStatusClass } from "../../util/helpers";
import ConfirmButton from "../shared/ConfirmButton";
import Icon from "../shared/Icon";
import Flex from "../shared/layout/Flex";
import Grid from "../shared/layout/Grid";
import { useAppDimensions } from "../../state/DimensionProvider";
import { useLocalStorageToggle } from "../../util/hooks";
import Updates from "./Updates";
import { PermissionLevel, ServerStatus } from "../../types";
import { A, useParams } from "@solidjs/router";
import { client } from "../..";
import Loading from "../shared/loading/Loading";
import HoverMenu from "../shared/menu/HoverMenu";
import ConfirmMenuButton from "../shared/ConfirmMenuButton";

const Header: Component<{}> = (p) => {
  const { servers } = useAppState();
  const params = useParams();
  const server = () => servers.get(params.id)!;
  const status = () => server().status.replaceAll("_", " ").toUpperCase();
  const { user } = useUser();
  const { isMobile, isSemiMobile } = useAppDimensions();
  const [showUpdates, toggleShowUpdates] =
    useLocalStorageToggle("show-updates");
  const userCanUpdate = () =>
    user().admin ||
    server().server.permissions![getId(user())] === PermissionLevel.Update;
  const [version] = createResource(
    () => server() && server().status === ServerStatus.Ok,
    async (do_it?: boolean) => {
      if (!do_it) return;
      return await client.get_server_version(params.id).catch();
    }
  );
  return (
    <>
      <Grid
        gap="0.5rem"
        class={combineClasses("card shadow")}
        style={{
          position: "relative",
          cursor: isSemiMobile() ? "pointer" : undefined,
          height: "fit-content",
        }}
        onClick={() => {
          if (isSemiMobile()) toggleShowUpdates();
        }}
      >
        <Flex alignItems="center" justifyContent="space-between">
          <h1>{server().server.name}</h1>
          <Show when={userCanUpdate()}>
            <Flex alignItems="center">
              <div class={serverStatusClass(server().status)}>{status()}</div>
              <HoverMenu
                target={
                  <A
                    href={`/server/${params.id}/stats`}
                    class="blue"
                    onClick={(e) => e.stopPropagation()}
                  >
                    <Icon type="timeline-line-chart" />
                  </A>
                }
                content="server stats"
                position="bottom center"
                padding="0.5rem"
              />
              <HoverMenu
                target={
                  <ConfirmMenuButton
                    onConfirm={() => {
                      client.delete_server(params.id);
                    }}
                    class="red"
                    title="delete server"
                    match={server().server.name}
                    info={
                      <div style={{ opacity: 0.7 }}>
                        warning! this will also delete all builds and
                        deployments on this server
                      </div>
                    }
                  >
                    <Icon type="trash" />
                  </ConfirmMenuButton>
                }
                content="delete server"
                position="bottom center"
                padding="0.5rem"
              />
            </Flex>
          </Show>
        </Flex>
        <Flex alignItems="center" justifyContent="space-between">
          <Flex gap="0.2rem" alignItems="center" style={{ opacity: 0.8 }}>
            <div>server</div>
            <Show when={server().server.region}>
              <Icon type="caret-right" width="0.7rem" />
              {server().server.region}
            </Show>
          </Flex>
          <Show when={!isMobile()}>
            <Show when={version()}>
              <div style={{ opacity: 0.7 }}>periphery v{version()}</div>
            </Show>
          </Show>
        </Flex>
      </Grid>
      <Show when={isSemiMobile() && showUpdates()}>
        <Updates />
      </Show>
    </>
  );
};

export default Header;
