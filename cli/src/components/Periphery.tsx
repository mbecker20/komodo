import React from "react";
import { Text } from "ink";
import { Next } from "../types";
import LabelledSelector from "./util/LabelledSelector";

const Periphery = ({
  setPeriphery,
  next,
}: {
  setPeriphery: (periphery: boolean) => void;
  next: Next;
}) => {
  return (
    <LabelledSelector
      label={
        <Text>
          Are you setting up <Text color="cyan">monitor core</Text> or{" "}
          <Text color="red">monitor periphery</Text>?
        </Text>
      }
      items={["core", "periphery"]}
      onSelect={(item) => {
        setPeriphery(item === "periphery");
        next();
      }}
      direction="vertical"
    />
  );
};

export default Periphery;
