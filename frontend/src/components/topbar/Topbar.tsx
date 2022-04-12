import { Component, createSignal } from "solid-js";
import { useAppState } from "../../state/StateProvider";
import { useUser } from "../../state/UserProvider";
import { combineClasses, inPx } from "../../util/helpers";
import Icon from "../util/Icon";
import Flex from "../util/layout/Flex";
import Menu from "../util/menu/Menu";
import Account from "./Account";
import Search from "./Search";
import s from "./topbar.module.scss";
import Updates from "./Updates";

export const TOPBAR_HEIGHT = 40;

const Topbar: Component = () => {
  const { sidebar, selected } = useAppState();
  const { username } = useUser();
  const [menu, setMenu] = createSignal<"updates" | "account">();
  const close = () => setMenu(undefined);
  return (
    <Flex
      class={combineClasses(s.Topbar, "shadow")}
      justifyContent="space-between"
      alignItems="center"
      style={{ height: inPx(TOPBAR_HEIGHT) }}
    >
      {/* right side */}
      <Flex alignItems="center" style={{ padding: "0rem 0.5rem" }}>
        <button onClick={sidebar.toggle}>
          <Icon type="menu" width="1.5rem" />
        </button>
        <div class={s.Monitor} onClick={() => selected.set("", "home")}>
          monitor
        </div>
      </Flex>
      {/* left side */}
      <Flex gap="0.5rem" alignItems="center" style={{ padding: "0rem 0.5rem" }}>
        <Search />
        <Menu
          show={menu() === "updates"}
          close={close}
          target={
            <button
              onClick={() =>
                menu() === "updates" ? setMenu(undefined) : setMenu("updates")
              }
            >
              <Icon type="notifications" alt="updates" width="1.5rem" />
            </button>
          }
          content={<Updates />}
          position="bottom right"
        />
        <Menu
          show={menu() === "account"}
          close={close}
          target={
            <button
              onClick={() =>
                menu() === "account" ? setMenu(undefined) : setMenu("account")
              }
            >
              {username()}
              <Icon type="chevron-down" />
            </button>
          }
          content={<Account close={close} />}
          position="bottom right"
        />
      </Flex>
    </Flex>
  );
};

export default Topbar;
