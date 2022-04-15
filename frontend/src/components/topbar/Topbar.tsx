import { Component, createSignal, Show } from "solid-js";
import { useAppDimensions } from "../../state/DimensionProvider";
import { useAppState } from "../../state/StateProvider";
import { useUser } from "../../state/UserProvider";
import { combineClasses, inPx } from "../../util/helpers";
import Circle from "../util/Circle";
import Icon from "../util/Icon";
import Flex from "../util/layout/Flex";
import HoverMenu from "../util/menu/HoverMenu";
import Menu from "../util/menu/Menu";
import Account from "./Account";
import { SearchProvider } from "./Search/Provider";
import Search from "./Search/Search";
import s from "./topbar.module.scss";
import Updates from "./Updates";

export const TOPBAR_HEIGHT = 40;

const Topbar: Component = () => {
  const { sidebar, selected, ws } = useAppState();
  const { width } = useAppDimensions();
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
        <button class="grey" onClick={sidebar.toggle}>
          <Icon type="menu" width="1.15rem" />
        </button>
        <Show when={width() > 500}>
          <div class={s.Monitor} onClick={() => selected.set("", "home")}>
            monitor
          </div>
        </Show>
        <HoverMenu
          target={
            <Circle
              size={1}
              class={ws.isOpen() ? "green" : "red"}
              style={{ transition: "all 500ms ease-in-out" }}
            />
          }
          content={ws.isOpen() ? "connected" : "disconnected"}
          position="right center"
        />
      </Flex>
      {/* left side */}
      <Flex gap="0.5rem" alignItems="center" style={{ padding: "0rem 0.5rem" }}>
        <SearchProvider>
          <Search />
        </SearchProvider>
        <Menu
          show={menu() === "updates"}
          close={close}
          target={
            <button
              class="grey"
              onClick={() =>
                menu() === "updates" ? setMenu(undefined) : setMenu("updates")
              }
            >
              <Icon type="notifications" alt="updates" width="1.15rem" />
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
              class="grey"
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
