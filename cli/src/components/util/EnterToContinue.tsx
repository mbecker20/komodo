import React from "react";
import { Text } from "ink";
import { useEnter } from "../../hooks";

const EnterToContinue = ({ onEnter }: { onEnter: () => void }) => {
	useEnter(onEnter);
  return (
    <Text>
      press{" "}
      <Text color="green" bold>
        enter
      </Text>{" "}
      to continue.
    </Text>
  );
};

export default EnterToContinue;
