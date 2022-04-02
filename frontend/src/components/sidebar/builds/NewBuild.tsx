import { Component, Show } from "solid-js";
import { CREATE_BUILD } from "../../../state/actions";
import { useAppState } from "../../../state/StateProvider";
import { useToggle } from "../../../util/hooks";
import ConfirmButton from "../../util/ConfirmButton";
import Icon from "../../util/icons/Icon";
import Input from "../../util/Input";
import Flex from "../../util/layout/Flex";
import New from "../New";

const NewBuild: Component<{}> = (p) => {
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
      <New 
        placeholder="name build"
        create={create}
        close={toggleShowNew}
      />
    </Show>
  );
}

export default NewBuild;