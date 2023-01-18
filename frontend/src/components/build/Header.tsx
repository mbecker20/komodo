import { Component, Show } from "solid-js";
import { useAppState } from "../../state/StateProvider";
import { useUser } from "../../state/UserProvider";
import ConfirmButton from "../shared/ConfirmButton";
import Icon from "../shared/Icon";
import Flex from "../shared/layout/Flex";
import Grid from "../shared/layout/Grid";
import { combineClasses, getId, version_to_string } from "../../util/helpers";
import { useAppDimensions } from "../../state/DimensionProvider";
import Updates from "./Updates";
import { useLocalStorageToggle } from "../../util/hooks";
import { useParams } from "@solidjs/router";
import { PermissionLevel } from "../../types";
import { client } from "../..";

const Header: Component<{}> = (p) => {
  const { builds } = useAppState();
  const params = useParams();
  const build = () => builds.get(params.id)!;
  const { user } = useUser();
  const { isSemiMobile } = useAppDimensions();
  const [showUpdates, toggleShowUpdates] =
    useLocalStorageToggle("show-updates");
  const userCanUpdate = () =>
    user().admin ||
    build().permissions![getId(user())] === PermissionLevel.Update;
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
          <h1>{build().name}</h1>
          <div style={{ opacity: 0.8 }}>
            build - v{version_to_string(build().version)}
          </div>
        </Grid>
        <Show when={userCanUpdate()}>
          <ConfirmButton
            onConfirm={() => {
              client.delete_build(params.id);
            }}
            class="red"
          >
            <Icon type="trash" />
          </ConfirmButton>
        </Show>
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
