import React, { useEffect } from "react";
import { Text } from "ink";
import LabelledSelector from "./util/LabelledSelector";
import { useConfig, useMainSequence } from "../cli";
import { useEsc } from "../util/hooks";

const IsPeriphery = ({
  setPeriphery,
}: {
  setPeriphery: (periphery: boolean) => void;
}) => {
  const { next, prev } = useMainSequence();
  const { setMany } = useConfig();
  useEffect(() => {
    setMany(
      ["core", undefined],
      ["periphery", undefined],
      ["mongo", undefined],
    );
  }, []);
  useEsc(prev);
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

export default IsPeriphery;
