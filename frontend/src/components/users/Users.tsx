import {
  Component,
  createMemo,
  createResource,
  createSignal,
  For,
  onCleanup,
  Show,
} from "solid-js";
import { USER_UPDATE } from "../../state/actions";
import { useAppState } from "../../state/StateProvider";
import { combineClasses, readablePermissions } from "../../util/helpers";
import { deleteUser, getUsers, updateUser } from "../../util/query";
import ConfirmButton from "../util/ConfirmButton";
import Icon from "../util/Icon";
import Input from "../util/Input";
import Flex from "../util/layout/Flex";
import Grid from "../util/layout/Grid";
import Loading from "../util/loading/Loading";
import Selector from "../util/menu/Selector";
import s from "./users.module.scss";

const Users: Component<{}> = (p) => {
  const { ws } = useAppState();
  const [users, { refetch }] = createResource(() => getUsers());
  onCleanup(ws.subscribe([USER_UPDATE], refetch));
  const [search, setSearch] = createSignal("");
  const filteredUsers = createMemo(() =>
    users()?.filter((user) => user.username.includes(search()))
  );
  return (
    <Show
      when={users()}
      fallback={
        <Grid placeItems="center">
          <Loading type="sonar" />
        </Grid>
      }
    >
      <Grid class={s.UsersContent}>
        <Grid class={combineClasses(s.Users, "card")}>
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
              <Flex class={s.User}>
                <div class={s.Username}>{user.username}</div>
                <Flex alignItems="center">
                  <Selector
                    items={["view only", "user"]}
                    selected={readablePermissions(user.permissions!)}
                    onSelect={(_, permissions) => {
                      updateUser({ userID: user._id!, permissions });
                    }}
                  />
                  <button
                    class={user.enabled ? "green" : "red"}
                    onClick={() => {
                      updateUser({ userID: user._id!, enabled: !user.enabled });
                    }}
                  >
                    {user.enabled ? "enabled" : "disabled"}
                  </button>
                  <ConfirmButton
                    color="red"
                    onConfirm={() => deleteUser(user._id!)}
                  >
                    <Icon type="trash" />
                  </ConfirmButton>
                </Flex>
              </Flex>
            )}
          </For>
        </Grid>
      </Grid>
    </Show>
  );
};

export default Users;
