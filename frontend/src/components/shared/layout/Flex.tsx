import { Component, JSX } from "solid-js";
import { combineClasses, filterOutFromObj } from "../../../util/helpers";
import s from "./layout.module.scss";

const Flex: Component<
  {
    gap?: string | number;
    alignItems?:
      | "flex-start"
      | "flex-end"
      | "center"
      | "baseline"
      | "stretch"
      | undefined;
    justifyContent?:
      | "flex-start"
      | "flex-end"
      | "center"
      | "stretch"
      | "space-between"
      | "space-around"
      | "space-evenly"
      | undefined;
    placeItems?: string;
    style?: JSX.CSSProperties;
    class?: string;
  } & JSX.HTMLAttributes<HTMLDivElement>
> = (p) => {
  return (
    <div
      {...filterOutFromObj(p, [
        "gap",
        "alignItems",
        "justifyContent",
        "placeItems",
        "style",
        "class",
      ])}
      class={combineClasses(s.Flex, p.class)}
      style={{
        gap: p.gap,
        "align-items": p.alignItems,
        "justify-content": p.justifyContent,
        "place-items": p.placeItems,
        ...(p.style as any),
      }}
    >
      {p.children}
    </div>
  );
};

export default Flex;
