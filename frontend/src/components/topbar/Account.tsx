import { Component, Show } from "solid-js";
import { useAppDimensions } from "../../state/DimensionProvider";
import { useAppState } from "../../state/StateProvider";
import { useTheme } from "../../state/ThemeProvider";
import { useUser } from "../../state/UserProvider";
import { readablePermissions } from "../../util/helpers";
import Button from "../util/Button";
import Flex from "../util/layout/Flex";
import Grid from "../util/layout/Grid";
import s from "./topbar.module.scss";

const Account: Component<{ close: () => void }> = (p) => {
  const { logout, selected } = useAppState();
  const { username, permissions } = useUser();
  const { isDark, toggleDarkTheme } = useTheme();
  const { isMobile } = useAppDimensions();
  return (
    <Grid gap="0.5rem" class={s.Account} placeItems="center end">
      <Show when={isMobile()}>
        <Flex justifyContent="center">{username()}</Flex>
      </Show>
      <Flex justifyContent="center">
        permissions: {readablePermissions(permissions())}
      </Flex>
      <Show when={permissions() > 1}>
        <Button
          class="grey"
          onClick={() => {
            selected.set("", "users");
            p.close();
          }}
          style={{ "font-size": "1rem", width: "100%" }}
        >
          manage users
        </Button>
      </Show>
      <Button
        class="grey"
        onClick={toggleDarkTheme}
        style={{ "font-size": "1rem", width: "100%" }}
      >
        {isDark() ? "dark" : "light"} theme
      </Button>
      <Button onClick={logout} class="red" style={{ width: "100%" }}>
        log out
      </Button>
    </Grid>
  );
};

export default Account;
