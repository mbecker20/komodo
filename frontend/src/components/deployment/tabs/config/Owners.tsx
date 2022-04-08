import { User } from "@monitor/types";
import { Component, createEffect, createSignal, For, Show } from "solid-js";
import { pushNotification } from "../../../..";
import { useUser } from "../../../../state/UserProvider";
import {
  addOwnerToDeployment,
  getUsers,
  removeOwnerFromDeployment,
} from "../../../../util/query";
import ConfirmButton from "../../../util/ConfirmButton";
import Input from "../../../util/Input";
import Flex from "../../../util/layout/Flex";
import Grid from "../../../util/layout/Grid";
import Menu from "../../../util/menu/Menu";
import { useConfig } from "./Provider";

const Owners: Component<{}> = (p) => {
  const { deployment } = useConfig();
  const { permissions, username } = useUser();
  const [userSearch, setUserSearch] = createSignal("");
  const [users, setUsers] = createSignal<User[]>([]);
  createEffect(() => {
    if (userSearch().length > 0) {
      getUsers(userSearch(), true).then((users) => {
        setUsers(
          users.filter((user) => !deployment.owners.includes(user.username))
        );
      });
    } else {
      setUsers([]);
    }
  });
  return (
    <Grid class="config-item shadow">
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
                  style={{ width: "100%", "justify-content": "flex-start" }}
                  onConfirm={async () => {
                    await addOwnerToDeployment(deployment._id!, user.username);
                    pushNotification("good", "owner added to deployment");
                    setUserSearch("");
                  }}
                  confirmText="add user"
                >
                  {user.username}
                </ConfirmButton>
              )}
            </For>
            <Show when={users().length === 0}>no matching users</Show>
          </>
        }
        style={{ width: "12rem" }}
      />
      <For each={deployment.owners}>
        {(owner) => (
          <Flex alignItems="center" justifyContent="space-between">
            <div class="big-text">
              {owner}
              {owner === username() && " ( you )"}
            </div>
            <Show when={permissions() > 1}>
              <ConfirmButton
                color="red"
                onConfirm={async () => {
                  await removeOwnerFromDeployment(deployment._id!, owner);
                  pushNotification("good", "user removed from collaborators");
                }}
              >
                remove
              </ConfirmButton>
            </Show>
          </Flex>
        )}
      </For>
    </Grid>
  );
};

export default Owners;
