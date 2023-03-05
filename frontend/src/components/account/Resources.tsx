import { Component, createMemo, createSignal, For } from "solid-js";
import { useAppDimensions } from "../../state/DimensionProvider";
import { useAppState } from "../../state/StateProvider";
import { useUser } from "../../state/UserProvider";
import { PermissionLevel } from "../../types";
import Flex from "../shared/layout/Flex";
import Grid from "../shared/layout/Grid";

const Resources: Component<{}> = (p) => {
  const { user, user_id } = useUser();
  const { isMobile } = useAppDimensions();
  const { builds, deployments, servers } = useAppState();
  const [search, setSearch] = createSignal("");
  const _servers = createMemo(() => {
    return servers.filterArray((s) => {
      if (!s.server.name.includes(search())) return false;
      const p = s.server.permissions?.[user_id()];
      return p ? p !== PermissionLevel.None : false;
    });
  });
  const _deployments = createMemo(() => {
    return deployments.filterArray((d) => {
      if (!d.deployment.name.includes(search())) return false;
      const p = d.deployment.permissions?.[user_id()];
      return p ? p !== PermissionLevel.None : false;
    });
  });
  const _builds = createMemo(() => {
    return builds.filterArray((b) => {
      if (!b.name.includes(search())) return false;
      const p = b.permissions?.[user_id()];
      return p ? p !== PermissionLevel.None : false;
    });
  });
  return (
    <>
      <Grid
        class="card shadow"
        style={{ width: "100%", "box-sizing": "border-box" }}
      >
        <h1>servers</h1>
        <Grid gridTemplateColumns={isMobile() ? undefined : "1fr 1fr"}>
          <For each={_servers()}>
            {(item) => (
              <Flex class="card light shadow" justifyContent="space-between">
                <h2>{item.server.name}</h2>
                <div>{item.server.permissions?.[user_id()] || "none"}</div>
              </Flex>
            )}
          </For>
        </Grid>
      </Grid>
      <Grid
        class="card shadow"
        style={{ width: "100%", "box-sizing": "border-box" }}
      >
        <h1>deployments</h1>
        <Grid gridTemplateColumns={isMobile() ? undefined : "1fr 1fr"}>
          <For each={_deployments()}>
            {(item) => (
              <Flex class="card light shadow" justifyContent="space-between">
                <h2>{item.deployment.name}</h2>
                <div>{item.deployment.permissions?.[user_id()] || "none"}</div>
              </Flex>
            )}
          </For>
        </Grid>
      </Grid>
      <Grid
        class="card shadow"
        style={{ width: "100%", "box-sizing": "border-box" }}
      >
        <h1>builds</h1>
        <Grid gridTemplateColumns={isMobile() ? undefined : "1fr 1fr"}>
          <For each={_builds()}>
            {(item) => (
              <Flex class="card light shadow" justifyContent="space-between">
                <h2>{item.name}</h2>
                <div>{item.permissions?.[user_id()] || "none"}</div>
              </Flex>
            )}
          </For>
        </Grid>
      </Grid>
    </>
  );
};

export default Resources;
