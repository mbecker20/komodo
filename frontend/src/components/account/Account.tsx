import { Component, Show } from "solid-js";
import { useUser } from "../../state/UserProvider";
import { readableMonitorTimestamp, readableUserType } from "../../util/helpers";
import Flex from "../shared/layout/Flex";
import Resources from "./Resources";
import Secrets from "./Secrets";

const Account: Component<{}> = (p) => {
  const { user, username } = useUser();
  return (
    <>
      <Flex
        class="card shadow"
        style={{ width: "100%", "box-sizing": "border-box" }}
        alignItems="center"
        justifyContent="space-between"
      >
        <h1>{username()}</h1>
        <Flex>
          <Show when={user().admin}>
            <div class="dimmed">admin</div>
          </Show>
          <Flex gap="0.5rem">
            <div class="dimmed">type:</div>
            <div>{user() ? readableUserType(user()!) : "unknown"}</div>
          </Flex>
          <Flex gap="0.5rem">
            <div class="dimmed">created:</div>
            <div>{readableMonitorTimestamp(user().created_at!)}</div>
          </Flex>
        </Flex>
      </Flex>
      <Secrets />
      <Show when={!user().admin}>
        <Resources />
      </Show>
    </>
  );
};

export default Account;
