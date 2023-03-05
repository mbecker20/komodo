import { A, useParams } from "@solidjs/router";
import {
  Component,
  createMemo,
  createResource,
  createSignal,
  For,
  onCleanup,
  Show,
} from "solid-js";
import { client } from "../..";
import { useAppDimensions } from "../../state/DimensionProvider";
import { useAppState } from "../../state/StateProvider";
import {
  Operation,
  PermissionLevel,
  PermissionsTarget,
  User as UserType,
} from "../../types";
import { getId } from "../../util/helpers";
import { useToggle } from "../../util/hooks";
import CheckBox from "../shared/CheckBox";
import Icon from "../shared/Icon";
import Input from "../shared/Input";
import Flex from "../shared/layout/Flex";
import Grid from "../shared/layout/Grid";
import Loading from "../shared/loading/Loading";
import Selector from "../shared/menu/Selector";

const User: Component = () => {
  const { isMobile } = useAppDimensions();
  const { builds, deployments, servers, ws } = useAppState();
  const params = useParams<{ id: string }>();
  const [user, { refetch }] = createResource(() =>
    client.get_user_by_id(params.id)
  );
  onCleanup(
    ws.subscribe(
      [
        Operation.ModifyUserEnabled,
        Operation.ModifyUserCreateServerPermissions,
        Operation.ModifyUserCreateBuildPermissions,
        Operation.ModifyUserPermissions,
      ],
      refetch
    )
  );
  const [showAll, toggleShowAll] = useToggle(false);
  const [search, setSearch] = createSignal("");
  const _servers = createMemo(() => {
    if (showAll()) {
      return servers.filterArray((s) => s.server.name.includes(search()));
    } else {
      return servers.filterArray((s) => {
        if (!s.server.name.includes(search())) return false;
        const p = s.server.permissions?.[params.id];
        return p ? p !== PermissionLevel.None : false;
      });
    }
  });
  const _deployments = createMemo(() => {
    if (showAll()) {
      return deployments.filterArray((d) =>
        d.deployment.name.includes(search())
      );
    } else {
      return deployments.filterArray((d) => {
        if (!d.deployment.name.includes(search())) return false;
        const p = d.deployment.permissions?.[params.id];
        return p ? p !== PermissionLevel.None : false;
      });
    }
  });
  const _builds = createMemo(() => {
    if (showAll()) {
      return builds.filterArray((b) => b.name.includes(search()));
    } else {
      return builds.filterArray((b) => {
        if (!b.name.includes(search())) return false;
        const p = b.permissions?.[params.id];
        return p ? p !== PermissionLevel.None : false;
      });
    }
  });
  return (
    <Grid
      class="card shadow"
      style={{ width: "100%", "box-sizing": "border-box" }}
    >
      <Show when={user()} fallback={<Loading type="three-dot" />}>
        <Flex alignItems="center" justifyContent="space-between">
          <Flex alignItems="center">
            <A href="/users" class="grey">
              <Icon type="arrow-left" />
            </A>
            <h1>{user()?.username}</h1>
            <Show when={user()?.admin}>
              <h2 style={{ opacity: 0.7 }}>admin</h2>
            </Show>
          </Flex>
          <Flex alignItems="center">
            <CheckBox
              label="show all resources"
              checked={showAll()}
              toggle={toggleShowAll}
            />
            <UserPermissionButtons user={user()!} />
          </Flex>
        </Flex>
        <Input
          placeholder="search resources"
          class="lightgrey"
          style={{ padding: "0.5rem" }}
          value={search()}
          onEdit={setSearch}
        />
        <Grid class="card light shadow">
          <Flex alignItems="center">
            <h1>servers</h1>
            <Show when={_servers()?.length === 0}>
              <div>empty</div>
            </Show>
          </Flex>
          <Grid gridTemplateColumns={isMobile() ? undefined : "1fr 1fr"}>
            <For each={_servers()}>
              {(item) => (
                <Flex
                  class="card shadow"
                  alignItems="center"
                  justifyContent="space-between"
                >
                  <h2>{item.server.name}</h2>
                  <Selector
                    targetClass={
                      (item.server.permissions?.[params.id] || "none") !==
                      "none"
                        ? "blue"
                        : "red"
                    }
                    selected={item.server.permissions?.[params.id] || "none"}
                    items={["none", "read", "execute", "update"]}
                    onSelect={(permission) => {
                      client.update_user_permissions_on_target({
                        user_id: params.id,
                        target_type: PermissionsTarget.Server,
                        target_id: getId(item.server),
                        permission: permission as PermissionLevel,
                      });
                    }}
                  />
                </Flex>
              )}
            </For>
          </Grid>
        </Grid>
        <Grid class="card light shadow">
          <Flex alignItems="center">
            <h1>deployments</h1>
            <Show when={_deployments()?.length === 0}>
              <div>empty</div>
            </Show>
          </Flex>
          <Grid gridTemplateColumns={isMobile() ? undefined : "1fr 1fr"}>
            <For each={_deployments()}>
              {(item) => (
                <Flex
                  class="card shadow"
                  alignItems="center"
                  justifyContent="space-between"
                >
                  <h2>{item.deployment.name}</h2>
                  <Selector
                    targetClass={
                      (item.deployment.permissions?.[params.id] || "none") !==
                      "none"
                        ? "blue"
                        : "red"
                    }
                    selected={
                      item.deployment.permissions?.[params.id] || "none"
                    }
                    items={["none", "read", "execute", "update"]}
                    onSelect={(permission) => {
                      client.update_user_permissions_on_target({
                        user_id: params.id,
                        target_type: PermissionsTarget.Deployment,
                        target_id: getId(item.deployment),
                        permission: permission as PermissionLevel,
                      });
                    }}
                  />
                </Flex>
              )}
            </For>
          </Grid>
        </Grid>
        <Grid class="card light shadow">
          <Flex alignItems="center">
            <h1>builds</h1>
            <Show when={_builds()?.length === 0}>
              <div>empty</div>
            </Show>
          </Flex>
          <Grid gridTemplateColumns={isMobile() ? undefined : "1fr 1fr"}>
            <For each={_builds()}>
              {(item) => (
                <Flex
                  class="card shadow"
                  alignItems="center"
                  justifyContent="space-between"
                >
                  <h2>{item.name}</h2>
                  <Selector
                    targetClass={
                      (item.permissions?.[params.id] || "none") !== "none"
                        ? "blue"
                        : "red"
                    }
                    selected={item.permissions?.[params.id] || "none"}
                    items={["none", "read", "execute", "update"]}
                    onSelect={(permission) => {
                      client.update_user_permissions_on_target({
                        user_id: params.id,
                        target_type: PermissionsTarget.Build,
                        target_id: getId(item),
                        permission: permission as PermissionLevel,
                      });
                    }}
                  />
                </Flex>
              )}
            </For>
          </Grid>
        </Grid>
      </Show>
    </Grid>
  );
};

