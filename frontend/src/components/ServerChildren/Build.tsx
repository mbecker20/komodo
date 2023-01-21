import { A } from "@solidjs/router";
import { Component, Show } from "solid-js";
import { useAppState } from "../../state/StateProvider";
import {
  combineClasses,
  readableMonitorTimestamp,
} from "../../util/helpers";
import Flex from "../shared/layout/Flex";
import s from "./serverchildren.module.scss";

const Build: Component<{ id: string }> = (p) => {
  const { builds } = useAppState();
  const build = () => builds.get(p.id)!;
  const version = () => {
    return `v${build().version.major}.${build().version.minor}.${
      build().version.patch
    }`;
  };
  const lastBuiltAt = () => {
    if (
      build().last_built_at === undefined ||
      build().last_built_at?.length === 0 ||
      build().last_built_at === "never"
    ) {
      return "not built";
    } else {
      return readableMonitorTimestamp(build().last_built_at!);
    }
  };
  return (
    <Show when={build()}>
      <A href={`/build/${p.id}`} class={combineClasses(s.DropdownItem)}>
        <h2>{build().name}</h2>
        <Flex>
          <div>{version()}</div>
          <div style={{ opacity: 0.7 }}>{lastBuiltAt()}</div>
        </Flex>
      </A>
    </Show>
  );
};
export default Build;
