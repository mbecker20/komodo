import { Component, JSXElement } from "solid-js";
import Flex from "../layout/Flex";

const Toggle: Component<{
  label?: JSXElement;
  toggled: boolean;
  onChange?: (toggled: boolean) => void;
}> = (p) => {
  return (
    <Flex alignItems="center" justifyContent="space-between">
      {p.label}
      <label class="switch">
        <input
          type="checkbox"
          checked={p.toggled}
          onInput={() => {
            p.onChange && p.onChange(!p.toggled);
          }}
        />
        <span class="slider round"></span>
      </label>
    </Flex>
  );
};

export default Toggle;
