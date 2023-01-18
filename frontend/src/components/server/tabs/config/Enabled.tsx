import { Component } from "solid-js";
import { combineClasses } from "../../../../util/helpers";
import Flex from "../../../shared/layout/Flex";
import { useConfig } from "./Provider";

const Enabled: Component<{}> = (p) => {
  const { server, setServer, userCanUpdate } = useConfig();
  return (
    <Flex
      class={combineClasses("config-item shadow")}
      justifyContent="space-between"
      alignItems="center"
    >
      <h1>enabled</h1>
      <button
        class={server.enabled ? "green" : "red"}
        onClick={() => setServer("enabled", !server.enabled)}
        disabled={!userCanUpdate()}
      >
        {server.enabled ? "yes" : "no"}
      </button>
    </Flex>
  );
};

export default Enabled;
