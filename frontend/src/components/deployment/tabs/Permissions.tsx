import { useParams } from "@solidjs/router";
import {
  Component,
  createEffect,
  createMemo,
  createSignal,
  For,
  onCleanup,
  Show,
} from "solid-js";
import { client } from "../../..";
import { useAppState } from "../../../state/StateProvider";
import { useUser } from "../../../state/UserProvider";
import {
  Operation,
  PermissionLevel,
  PermissionsTarget,
  User,
} from "../../../types";
import { combineClasses, getId } from "../../../util/helpers";
import ConfirmButton from "../../shared/ConfirmButton";
import Input from "../../shared/Input";
import Flex from "../../shared/layout/Flex";
import Grid from "../../shared/layout/Grid";
import Menu from "../../shared/menu/Menu";
import Selector from "../../shared/menu/Selector";
import { useConfig } from "./config/Provider";

const PERMISSIONS_OPTIONS = [
  PermissionLevel.Read,
  PermissionLevel.Execute,
  PermissionLevel.Update,
];

const Permissions: Component<{}> = (p) => {
  const { ws } = useAppState();
  const { deployment, reset } = useConfig();
  const { user } = useUser();
  const params = useParams();
  const [userSearch, setUserSearch] = createSignal("");
  const [users, setUsers] = createSignal<User[]>([]);
  createEffect(() => {
    client.list_users().then(setUsers);
  });
  const getUser = (user_id: string) =>
    users().find((u) => getId(u) === user_id);
  const searchUsers = createMemo(() =>
    users().filter(
      (u) =>
        !u.admin &&
        u.enabled &&
        u.username.includes(userSearch()) &&
        (deployment.permissions![getId(u)] === undefined ||
          deployment.permissions![getId(u)] === PermissionLevel.None)
    )
  );
  let unsub_permissions = () => {};
  createEffect(() => {
    unsub_permissions();
    unsub_permissions = ws.subscribe([Operation.ModifyUserPermissions], () =>
      reset()
    );
  });
  onCleanup(() => unsub_permissions());
  let unsub_enabled = () => {};
  createEffect(() => {
    unsub_enabled();
    unsub_enabled = ws.subscribe([Operation.ModifyUserEnabled], () =>
      client.list_users().then(setUsers)
    );
  });
  onCleanup(() => unsub_enabled());
  return (
    <Grid class="config">
      <Grid
        class="config-items scroller"
        style={{ height: "100%", "min-height": "400px" }}
      >
        <Grid class={combineClasses("config-item shadow")} gap="0.5rem">
          <Menu
            show={userSearch() ? true : false}
            close={() => setUserSearch("")}
            target={
              <Input
                placeholder="add user"
                value={userSearch()}
                onEdit={setUserSearch}
              />
            }
            content={
              <>
                <For each={searchUsers()}>
                  {(user) => (
                    <ConfirmButton
                      class="grey"
                      style={{
                        width: "100%",
                        "justify-content": "flex-start",
                      }}
                      onConfirm={() => {
                        client.update_user_permissions_on_target({
                          user_id: getId(user),
                          permission: PermissionLevel.Read,
                          target_type: PermissionsTarget.Deployment,
                          target_id: params.id,
                        });
                        setUserSearch("");
                      }}
                      confirm="add user"
                    >
                      {user.username}
                    </ConfirmButton>
                  )}
                </For>
                <Show when={users().length === 0}>no matching users</Show>
              </>
            }
            menuStyle={{ width: "12rem" }}
          />
          <For
            each={Object.entries(deployment.permissions!)
              .filter(([_, permission]) => permission !== PermissionLevel.None)
              .map(([user_id, _]) => user_id)}
          >
            {(user_id) => {
              const u = () => getUser(user_id)!;
              const permissions = () => deployment.permissions![user_id];
              return (
                <Show when={u()}>
                  <Flex
                    alignItems="center"
                    justifyContent="space-between"
                    class={combineClasses("grey-no-hover")}
                    style={{
                      padding: "0.5rem",
                    }}
                  >
                    <div class="big-text">
                      {u().username}
                      {user_id === getId(user()) && " ( you )"}
                    </div>
                    <Show when={!u().admin && user_id !== getId(user())}>
                      <Flex alignItems="center">
                        <Selector
                          selected={permissions()}
                          items={PERMISSIONS_OPTIONS}
                          onSelect={(permission) => {
                            client.update_user_permissions_on_target({
                              user_id,
                              permission: permission as PermissionLevel,
                              target_type: PermissionsTarget.Deployment,
                              target_id: params.id,
                            });
                          }}
                          position="bottom center"
                        />
                        <ConfirmButton
                          class="red"
                          onConfirm={() => {
                            client.update_user_permissions_on_target({
                              user_id,
                              permission: PermissionLevel.None,
                              target_type: PermissionsTarget.Deployment,
                              target_id: params.id,
                            });
                          }}
                        >
                          remove
                        </ConfirmButton>
                      </Flex>
                    </Show>
                  </Flex>
                </Show>
              );
            }}
          </For>
        </Grid>
      </Grid>
    </Grid>
  );
};

export default Permissions;
