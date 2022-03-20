import React from "react";
import TextInput, { UncontrolledTextInput } from "ink-text-input";
import { useBlinker, useEsc } from "../../util/hooks";

export const Input = ({
  initialValue,
  onSubmit,
  onEsc,
}: {
  initialValue?: string;
  onSubmit?: (val: string) => void;
  onEsc?: () => void;
}) => {
  useEsc(onEsc ? onEsc : () => {});
  return (
    <UncontrolledTextInput
      initialValue={initialValue}
      onSubmit={onSubmit}
    />
  );
};

export const ControlledInput = ({
  value,
  onChange,
  onSubmit,
  onEsc,
}: {
  value: string;
  onChange: (val: string) => void;
  onSubmit?: (val: string) => void;
  onEsc?: () => void;
}) => {
  useEsc(onEsc ? onEsc : () => {});
  return (
    <TextInput
      value={value}
      onChange={onChange}
      onSubmit={onSubmit}
    />
  );
};
