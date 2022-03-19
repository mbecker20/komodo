import { Box, Newline, Text } from "ink";
import React, { ReactNode } from "react";
import Selector from "./Selector";

const LabelledSelector = ({
  label,
  items,
  onSelect,
  vertical,
  labelColor = "white",
}: {
  label: ReactNode;
  labelColor?: "green" | "white";
  items: string[];
  onSelect?: (item: string, index: number) => void;
  vertical?: boolean;
}) => {
  return (
    <Box flexDirection={vertical ? "column" : "row"}>
      {typeof label === "string" ? (
        <Text color={labelColor}>{label} </Text>
      ) : (
        label
      )}
      {vertical && <Newline />}
      <Selector items={items} onSelect={onSelect} />
    </Box>
  );
};

export default LabelledSelector;
