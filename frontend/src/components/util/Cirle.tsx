import { Component, JSX } from "solid-js";

const Cirle: Component<{
  size?: number;
  class?: string;
  color?: string;
  style: JSX.CSSProperties;
}> = (p) => {
  return (
    <div
      class={p.class}
      style={{
        width: `${p.size || 1.5}rem`,
        height: `${p.size || 1.5}rem`,
        "border-radius": `${p.size ? p.size / 2 : 0.75}rem`,
        "background-color": p.color,
        ...p.style
      }}
    />
  );
};

export default Cirle;
