import { Component } from "solid-js";
import { useTheme } from "../../../../state/ThemeProvider";
import { combineClasses } from "../../../../util/helpers";
import Input from "../../../util/Input";
import Flex from "../../../util/layout/Flex";
import { useConfig } from "./Provider";

const Passkey: Component<{}> = (p) => {
  const { server, setServer } = useConfig();
  const { themeClass } = useTheme();
  return (
    <Flex
      class={combineClasses("config-item shadow", themeClass())}
      justifyContent="space-between"
    >
      <h1>passkey</h1>
      <Input
        value={server.passkey}
        placeholder="using default"
        onEdit={(value) => setServer("passkey", value)}
        style={{ width: "13rem" }}
      />
    </Flex>
  );
};

export default Passkey;
