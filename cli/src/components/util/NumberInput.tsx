import React from "react";
import { Text } from "ink";
import TextInput from "ink-text-input";
import { useState } from "react";
import { useBlinker, useEsc } from "../../util/hooks";

const NumberInput = ({
  initialValue,
  onSubmit,
  onEsc,
}: {
  initialValue: number;
  onSubmit?: (val: number) => void;
  onEsc?: () => void;
}) => {
	const [value, setValue] = useState(initialValue?.toString() || "");
	const [error, setError] = useState(isNaN(Number(value)));
  useEsc(onEsc ? onEsc : () => {});
  return (
    <Text>
      <TextInput
        value={value}
        onChange={(val: string) => {
          setError(isNaN(Number(val)));
          setValue(val);
        }}
        onSubmit={(val: string) => {
          const value = Number(val);
          if (val) {
            onSubmit && onSubmit(value);
          } else {
            setError(true);
          }
        }}
      />
      {error && <Text color="gray"> (please enter a number)</Text>}
    </Text>
  );
};

export default NumberInput;