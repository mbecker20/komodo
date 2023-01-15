import { Component, Show } from "solid-js";
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

const Header: Component<{}> = (p) => {
  const { servers } = useAppState();
  const params = useParams();
  const server = () => servers.get(params.id)!;
  const status = () => server().status.replaceAll("_", " ").toUpperCase();
  const { user } = useUser();
  const { isMobile } = useAppDimensions();
  const [showUpdates, toggleShowUpdates] =
    useLocalStorageToggle("show-updates");
  const userCanUpdate = () =>
    user().admin ||
    server().server.permissions![getId(user())] === PermissionLevel.Update;
  return (
    <>
      <Flex
        class={combineClasses("card shadow")}
        justifyContent="space-between"
        alignItems="center"
        style={{
          position: "relative",
          cursor: isMobile() && userCanUpdate() ? "pointer" : undefined,
        }}
        onClick={() => {
          if (isMobile() && userCanUpdate()) toggleShowUpdates();
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
        <Show when={isMobile() && userCanUpdate()}>
          <Flex gap="0.5rem" alignItems="center" class="show-updates-indicator">
            updates{" "}
            <Icon
              type={showUpdates() ? "chevron-up" : "chevron-down"}
              width="0.9rem"
            />
          </Flex>
        </Show>
      </Flex>
      <Show when={isMobile() && userCanUpdate() && showUpdates()}>
        <Updates />
      </Show>
    </>
  );
};

export default Header;
