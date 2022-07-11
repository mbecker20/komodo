import { Component, createSignal, JSX, Show } from "solid-js";
import { TOPBAR_HEIGHT } from "../..";
import { useAppDimensions } from "../../state/DimensionProvider";
import { useAppState } from "../../state/StateProvider";
import { useTheme } from "../../state/ThemeProvider";
import { useUser } from "../../state/UserProvider";
import { combineClasses, inPx } from "../../util/helpers";
import Button from "../util/Button";
import Circle from "../util/Circle";
import Icon from "../util/Icon";
import Flex from "../util/layout/Flex";
import HoverMenu from "../util/menu/HoverMenu";
import Menu from "../util/menu/Menu";
import Account from "./Account";
import { SearchProvider } from "./Search/Provider";
import { Search } from "./Search/Search";
import s from "./topbar.module.scss";
import Updates from "./Updates";

const mobileStyle: JSX.CSSProperties = {
  position: "fixed",
  top: inPx(44),
  left: "1rem",
  width: "calc(100vw - 2rem)",
};

const Topbar: Component = () => {
  const { themeClass } = useTheme();
  return (
    <Flex
      class={combineClasses(s.Topbar, "shadow", themeClass())}
      justifyContent="space-between"
      alignItems="center"
      style={{ height: inPx(TOPBAR_HEIGHT) }}
    >
      <LeftTopbar />
      <RightSide />
    </Flex>
  );
};

const LeftTopbar: Component = () => {
  const { sidebar, selected, ws } = useAppState();
  return (
    <Flex alignItems="center" style={{ padding: "0rem 0.5rem" }}>
      {/* <Button class="grey" onClick={sidebar.toggle}>
        <Icon type="menu" width="1.15rem" />
      </Button> */}
      <Button class="grey" onClick={() => selected.set("", "home")}>
        <Icon type="home" width="1.15rem" />
      </Button>
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
  );
} 

const RightSide: Component = () => {
  const { isMobile } = useAppDimensions();
  const { username } = useUser();
  const [menu, setMenu] = createSignal<"updates" | "account">();
  const close = () => setMenu(undefined);
  return (
    <Flex gap="0.5rem" alignItems="center" style={{ padding: "0rem 0.5rem" }}>
      <SearchProvider>
        <Search />
      </SearchProvider>
      <Menu
        show={menu() === "updates"}
        close={close}
        menuStyle={isMobile() ? mobileStyle : undefined}
        target={
          <Button
            class="grey"
            onClick={() =>
              menu() === "updates" ? setMenu(undefined) : setMenu("updates")
            }
          >
            <Icon type="notifications" alt="updates" width="1.15rem" />
          </Button>
        }
        content={<Updates />}
        position="bottom right"
        backgroundColor={isMobile() ? "rgba(0,0,0,0.6)" : undefined}
      />
      <Menu
        show={menu() === "account"}
        close={close}
        target={
          <Button
            class="grey"
            onClick={() =>
              menu() === "account" ? setMenu(undefined) : setMenu("account")
            }
          >
            <Show when={!isMobile()}>{username()}</Show>
            <Icon type={!isMobile() ? "chevron-down" : "user"} />
          </Button>
        }
        content={<Account close={close} />}
        position="bottom right"
      />
    </Flex>
  );
}

export default Topbar;
