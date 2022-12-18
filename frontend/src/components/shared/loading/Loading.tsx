import { Component, JSX, Match, Switch } from "solid-js";
import { sonarCss, spinnerCss, threeDotsCss } from "./css";

export type LoadingType = "three-dot" | "sonar" | "spinner";

const Loading: Component<{
  type?: LoadingType;
  scale?: number;
  style?: JSX.CSSProperties;
}> = (p) => {
  return (
    <Switch>
      <Match when={p.type === "three-dot"}>
        <style>{threeDotsCss(p.scale)}</style>
        <div class="ThreeDot" style={p.style}>
          <div></div>
          <div></div>
          <div></div>
          <div></div>
        </div>
      </Match>
      <Match when={p.type === "sonar"}>
        <style>{sonarCss(p.scale)}</style>
        <div class="Sonar" style={p.style}>
          <div></div>
          <div></div>
        </div>
      </Match>
      <Match when={p.type === "spinner" || p.type === undefined}>
        <style>{spinnerCss(p.scale)}</style>
        <div class="Spinner" style={p.style}>
          <div></div>
          <div></div>
          <div></div>
          <div></div>
        </div>
      </Match>
    </Switch>
  );
};

export default Loading;
