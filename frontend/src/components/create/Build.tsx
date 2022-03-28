import { Component, createSignal, onMount } from "solid-js";
import { pushNotification } from "../..";
import { CREATE_BUILD } from "../../state/actions";
import { useAppState } from "../../state/StateProvider";
import { useToggle } from "../../util/hooks";
import Icon from "../util/icons/Icon";
import Input from "../util/Input";
import Flex from "../util/layout/Flex";
import CenterMenu from "../util/menu/CenterMenu";

const CreateBuild: Component<{}> = (p) => {
  const [show, toggleShow] = useToggle();
  return (
    <CenterMenu
      show={show}
      toggleShow={toggleShow}
      title="create build"
      target={<Icon type="plus" />}
      targetClass="green"
      targetStyle={{ width: "100%" }}
      content={<Content close={toggleShow} />}
    />
  );
};

const Content: Component<{ close: () => void }> = (p) => {
  const { ws } = useAppState();
  let nameInput: HTMLInputElement | undefined;

  const [name, setName] = createSignal("");
  onMount(() => nameInput?.focus());
  const create = () => {
    if (name().length > 0) {
      ws.send(CREATE_BUILD, {
        build: { name: name() },
      });
      p.close();
    } else {
      pushNotification("bad", "please provide a name");
    }
  };
  return (
    <>
      <Flex
        alignItems="center"
        justifyContent="space-between"
        style={{ padding: "2rem 1rem 1rem 1rem" }}
      >
        <Input
          ref={nameInput}
          value={name()}
          onEdit={setName}
          placeholder="name"
          style={{ "font-size": "1.5rem" }}
          onConfirm={create}
        />
        <button class="green" style={{ width: "100%" }} onClick={create}>
          create
        </button>
      </Flex>
    </>
  );
};

export default CreateBuild;
