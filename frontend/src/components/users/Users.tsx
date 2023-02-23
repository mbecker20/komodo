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
import { useAppState } from "../../state/StateProvider";
import { Operation } from "../../types";
import { combineClasses, getId } from "../../util/helpers";
import Input from "../shared/Input";
import Flex from "../shared/layout/Flex";
import Grid from "../shared/layout/Grid";
import Loading from "../shared/loading/Loading";
import s from "./users.module.scss";

const Users: Component<{}> = (p) => {
  const { ws } = useAppState();
  const [users, { refetch }] = createResource(() => client.list_users());
  onCleanup(
    ws.subscribe(
      [
        Operation.ModifyUserEnabled,
        Operation.ModifyUserCreateServerPermissions,
        Operation.ModifyUserCreateBuildPermissions,
      ],
      refetch
    )
  );
  const [search, setSearch] = createSignal("");
  const filteredUsers = createMemo(() =>
    users()?.filter((user) => user.username.includes(search()))
  );
  return (
    <Show
      when={users()}
      fallback={
        <Grid placeItems="center" class="content">
          <Loading type="sonar" />
        </Grid>
      }
    >
      <Grid
        class="card shadow"
        style={{ width: "100%", "box-sizing": "border-box" }}
      >
        <Flex justifyContent="space-between">
          <h1>users</h1>
          <Input
            class="lightgrey"
            placeholder="search"
            value={search()}
            onEdit={setSearch}
          />
        </Flex>
        <For each={filteredUsers()}>
          {(user) => (
            <Flex class={combineClasses(s.User, "shadow")}>
              <div class={s.Username}>{user.username}</div>
              <Flex alignItems="center">
                <button
                  class={user.enabled ? "green" : "red"}
                  style={{ width: "6rem" }}
                  onClick={() => {
                    client.modify_user_enabled({
                      user_id: getId(user),
                      enabled: !user.enabled,
                    });
                  }}
                >
                  {user.enabled ? "enabled" : "disabled"}
                </button>
                <button
                  class={user.create_server_permissions ? "green" : "red"}
                  style={{ width: "11rem" }}
                  onClick={() => {
                    client.modify_user_create_server_permissions({
                      user_id: getId(user),
                      create_server_permissions:
                        !user.create_server_permissions,
                    });
                  }}
                >
                  {user.create_server_permissions
                    ? "can create servers"
                    : "cannot create servers"}
                </button>
                <button
                  class={user.create_build_permissions ? "green" : "red"}
                  style={{ width: "11rem" }}
                  onClick={() => {
                    client.modify_user_create_build_permissions({
                      user_id: getId(user),
                      create_build_permissions: !user.create_build_permissions,
                    });
                  }}
                >
                  {user.create_build_permissions
                    ? "can create builds"
                    : "cannot create builds"}
                </button>
                {/* <ConfirmButton
                    class="red"
                    onConfirm={() => deleteUser(user._id!)}
                  >
                    <Icon type="trash" />
                  </ConfirmButton> */}
              </Flex>
            </Flex>
          )}
        </For>
      </Grid>
    </Show>
  );
};

export default Users;
