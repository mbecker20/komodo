import s from "./menu.module.scss";

export type Position =
  | "left"
  | "left center"
  | "right"
  | "right center"
  | "bottom"
  | "bottom right"
  | "bottom center";

export function getPositionClass(position: Position = "bottom") {
  switch (position) {
    case "left":
      return s.Left;
    case "left center":
      return s.LeftCenter;
    case "right":
      return s.Right;
    case "right center":
      return s.RightCenter;
    case "bottom":
      return s.Bottom;
    case "bottom right":
      return s.BottomRight;
    case "bottom center":
      return s.BottomCenter;
  }
}
