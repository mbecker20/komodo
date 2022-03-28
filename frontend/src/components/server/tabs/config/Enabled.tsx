import { Component } from "solid-js";
import { combineClasses } from "../../../../util/helpers";
import Flex from "../../../util/layout/Flex";
import s from "../../server.module.css";
import { useConfig } from "./Provider";

const Enabled: Component<{}> = (p) => {
  const { server, setServer } = useConfig();
  return (
    <Flex
      class={combineClasses(s.ConfigItem, "shadow")}
      justifyContent="space-between"
      alignItems="center"
    >
      <h1>enabled</h1>
      <button class={server.enabled ? "green" : "red"} onClick={() => setServer("enabled", !server.enabled)}>
        {server.enabled ? "yes" : "no"}
      </button>
    </Flex>
  );
};

export default Enabled;
