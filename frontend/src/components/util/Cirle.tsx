import { Component } from "solid-js";

const Cirle: Component<{ size?: number; color?: string }> = (p) => {
  return (
    <div
      style={{
        width: `${p.size || 1.5}rem`,
        height: `${p.size || 1.5}rem`,
        "border-radius": `${p.size ? p.size / 2 : 0.75}rem`,
      }}
    />
  );
};

export default Cirle;
