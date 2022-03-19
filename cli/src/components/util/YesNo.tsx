import React, { ReactNode } from "react";
import LabelledSelector from "./LabelledSelector";

const YesNo = ({
  label,
  onYes,
  onNo,
  onSelect,
  direction,
  labelColor,
	noYes
}: {
  label: ReactNode;
  onYes?: () => void;
  onNo?: () => void;
  onSelect?: (res: "yes" | "no") => void;
  direction?: "vertical" | "horizontal";
  labelColor?: "green" | "white";
  noYes?: boolean;
}) => {
  return (
    <LabelledSelector
      label={label}
      items={noYes ? ["no", "yes"] : ["yes", "no"]}
      onSelect={(item) => {
        if (item === "yes") {
          onYes && onYes();
        } else {
          onNo && onNo();
        }
        onSelect && onSelect(item as "yes" | "no");
      }}
      direction={direction}
      labelColor={labelColor}
    />
  );
};

export default YesNo;
