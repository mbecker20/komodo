import { Component, createSignal, onMount } from "solid-js";
import { pushNotification } from "../..";
import { useKeyDown } from "../../util/hooks";
import Icon from "../util/Icon";
import Input from "../util/Input";
import Flex from "../util/layout/Flex";

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
    <Flex gap="0.2rem" justifyContent="space-between">
      <Input
        ref={inputRef}
        placeholder={p.placeholder}
        value={name()}
        onEdit={setName}
        onEnter={create}
        style={{ width: "11rem" }}
      />
      <Flex gap="0.4rem">
        <button
          class="green"
          onClick={create}
        >
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

export default New;