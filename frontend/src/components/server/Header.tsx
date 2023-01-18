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
import { PermissionLevel, Server } from "../../types";
import { A, useParams } from "@solidjs/router";
import { client } from "../..";
import Loading from "../shared/loading/Loading";

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
  const [version] = createResource(async () => {
    return await client.get_server_version(params.id).catch();
  });
  return (
    <>
      <Flex
        class={combineClasses("card shadow")}
        justifyContent="space-between"
        alignItems="center"
        style={{
          position: "relative",
          cursor: isSemiMobile() ? "pointer" : undefined,
        }}
        onClick={() => {
          if (isSemiMobile()) toggleShowUpdates();
        }}
      >
        <Grid gap="0.1rem">
          <h1>{server().server.name}</h1>
          <Flex gap="0.2rem" alignItems="center" style={{ opacity: 0.8 }}>
            <div>server</div>
            <Show when={server().server.region}>
              <Icon type="caret-right" width="0.7rem" />
              {server().server.region}
            </Show>
          </Flex>
        </Grid>
        <Flex alignItems="center">
          <Show when={!isMobile()}>
            <Show when={version()} fallback={<Loading type="three-dot" />}>
              <div style={{ opacity: 0.7 }}>periphery v{version()}</div>
            </Show>
          </Show>
          <div class={serverStatusClass(server().status)}>{status()}</div>
          <A
            href={`/server/${params.id}/stats`}
            class="blue"
            onClick={(e) => e.stopPropagation()}
          >
            <Icon type="timeline-line-chart" />
          </A>
          <Show when={userCanUpdate()}>
            <ConfirmButton
              onConfirm={() => {
                client.delete_server(params.id);
              }}
              class="red"
            >
              <Icon type="trash" />
            </ConfirmButton>
          </Show>
        </Flex>
        <Show when={isSemiMobile()}>
          <Flex gap="0.5rem" alignItems="center" class="show-updates-indicator">
            updates{" "}
            <Icon
              type={showUpdates() ? "chevron-up" : "chevron-down"}
              width="0.9rem"
            />
          </Flex>
        </Show>
      </Flex>
      <Show when={isSemiMobile() && showUpdates()}>
        <Updates />
      </Show>
    </>
  );
};

export default Header;
