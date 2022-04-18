import { Component, Show } from "solid-js";
import { CREATE_DEPLOYMENT } from "@monitor/util";
import { defaultDeployment } from "../../../state/defaults";
import { useAppState } from "../../../state/StateProvider";
import { useToggle } from "../../../util/hooks";
import Icon from "../../util/Icon";
import New from "../New";

const NewDeployment: Component<{ serverID: string }> = (p) => {
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

export default NewDeployment;
