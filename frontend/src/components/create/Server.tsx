import { Server } from "@monitor/types";
import { Component, createSignal, onMount } from "solid-js";
import { createStore } from "solid-js/store";
import { pushNotification } from "../..";
import { ADD_SERVER } from "../../state/actions";
import { useAppState } from "../../state/StateProvider";
import { useToggle } from "../../util/hooks";
import Input from "../util/Input";
import Grid from "../util/layout/Grid";
import CenterMenu from "../util/menu/CenterMenu";

const AddServer: Component<{}> = (p) => {
  const [show, toggleShow] = useToggle();
  return (
    <CenterMenu
      show={show}
      toggleShow={toggleShow}
      title="add server"
      target="add server"
      targetClass="blue"
      targetStyle={{ width: "100%" }}
      content={<Content close={toggleShow} />}
    />
  );
};

const Content: Component<{ close: () => void }> = (p) => {
  const { ws } = useAppState();
  let nameInput: HTMLInputElement | undefined;
  const [server, setServer] = createStore<Server>({
    name: "",
    address: "",
    passkey: "",
    enabled: true,
  });
  onMount(() => nameInput?.focus());
  const create = () => {
    if (
      server.name.length > 0 &&
      server.address.length > 0 &&
      server.passkey.length > 0
    ) {
      ws.send(ADD_SERVER, {
        server,
      });
      p.close();
    } else {
			pushNotification("bad", "field empty. fill in field");
		}
  };
  return (
    <>
      <Grid
        placeItems="center"
        style={{ padding: "2rem 1rem 1rem 1rem" }}
      >
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
        <Input
          value={server.passkey}
          onEdit={(passkey) => setServer("passkey", passkey)}
          placeholder="passkey"
          style={{ "font-size": "1.5rem" }}
        />
        <button class="green" style={{ width: "100%" }} onClick={create}>
          add
        </button>
      </Grid>
    </>
  );
};

export default AddServer;
