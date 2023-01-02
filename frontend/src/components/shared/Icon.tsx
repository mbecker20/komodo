import { Component, JSX } from "solid-js";

export type IconType =
  | "arrow-down"
  | "arrow-up"
  | "arrow-left"
  | "chevron-down"
  | "chevron-up"
  | "cross"
  | "double-chevron-right"
  | "exchange"
  | "eye-off"
  | "eye-open"
  | "star-empty"
  | "star"
  | "chevron-left"
  | "trash"
  | "info-sign"
  | "menu"
  | "build"
  | "notifications"
  | "user"
  | "play"
  | "pause"
  | "reset"
  | "plus"
  | "minus"
  | "floppy-disk"
  | "command"
  | "log"
  | "console"
  | "application"
  | "error"
  | "refresh"
  | "cut"
  | "fullscreen"
  | "github"
  | "google"
  | "edit"
  | "clipboard"
  | "check"
  | "caret-right"
  | "search"
  | "cog"
  | "home"
  | "timeline-line-chart"
  | "arrow-right";

const ICON_DIR = import.meta.env.VITE_ICON_DIR || "/assets/icons"

const Icon: Component<{
  type: IconType;
  alt?: string;
  class?: string;
  style?: JSX.CSSProperties;
  width?: string;
  height?: string;
  onClick?: JSX.EventHandlerUnion<HTMLImageElement, MouseEvent>;
  title?: string;
}> = (p) => {
  return (
    <img
      class={p.class}
      src={`${ICON_DIR}/${p.type}.svg`}
      alt={p.alt || p.type}
      title={p.title}
      style={{
        ...p.style,
        width: p.width || "1rem",
        height: p.height,
      }}
      onClick={p.onClick}
    />
  );
};

export default Icon;
