import { Component, createSignal, JSX, Show } from "solid-js";
import { TOPBAR_HEIGHT } from "../..";
import { useAppDimensions } from "../../state/DimensionProvider";
import { useUser } from "../../state/UserProvider";
import { combineClasses, inPx } from "../../util/helpers";
import Circle from "../shared/Circle";
import Icon from "../shared/Icon";
import Flex from "../shared/layout/Flex";
import Grid from "../shared/layout/Grid";
import HoverMenu from "../shared/menu/HoverMenu";
import Menu from "../shared/menu/Menu";
import Account from "./Account";
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
      class={combineClasses(s.GridTopbar, "shadow")}
      placeItems="center"
      style={{ height: inPx(TOPBAR_HEIGHT) }}
    >
      <LeftSide />
      {/* <SearchProvider>
        <Search />
      </SearchProvider> */}
	  <div>search</div>
      <RightSide />
    </Grid>
  );
};

const LeftSide: Component = () => {
  return (
    <Flex
      alignItems="center"
      style={{ padding: "0rem 0.5rem", "place-self": "center start" }}
    >
      <button class="grey" onClick={() => {
		// selected.set("", "home");
	  }}>
        <Icon type="home" width="1.15rem" />
      </button>
      {/* <HoverMenu
        target={
          <Circle
            size={1}
            class={ws.isOpen() ? "green" : "red"}
            style={{ transition: "all 500ms ease-in-out" }}
          />
        }
        content={ws.isOpen() ? "connected" : "disconnected"}
        position="right center"
      /> */}
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
      {/* <Menu
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
      /> */}
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
            <Show when={!isMobile()}>{username()}</Show>
            <Icon type={!isMobile() ? "chevron-down" : "user"} />
          </button>
        }
        content={<Account close={close} />}
        position="bottom right"
      />
    </Flex>
  );
};

export default Topbar;
