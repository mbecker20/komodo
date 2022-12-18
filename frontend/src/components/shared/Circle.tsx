import { Component, JSX } from "solid-js";

const Circle: Component<{
  size?: number;
  class?: string;
  color?: string;
  style?: JSX.CSSProperties;
}> = (p) => {
  return (
    <div
      class={p.class}
      style={{
        width: `${p.size || 1}rem`,
        height: `${p.size || 1}rem`,
        "border-radius": `${p.size ? p.size / 2 : 0.75}rem`,
        "background-color": p.color,
        "transition": "all 250ms ease-in-out",
        ...p.style
      }}
    />
  );
};

export default Circle;
