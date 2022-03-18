import React, { ReactNode } from "react";
import LabelledSelector from "./LabelledSelector";

const YesNo = ({
  label,
  onYes,
	onNo,
	direction,
	labelColor
}: {
  label: ReactNode;
  onYes?: () => void;
	onNo?: () => void;
	direction?: "vertical" | "horizontal";
	labelColor?: "green" | "white"
}) => {
  return (
    <LabelledSelector
			label={label}
			items={["yes", "no"]}
			onSelect={(item) => {
				if (item === "yes") {
					onYes && onYes();
				} else {
					onNo && onNo()
				}
			}}
			direction={direction}
			labelColor={labelColor}
		/>
  );
};

export default YesNo;
