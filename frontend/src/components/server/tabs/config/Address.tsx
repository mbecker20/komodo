import { Component } from "solid-js";
import Input from "../../../util/Input";
import Flex from "../../../util/layout/Flex";
import { useConfig } from "./Provider";

const Address: Component<{}> = (p) => {
  const { server, setServer } = useConfig();
  return (
    <Flex
      class="config-item shadow"
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