export default User;

export const UserPermissionButtons: Component<{ user: UserType }> = (p) => {
  const { isMobile } = useAppDimensions();
  return (
    <Show when={!p.user.admin}>
      <Grid
        placeItems="center end"
        gridTemplateColumns={!isMobile() ? "auto 1fr 1fr" : undefined}
      >
        <button
          class={p.user.enabled ? "green" : "red"}
          style={{ width: isMobile() ? "11rem" : "6rem" }}
          onClick={(e) => {
            e.stopPropagation();
            e.preventDefault();
            client.modify_user_enabled({
              user_id: getId(p.user),
              enabled: !p.user.enabled,
            });
          }}
        >
          {p.user.enabled ? "enabled" : "disabled"}
        </button>
        <button
          class={p.user.create_server_permissions ? "green" : "red"}
          style={{ width: "11rem" }}
          onClick={(e) => {
            e.stopPropagation();
            e.preventDefault();
            client.modify_user_create_server_permissions({
              user_id: getId(p.user),
              create_server_permissions: !p.user.create_server_permissions,
            });
          }}
        >
          {p.user.create_server_permissions
            ? "can create servers"
            : "cannot create servers"}
        </button>
        <button
          class={p.user.create_build_permissions ? "green" : "red"}
          style={{ width: "11rem" }}
          onClick={(e) => {
            e.stopPropagation();
            e.preventDefault();
            client.modify_user_create_build_permissions({
              user_id: getId(p.user),
              create_build_permissions: !p.user.create_build_permissions,
            });
          }}
        >
          {p.user.create_build_permissions
            ? "can create builds"
            : "cannot create builds"}
        </button>
      </Grid>
    </Show>
  );
};
