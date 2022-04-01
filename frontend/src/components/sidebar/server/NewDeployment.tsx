import { Component, createEffect, createSignal, Show } from "solid-js";
import { pushNotification } from "../../..";
import { CREATE_DEPLOYMENT } from "../../../state/actions";
import { defaultDeployment } from "../../../state/defaults";
import { useAppState } from "../../../state/StateProvider";
import { useToggle } from "../../../util/hooks";
import ConfirmButton from "../../util/ConfirmButton";
import Icon from "../../util/icons/Icon";
import Input from "../../util/Input";
import Flex from "../../util/layout/Flex";
import s from "../sidebar.module.css";

const NewDeployment: Component<{ serverID: string }> = (p) => {
	const { ws } = useAppState();
	const [showNew, toggleShowNew] = useToggle();
  const [name, setName] = createSignal("");
  const create = () => {
    if (name().length > 0) {
      ws.send(CREATE_DEPLOYMENT, {
        deployment: defaultDeployment(name(), p.serverID),
      });
      setName("");
      toggleShowNew();
    } else {
      pushNotification("bad", "please provide a name");
    }
  };
  let inputRef: HTMLInputElement | undefined;
  createEffect(() => {
    if (showNew()) inputRef?.focus();
  });
	return (
    <Show
      when={showNew()}
      fallback={
        <button class="green" onClick={toggleShowNew} style={{ width: "100%" }}>
          <Icon type="plus" />
        </button>
      }
    >
      <Flex gap="0.2rem" justifyContent="space-between">
        <Input
          ref={inputRef}
          placeholder="name deployment"
          value={name()}
          onEdit={setName}
          style={{ width: "11rem" }}
        />
        <Flex gap="0.4rem">
          <ConfirmButton color="green" onConfirm={create}>
            create
          </ConfirmButton>
          <button
            class="red"
            onClick={() => {
              toggleShowNew();
              setName("");
            }}
          >
            <Icon type="cross" />
          </button>
        </Flex>
      </Flex>
    </Show>
  );
}

export default NewDeployment;