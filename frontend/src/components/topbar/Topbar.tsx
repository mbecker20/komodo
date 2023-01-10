import { A, useNavigate } from "@solidjs/router";
import { Component, createSignal, JSX, Show } from "solid-js";
import { TOPBAR_HEIGHT } from "../..";
import { useAppDimensions } from "../../state/DimensionProvider";
import { useAppState } from "../../state/StateProvider";
import { useUser } from "../../state/UserProvider";
import { combineClasses, inPx } from "../../util/helpers";
import Circle from "../shared/Circle";
import Icon from "../shared/Icon";
import Flex from "../shared/layout/Flex";
import Grid from "../shared/layout/Grid";
import HoverMenu from "../shared/menu/HoverMenu";
import Menu from "../shared/menu/Menu";
import Account from "./Account";
import { SearchProvider } from "./Search/Provider";
import { Search } from "./Search/Search";
import s from "./topbar.module.scss";

const mobileStyle: JSX.CSSProperties = {
  position: "fixed",
  top: inPx(44),
  left: "1rem",
  width: "calc(100vw - 2rem)",
};

const Topbar: Component = () => {
  return (
    <Grid
      class={combineClasses(s.Topbar, "shadow")}
      placeItems="center"
      style={{ height: inPx(TOPBAR_HEIGHT) }}
    >
      <LeftSide />
      <SearchProvider>
        <Search />
      </SearchProvider>
      <RightSide />
    </Grid>
  );
};

const LeftSide: Component = () => {
  const { ws } = useAppState();
  return (
    <Flex
      alignItems="center"
      style={{ padding: "0rem 0.5rem", "place-self": "center start" }}
    >
      <A href="/" class="grey">
        <Icon type="home" width="1.15rem" />
      </A>
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
};

const RightSide: Component = () => {
  const { isMobile } = useAppDimensions();
  const { username } = useUser();
  const [menu, setMenu] = createSignal<"updates" | "account">();
  const close = () => setMenu(undefined);
  return (
    <Flex
      gap="0.5rem"
      alignItems="center"
      style={{ padding: "0rem 0.5rem", "place-self": "center end" }}
    >
      <Menu
        show={menu() === "account"}
        close={close}
        containerStyle={{ cursor: "pointer" }}
        target={
          <button
            class="grey"
            onClick={() =>
              menu() === "account" ? setMenu(undefined) : setMenu("account")
            }
          >
            <Show when={!isMobile()}>{username()}</Show>
            <Icon style={{cursor: "pointer"}} type={!isMobile() ? "chevron-down" : "user"} />
          </button>
        }
        
        content={<Account close={close} />}
        position="bottom right"
      />
    </Flex>
  );
};

export default Topbar;
