import { Component, onMount } from "solid-js";
import { createStore } from "solid-js/store";
import { client, pushNotification } from "../../..";
import { useAppState } from "../../../state/StateProvider";
import { CreateServerBody } from "../../../util/client_types";
import { useToggle } from "../../../util/hooks";
import Input from "../../shared/Input";
import Grid from "../../shared/layout/Grid";
import CenterMenu from "../../shared/menu/CenterMenu";

const AddServer: Component<{}> = () => {
  const [show, toggleShow] = useToggle();
  return (
    <CenterMenu
      show={show}
      toggleShow={toggleShow}
      title="add server"
      target="add server"
      targetClass="green shadow"
      targetStyle={{ width: "100%" }}
      content={() => <Content close={toggleShow} />}
      position="center"
    />
  );
};

const Content: Component<{ close: () => void }> = (p) => {
  const { ws } = useAppState();
  let nameInput: HTMLInputElement | undefined;
  const [server, setServer] = createStore<CreateServerBody>({
    name: "",
    address: "",
  });
  onMount(() => nameInput?.focus());
  const create = async () => {
    if (server.name.length > 0 && server.address.length > 0) {
      await client.create_server(server);
      p.close();
    } else {
      pushNotification("bad", "a field is empty. fill in all fields");
    }
  };
  return (
    <Grid placeItems="center" style={{ padding: "2rem 1rem 1rem 1rem" }}>
      <Input
        ref={nameInput}
        value={server.name}
        onEdit={(name) => setServer("name", name)}
        placeholder="name"
        style={{ "font-size": "1.5rem" }}
      />
      <Input
        value={server.address}
        onEdit={(address) => setServer("address", address)}
        placeholder="address"
        style={{ "font-size": "1.5rem" }}
      />
      <button class="green" style={{ width: "100%" }} onClick={create}>
        add
      </button>
    </Grid>
  );
};

export default AddServer;
