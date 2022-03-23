import { Update as UpdateType } from "@monitor/types";
import { Component } from "solid-js";
import s from "../topbar.module.css";

const Update: Component<{ update: UpdateType }> = (p) => {
  return <div class={s.Update}></div>;
};

export default Update;