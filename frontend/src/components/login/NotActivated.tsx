import { Component } from "solid-js";
import s from "./login.module.scss";
import Grid from "../util/layout/Grid";
import Loading from "../util/loading/Loading";
import { useUser } from "../../state/UserProvider";
import Flex from "../util/layout/Flex";

const NotActivated: Component<{}> = (p) => {
	const { logout } = useUser();
	return (
    <div class={s.Login}>
      <Grid placeItems="center">
        <Flex alignItems="center">
          <div style={{ "font-size": "1.5rem" }}>account inactive</div>
          <Loading type="sonar" scale={0.7} />
        </Flex>

        <button class="red big-text" onClick={logout}>
          sign out
        </button>
      </Grid>
    </div>
  );
}

export default NotActivated;