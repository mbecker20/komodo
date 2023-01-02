import { useParams } from "@solidjs/router";
import { Component, createEffect, createSignal, For, Show } from "solid-js";
import { client, pushNotification } from "../../..";
import { useUser } from "../../../state/UserProvider";
import { PermissionLevel, PermissionsTarget, User } from "../../../types";
import { combineClasses, getId } from "../../../util/helpers";
import ConfirmButton from "../../shared/ConfirmButton";
import Input from "../../shared/Input";
import Flex from "../../shared/layout/Flex";
import Grid from "../../shared/layout/Grid";
import Menu from "../../shared/menu/Menu";
import { useConfig } from "./config/Provider";

const Owners: Component<{}> = (p) => {
  const { server } = useConfig();
  const { id } = useParams();
  const { user } = useUser();
  const [userSearch, setUserSearch] = createSignal("");
  const [users, setUsers] = createSignal<User[]>([]);
  createEffect(() => {
    if (userSearch().length > 0) {
      // getUsers(userSearch(), true).then((users) => {
      //   setUsers(
      //     users.filter((user) => !server.owners.includes(user.username))
      //   );
      // });
    } else {
      setUsers([]);
    }
  });
  return (
    <Show when={server.loaded}>
      <Grid class="config">
        <Grid class="config-items scroller" style={{ height: "100%" }}>
          <Grid
            class={combineClasses("config-item shadow")}
            gap="0.5rem"
          >
            <Menu
              show={userSearch() ? true : false}
              close={() => setUserSearch("")}
              target={
                <Input
                  placeholder="add user"
                  value={userSearch()}
                  onEdit={setUserSearch}
                  style={{ width: "12rem" }}
                />
              }
              content={
                <>
                  <For each={users()}>
                    {(user) => (
                      <ConfirmButton
                        color="grey"
                        style={{
                          width: "100%",
                          "justify-content": "flex-start",
                        }}
                        onConfirm={async () => {
                          await client.update_user_permissions_on_target({ user_id: getId(user), permission: PermissionLevel.Read, target_type: PermissionsTarget.Server, target_id: id });
                          pushNotification("good", "owner added to server");
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
            {/* <For each={Object.keys(server.permissions!)}>
              {(user_id) => (
                <Flex
                  alignItems="center"
                  justifyContent="space-between"
                  class={combineClasses("grey-no-hover")}
                  style={{
                    padding: "0.5rem",
                  }}
                >
                  <div class="big-text">
                    {owner}
                    {owner === username() && " ( you )"}
                  </div>
                  <Show when={permissions() > 1}>
                    <ConfirmButton
                      color="red"
                      onConfirm={async () => {
                        await removeOwnerFromServer(server._id!, owner);
                        pushNotification(
                          "good",
                          "user removed from collaborators"
                        );
                      }}
                    >
                      remove
                    </ConfirmButton>
                  </Show>
                </Flex>
              )}
            </For> */}
          </Grid>
        </Grid>
      </Grid>
    </Show>
  );
};

export default Owners;
