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
  const { build, setBuild } = useConfig();
  return (
    <Grid class={combineClasses(s.ConfigItem, "shadow")}>
      <h1>docker build</h1> {/* checkbox here? */}
      <Flex justifyContent="space-between" alignItems="center">
        <div>build path</div>
        <Input
          placeholder="from root of repo"
          value={build.dockerBuildArgs?.buildPath || ""}
          onEdit={(buildPath) => setBuild("dockerBuildArgs", { buildPath })}
        />
      </Flex>
      <Flex justifyContent="space-between" alignItems="center">
        <div>dockerfile path</div>
        <Input
          placeholder="from root of build path"
          value={build.dockerBuildArgs?.dockerfilePath || ""}
          onEdit={(dockerfilePath) =>
            setBuild("dockerBuildArgs", { dockerfilePath })
          }
        />
      </Flex>
      <Show when={dockerAccounts()}>
        <Flex justifyContent="space-between" alignItems="center">
          <div>account</div>
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
          />
        </Flex>
      </Show>
    </Grid>
  );
};

export default Docker;
