import { Component } from "solid-js";
import { useAppState } from "../../state/StateProvider";
import { inPx } from "../../util/helpers";
import Icon from "../util/icons/Icon";
import Flex from "../util/layout/Flex";
import s from "./topbar.module.css";

export const TOPBAR_HEIGHT = 40;

const Topbar: Component<{}> = (p) => {
  const { sidebar } = useAppState();
  return (
    <Flex
      class={s.Topbar}
      justifyContent="space-between"
      style={{ height: inPx(TOPBAR_HEIGHT) }}
    >
      {/* right side */}
      <Flex alignItems="center">
        <button onClick={sidebar.toggle}>
          <Icon type="menu" width="1.5rem" />
        </button>
        <div class={s.Monitor}>monitor</div>
      </Flex>

      {/* left side */}
      <Flex></Flex>
    </Flex>
  );
};

export default Topbar;
