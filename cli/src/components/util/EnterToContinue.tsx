import React from "react";
import { Text } from "ink";
import { useEnter, useEsc } from "../../util/hooks";

const EnterToContinue = ({ onEnter, pressEnterTo, onEsc }: { onEnter: () => void; pressEnterTo?: string; onEsc?: () => void; }) => {
	useEnter(onEnter);
  useEsc(() => onEsc && onEsc());
  return (
    <Text>
      press{" "}
      <Text color="green" bold>
        enter
      </Text>{" "}
      to {pressEnterTo || "continue"}.
    </Text>
  );
};

export default EnterToContinue;
