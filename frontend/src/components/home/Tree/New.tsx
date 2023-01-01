import { Component, createSignal, onMount, Show } from "solid-js";
import { client, pushNotification } from "../../..";
import { useKeyDown, useToggle } from "../../../util/hooks";
import Icon from "../../shared/Icon";
import Input from "../../shared/Input";
import Flex from "../../shared/layout/Flex";

export const NewDeployment: Component<{ serverID: string }> = (p) => {
  const [showNew, toggleShowNew] = useToggle();
  const create = (name: string) => {
    client.create_deployment({ name, server_id: p.serverID });
  };
  return (
    <Show
      when={showNew()}
      fallback={
        <button class="green" onClick={toggleShowNew} style={{ width: "100%" }}>
          <Icon type="plus" />
        </button>
      }
    >
      <New
        create={create}
        close={toggleShowNew}
        placeholder="name deployment"
      />
    </Show>
  );
};

export const NewBuild: Component<{ serverID: string }> = (p) => {
  const [showNew, toggleShowNew] = useToggle();
  const create = (name: string) => {
    client.create_build({ name, server_id: p.serverID });
  };
  return (
    <Show
      when={showNew()}
      fallback={
        <button class="green" onClick={toggleShowNew} style={{ width: "100%" }}>
          <Icon type="plus" />
        </button>
      }
    >
      <New placeholder="name build" create={create} close={toggleShowNew} />
    </Show>
  );
};

const New: Component<{
  create: (value: string) => void;
  close: () => void;
  placeholder: string;
}> = (p) => {
  const [name, setName] = createSignal("");
  let inputRef: HTMLInputElement | undefined;
  onMount(() => {
    inputRef?.focus();
  });
  useKeyDown("Escape", p.close);
  const create = () => {
    if (name().length > 0) {
      p.create(name());
      setName("");
      p.close();
    } else {
      pushNotification("bad", "please provide a name");
    }
  };
  return (
    <Flex justifyContent="space-between">
      <Input
        ref={inputRef}
        placeholder={p.placeholder}
        value={name()}
        onEdit={setName}
        onEnter={create}
        style={{ width: "20rem" }}
      />
      <Flex gap="0.4rem">
        <button class="green" onClick={create}>
          create
        </button>
        {/* <ConfirmButton
          color="green"
          onConfirm={create}
        >
          create
        </ConfirmButton> */}
        <button class="red" onClick={p.close}>
          <Icon type="cross" />
        </button>
      </Flex>
    </Flex>
  );
};