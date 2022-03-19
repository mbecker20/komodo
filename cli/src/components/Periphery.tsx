import React from "react";
import { Text } from "ink";
import LabelledSelector from "./util/LabelledSelector";
import { useMainSequence } from "../cli";

const Periphery = ({
  setPeriphery,
}: {
  setPeriphery: (periphery: boolean) => void;
}) => {
  const { next } = useMainSequence();
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
      vertical
    />
  );
};

export default Periphery;
