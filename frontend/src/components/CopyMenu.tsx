import { useNavigate } from "@solidjs/router";
import { Component, createSignal, Show } from "solid-js";
import { client, pushNotification } from "..";
import { useAppState } from "../state/StateProvider";
import { Build, Deployment } from "../types";
import { getId } from "../util/helpers";
import { useToggle } from "../util/hooks";
import ConfirmButton from "./shared/ConfirmButton";
import Icon from "./shared/Icon";
import Input from "./shared/Input";
import Flex from "./shared/layout/Flex";
import Grid from "./shared/layout/Grid";
import CenterMenu from "./shared/menu/CenterMenu";
import Selector from "./shared/menu/Selector";

const CopyMenu: Component<{
  type: "deployment" | "build";
  id: string;
}> = (p) => {
	const navigate = useNavigate();
  const [show, toggleShow] = useToggle();
  const [newName, setNewName] = createSignal("");
  const { builds, deployments, servers } = useAppState();
  const curr_server = () => {
    if (p.type === "build") {
      return builds.get(p.id)!.server_id;
    } else {
      return deployments.get(p.id)!.deployment.server_id;
    }
  }
  const [selectedId, setSelected] = createSignal(curr_server());
  const name = () => {
    if (p.type === "build") {
      return builds.get(p.id)?.name;
    } else if (p.type === "deployment") {
      return deployments.get(p.id)?.deployment.name;
    }
  };
	const copy = () => {
    if (newName().length !== 0) {
      let promise: Promise<Build | Deployment>;
      if (p.type === "build") {
        promise = client.copy_build(p.id, {
          name: newName(),
        });
      } else {
        promise = client.copy_deployment(p.id, {
          name: newName(),
          server_id: selectedId()!,
        });
      }
      toggleShow();
      promise.then((val) => {
        navigate(`/${p.type}/${getId(val)}`);
      });
    } else {
      pushNotification("bad", "copy name cannot be empty");
    }
  };
  return (
    <CenterMenu
      show={show}
      toggleShow={toggleShow}
      title={`copy ${p.type} | ${name()}`}
      target={<Icon type="duplicate" />}
      targetClass="blue"
      content={() => (
        <Grid placeItems="center">
          <Flex alignItems="center">
            <Input
              placeholder="copy name"
              class="card dark"
              style={{ padding: "0.5rem" }}
              value={newName()}
              onEdit={setNewName}
            />
            <Show when={p.type === "deployment"}>
              <Selector
                label="target: "
                selected={selectedId()!}
                items={servers.ids()!}
                onSelect={setSelected}
                itemMap={(id) => servers.get(id)!.server.name}
                targetClass="blue"
                targetStyle={{ display: "flex", gap: "0.5rem" }}
                searchStyle={{ width: "100%" }}
                position="bottom right"
                useSearch
              />
            </Show>
          </Flex>
          <ConfirmButton
            class="green"
            style={{ width: "100%" }}
            onConfirm={copy}
          >
            copy {p.type}
          </ConfirmButton>
        </Grid>
      )}
      position="center"
    />
  );
};

export default CopyMenu;
