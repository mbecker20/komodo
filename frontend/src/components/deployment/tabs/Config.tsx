import { Component } from "solid-js";
import s from "../deployment.module.css";
import { useToggle } from "../../../util/hooks";

const Config: Component<{}> = (p) => {
  const [editing, toggleEditing] = useToggle();
  return <div class={s.Config}>config</div>;
};

export default Config;
