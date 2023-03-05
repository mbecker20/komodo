import { Component, JSX } from "solid-js";
import Flex from "./layout/Flex";

const CheckBox: Component<{
  label: JSX.Element;
  checked: boolean;
  toggle: () => void;
}> = (p) => {
  return (
    <button
      class="blue"
      style={{ gap: "0.5rem" }}
      onClick={(e) => {
        e.stopPropagation();
        p.toggle();
      }}
    >
      {p.label}
      <input
        type="checkbox"
        checked={p.checked}
        style={{
          width: "fit-content",
          margin: 0,
          appearance: "auto",
          "-webkit-appearance": "checkbox",
        }}
      />
    </button>
  );
};

export default CheckBox;
