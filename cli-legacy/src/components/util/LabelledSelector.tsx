import { Box, Newline, Text } from "ink";
import React, { ReactNode } from "react";
import Selector from "./Selector";

const LabelledSelector = ({
  label,
  items,
  onSelect,
  direction = "horizontal",
  labelColor = "green"
}: {
  label: ReactNode;
  labelColor?: "green" | "white";
  items: string[];
  onSelect?: (item: string, index: number) => void;
  direction?: "vertical" | "horizontal"
}) => {
  return (
    <Box flexDirection={direction === "horizontal" ? "row" : "column"}>
      {typeof label === "string" ? <Text color={labelColor}>{label} </Text> : label}
      {direction === "vertical" && <Newline />}
      <Selector items={items} onSelect={onSelect} />
    </Box>
  );
};

export default LabelledSelector;
