import { CREATE_BUILD, CREATE_DEPLOYMENT } from "@monitor/util";
import { Component, createSignal, onMount, Show } from "solid-js";
import { pushNotification } from "../../..";
import { defaultDeployment } from "../../../state/defaults";
import { useAppState } from "../../../state/StateProvider";
import { useKeyDown, useToggle } from "../../../util/hooks";
import Icon from "../../util/Icon";
import Input from "../../util/Input";
import Flex from "../../util/layout/Flex";

export const NewDeployment: Component<{ serverID: string }> = (p) => {
  const { ws } = useAppState();
  const [showNew, toggleShowNew] = useToggle();
  const create = (name: string) => {
    ws.send(CREATE_DEPLOYMENT, {
      deployment: defaultDeployment(name, p.serverID),
    });
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

export const NewBuild: Component<{}> = (p) => {
  const { ws } = useAppState();
  const [showNew, toggleShowNew] = useToggle();
  const create = (name: string) => {
    ws.send(CREATE_BUILD, {
      build: { name },
    });
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