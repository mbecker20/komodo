import { Component, For, Show } from "solid-js";
import { createStore } from "solid-js/store";
import { client, pushNotification } from "../..";
import { useUser } from "../../state/UserProvider";
import { copyToClipboard, readableMonitorTimestamp } from "../../util/helpers";
import { useToggle } from "../../util/hooks";
import ConfirmButton from "../shared/ConfirmButton";
import Icon from "../shared/Icon";
import Input from "../shared/Input";
import Flex from "../shared/layout/Flex";
import Grid from "../shared/layout/Grid";
import Loading from "../shared/loading/Loading";
import CenterMenu from "../shared/menu/CenterMenu";
import Selector from "../shared/menu/Selector";

const Secrets: Component<{}> = (p) => {
	const { user, reloadUser } = useUser();
  const [showCreate, toggleShowCreate] = useToggle();
	return (
    <Grid
      class="card shadow"
      style={{ width: "100%", "box-sizing": "border-box" }}
    >
      <Flex justifyContent="space-between">
        <h1>api secrets</h1>
        <CenterMenu
          show={showCreate}
          toggleShow={toggleShowCreate}
          targetClass="green"
          title="create secret"
          target={<Icon type="plus" />}
          content={() => <CreateNewSecretMenu />}
          position="center"
        />
      </Flex>
      <For each={user().secrets}>
        {(secret) => (
          <Flex
            class="card light shadow wrap"
            justifyContent="space-between"
            alignItems="center"
          >
            <h2>{secret.name}</h2>
            <Flex alignItems="center">
              <Flex gap="0.25rem">
                <div style={{ opacity: 0.7 }}>created:</div>
                <div>{readableMonitorTimestamp(secret.created_at)}</div>
              </Flex>
              <Flex gap="0.25rem">
                <div style={{ opacity: 0.7 }}>expires:</div>
                <div>
                  {secret.expires
                    ? readableMonitorTimestamp(secret.expires)
                    : "never"}
                </div>
              </Flex>
              <ConfirmButton
                class="red"
                onConfirm={() =>
                  client.delete_api_secret(secret.name).then(reloadUser)
                }
              >
                <Icon type="trash" />
              </ConfirmButton>
            </Flex>
          </Flex>
        )}
      </For>
    </Grid>
  );
}

export default Secrets;

const EXPIRE_LENGTHS = ["30 days", "90 days", "1 year", "never"] as const;
type ExpireLength = typeof EXPIRE_LENGTHS[number];

const CreateNewSecretMenu = () => {
  const { reloadUser } = useUser();
  const [info, setInfo] = createStore<{
    name: string;
    expires: ExpireLength;
    loading: boolean;
    secret: string | undefined;
  }>({
    name: "",
    expires: "90 days",
    loading: false,
    secret: undefined,
  });
  const createSecret = async () => {
    if (info.name.length === 0) {
      pushNotification("bad", "secret name cannot be empty");
    }
    setInfo("loading", true);
    try {
      const secret = await client.create_api_secret({
        name: info.name,
        expires: createExpires(info.expires),
      });
      reloadUser();
      setInfo("loading", false);
      setInfo("secret", secret);
    } catch (error) {
      pushNotification("bad", "failed to create api secret");
      console.log(error);
      setInfo("loading", false);
    }
  };
  return (
    <>
      <Show when={info.secret === undefined}>
        <Flex class="wrap" alignItems="center">
          <Input
            class="darkgrey"
            placeholder="name this secret"
            style={{ "font-size": "1.5rem" }}
            onEdit={(name) => setInfo("name", name)}
          />
          <Selector
            selected={info.expires}
            items={EXPIRE_LENGTHS.map((e) => e as string)}
            onSelect={(selected) =>
              setInfo("expires", selected as ExpireLength)
            }
            targetClass="blue"
          />
          <Show
            when={!info.loading}
            fallback={
              <button class="green">
                <Loading type="spinner" />
              </button>
            }
          >
            <ConfirmButton class="green" onConfirm={createSecret}>
              create
            </ConfirmButton>
          </Show>
        </Flex>
      </Show>

      <Show when={info.secret}>
        <div style={{ "place-self": "center" }}>
          note. you cannot see this again once this menu closes
        </div>
        <Flex class="wrap" alignItems="center">
          <pre class="card dark">{info.secret}</pre>
          <ConfirmButton
            class="blue"
            onFirstClick={() => {
              copyToClipboard(info.secret!);
              pushNotification("good", "copied secret to clipboard");
            }}
            confirm={<Icon type="check" />}
          >
            <Icon type="clipboard" />
          </ConfirmButton>
        </Flex>
      </Show>
    </>
  );
};

function createExpires(length: ExpireLength) {
  if (length === "never") {
    return undefined;
  }
  const add_days = length === "30 days" ? 30 : length === "90 days" ? 90 : 365;
  const add_ms = add_days * 24 * 60 * 60 * 1000;
  return new Date(Date.now() + add_ms).toISOString();
}