import { Component, Show } from "solid-js";
import { useAppState } from "../../../../state/StateProvider";
import { combineClasses } from "../../../../util/helpers";
import Input from "../../../util/Input";
import Flex from "../../../util/layout/Flex";
import Grid from "../../../util/layout/Grid";
import Selector from "../../../util/menu/Selector";
import s from "../../build.module.css";
import { useConfig } from "../Provider";

const Docker: Component<{}> = (p) => {
  const { dockerAccounts } = useAppState();
  const { build, setBuild, userCanUpdate } = useConfig();
  return (
    <Grid class="config-item shadow">
      <h1>docker build</h1> {/* checkbox here? */}
      <Flex
        justifyContent={userCanUpdate() ? "space-between" : undefined}
        alignItems="center"
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
        >
          <h2>account: </h2>
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
