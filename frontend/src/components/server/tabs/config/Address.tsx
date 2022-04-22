import { Component } from "solid-js";
import { useTheme } from "../../../../state/ThemeProvider";
import { combineClasses } from "../../../../util/helpers";
import Input from "../../../util/Input";
import Flex from "../../../util/layout/Flex";
import { useConfig } from "./Provider";

const Address: Component<{}> = (p) => {
  const { server, setServer } = useConfig();
  const { themeClass } = useTheme();
  return (
    <Flex
      class={combineClasses("config-item shadow", themeClass())}
      justifyContent="space-between"
    >
      <h1>address</h1>
      <Input
        value={server.address}
        placeholder="address"
        onEdit={(value) => setServer("address", value)}
        style={{ width: "13rem" }}
      />
    </Flex>
  );
};

export default Address;
