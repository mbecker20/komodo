import { Component, JSX } from "solid-js";
import { useTheme } from "../../state/ThemeProvider";
import { combineClasses } from "../../util/helpers";

const Button: Component<JSX.ButtonHTMLAttributes<HTMLButtonElement>> = (p) => {
  const { themeClass } = useTheme();
  return <button {...p} class={combineClasses(p.class, themeClass())} />;
};

export default Button;
