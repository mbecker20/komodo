import { Component, JSX } from "solid-js";
import { combineClasses } from "../../../util/helpers";
import s from "./Layout.module.css";

const Grid: Component<{
  gap?: string | number;
  placeItems?: string;
  style?: JSX.CSSProperties;
  class?: string
} & JSX.HTMLAttributes<HTMLDivElement>> = (p) => {
  return (
    <div
      class={combineClasses(s.Grid, p.class)}
      style={{
        gap: p.gap,
        "place-items": p.placeItems,
        ...(p.style as any),
      }}
      {...p}
    >
      {p.children}
    </div>
  );
};

export default Grid;
