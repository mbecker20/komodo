import { Component, Show } from "solid-js";
import { useAppState } from "../../../../state/StateProvider";
import { useTheme } from "../../../../state/ThemeProvider";
import { combineClasses } from "../../../../util/helpers";
import Input from "../../../util/Input";
import Flex from "../../../util/layout/Flex";
import Grid from "../../../util/layout/Grid";
import Selector from "../../../util/menu/Selector";
import { useConfig } from "../Provider";

const Docker: Component<{}> = (p) => {
  const { dockerAccounts } = useAppState();
  const { build, setBuild, userCanUpdate } = useConfig();
  const { themeClass } = useTheme();
  return (
    <Grid class={combineClasses("config-item shadow", themeClass())}>
      <h1>docker build</h1> {/* checkbox here? */}
      <Flex
        justifyContent={userCanUpdate() ? "space-between" : undefined}
        alignItems="center"
        style={{ "flex-wrap": "wrap" }}
      >
        <h2>build path: </h2>
        <Input
          placeholder="from root of repo"
          value={build.dockerBuildArgs?.buildPath || ""}
          onEdit={(buildPath) => setBuild("dockerBuildArgs", { buildPath })}
          disabled={!userCanUpdate()}
        />
      </Flex>
      <Flex
        justifyContent={userCanUpdate() ? "space-between" : undefined}
        alignItems="center"
        style={{ "flex-wrap": "wrap" }}
      >
        <h2>dockerfile path: </h2>
        <Input
          placeholder="from root of build path"
          value={
            build.dockerBuildArgs?.dockerfilePath ||
            (userCanUpdate() ? "" : "./dockerfile")
          }
          onEdit={(dockerfilePath) =>
            setBuild("dockerBuildArgs", { dockerfilePath })
          }
          disabled={!userCanUpdate()}
        />
      </Flex>
      <Show when={dockerAccounts() && dockerAccounts()!.length > 0}>
        <Flex
          justifyContent={userCanUpdate() ? "space-between" : undefined}
          alignItems="center"
          style={{ "flex-wrap": "wrap" }}
        >
          <h2>dockerhub account: </h2>
          <Selector
            targetClass="blue"
            selected={build.dockerAccount || "none"}
            items={["none", ...dockerAccounts()!]}
            onSelect={(account) => {
              setBuild(
                "dockerAccount",
                account === "none" ? undefined : account
              );
            }}
            position="bottom right"
            disabled={!userCanUpdate()}
          />
        </Flex>
      </Show>
    </Grid>
  );
};

export default Docker;
