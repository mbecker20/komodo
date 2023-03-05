import { A } from "@solidjs/router";
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
import { getId } from "../../util/helpers";
import Input from "../shared/Input";
import Flex from "../shared/layout/Flex";
import Grid from "../shared/layout/Grid";
import Loading from "../shared/loading/Loading";
import { UserPermissionButtons } from "./User";

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
            <Show
              when={!user.admin}
              fallback={
                <Flex class="card light shadow">
                  <h2>{user.username}</h2>
                  <h2 style={{ opacity: 0.7 }}>admin</h2>
                </Flex>
              }
            >
              <A
                href={`/user/${getId(user)}`}
                class="card light shadow"
                style={{
                  width: "100%",
                  "justify-content": "space-between",
                  "align-items": "center",
                }}
              >
                <h2>{user.username}</h2>
                <UserPermissionButtons user={user} />
              </A>
            </Show>
          )}
        </For>
      </Grid>
    </Show>
  );
};

export default Users;
