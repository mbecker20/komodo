import React from "react";
import { Text } from "ink";
import { useEnter } from "../../util/hooks";

const EnterToContinue = ({ onEnter, pressEnterTo }: { onEnter: () => void; pressEnterTo?: string }) => {
	useEnter(onEnter);
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
