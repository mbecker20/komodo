import { Component } from "solid-js";
import { combineClasses } from "../../../../util/helpers";
import Input from "../../../util/Input";
import Flex from "../../../util/layout/Flex";
import s from "../../server.module.css";
import { useConfig } from "./Provider";

const Passkey: Component<{}> = (p) => {
  const { server, setServer } = useConfig();
  return (
    <Flex
      class={combineClasses(s.ConfigItem, "shadow")}
      justifyContent="space-between"
    >
      <h1>passkey</h1>
      <Input
        value={server.passkey}
        placeholder="passkey"
        onConfirm={(value) => setServer("passkey", value)}
        style={{ width: "13rem" }}
      />
    </Flex>
  );
};

export default Passkey;
