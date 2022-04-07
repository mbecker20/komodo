import { Component } from "solid-js";
import s from "./login.module.scss";
import Grid from "../util/layout/Grid";
import Loading from "../util/loading/Loading";
import { useUser } from "../../state/UserProvider";

const NotActivated: Component<{}> = (p) => {
	const { logout } = useUser();
	return (
    <div class={s.Login}>
      <Grid placeItems="center">
        <Loading type="sonar" scale={0.7} />
        <div style={{ "font-size": "1.5rem" }}>account not activated</div>
        <button class="red" onClick={logout}>
          sign out
        </button>
      </Grid>
    </div>
  );
}

export default NotActivated;