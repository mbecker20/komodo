import { Component, Match, Switch } from "solid-js";
import { sonarCss, spinnerCss, threeDotsCss } from "./css";

export type LoadingType = "three-dot" | "sonar" | "spinner";

const Loading: Component<{ type?: LoadingType; scale?: number }> = (p) => {
  return (
    <Switch>
      <Match when={p.type === "three-dot"}>
        <style>{threeDotsCss(p.scale)}</style>
        <div class="ThreeDot">
          <div></div>
          <div></div>
          <div></div>
          <div></div>
        </div>
      </Match>
      <Match when={p.type === "sonar"}>
        <style>{sonarCss(p.scale)}</style>
        <div class="Sonar">
          <div></div>
          <div></div>
        </div>
      </Match>
      <Match when={p.type === "spinner" || p.type === undefined}>
        <style>{spinnerCss(p.scale)}</style>
        <div class="Spinner">
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
