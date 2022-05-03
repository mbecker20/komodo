import { Build } from "@monitor/types";
import { Component, Show } from "solid-js";
import { DELETE_BUILD } from "@monitor/util";
import { useAppState } from "../../state/StateProvider";
import { useUser } from "../../state/UserProvider";
import ConfirmButton from "../util/ConfirmButton";
import Icon from "../util/Icon";
import Flex from "../util/layout/Flex";
import Grid from "../util/layout/Grid";
import { useActionStates } from "./ActionStateProvider";
import { useTheme } from "../../state/ThemeProvider";
import { combineClasses } from "../../util/helpers";
import { useAppDimensions } from "../../state/DimensionProvider";
import Updates from "./Updates";
import { useLocalStorageToggle } from "../../util/hooks";

const Header: Component<{}> = (p) => {
  const { builds, selected, ws } = useAppState();
  const build = () => builds.get(selected.id())!;
  const actions = useActionStates();
  const { permissions, username } = useUser();
  const { themeClass } = useTheme();
  const { isMobile } = useAppDimensions();
  const [showUpdates, toggleShowUpdates] =
    useLocalStorageToggle("show-updates");
  const userCanUpdate = () => {
    if (permissions() > 1) {
      return true;
    } else if (permissions() > 0 && build()!.owners.includes(username()!)) {
      return true;
    } else {
      return false;
    }
  };
  return (
    <>
      <Flex
        class={combineClasses("card shadow", themeClass())}
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
          <h1>{build().name}</h1>
          <div style={{ opacity: 0.8 }}>{getSub(build())}</div>
        </Grid>
        <Show when={userCanUpdate()}>
          <Show
            when={!actions.deleting}
            fallback={
              <button class="red">
                <Icon type="trash" />
              </button>
            }
          >
            <ConfirmButton
              onConfirm={() => {
                ws.send(DELETE_BUILD, { buildID: selected.id() });
              }}
              color="red"
            >
              <Icon type="trash" />
            </ConfirmButton>
          </Show>
        </Show>
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

function getSub(build: Build) {
  if (build.dockerBuildArgs) {
    return "docker build";
  } else if (build.cliBuild) {
    return "cli build";
  } else {
    return "build";
  }
}

export default Header;
