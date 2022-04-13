/* @refresh reload */
import { render } from "solid-js/web";

import "./style/colors.scss";
import "./style/index.scss";
import "./style/app.scss";
import App from "./App";
import Client from "./util/client";
import makeNotifications from "./components/util/notification/Notifications";
import { UserProvider } from "./state/UserProvider";
import { DimensionProvider } from "./state/DimensionProvider";
import LoginGuard from "./components/login/LoginGuard";
import { AppStateProvider } from "./state/StateProvider";

export const URL =
  import.meta.env.MODE === "production"
    ? location.origin
    : "https://monitor.mogh.tech";
export const WS_URL = URL.replace("https", "wss").replace("http", "ws") + "/ws";

export const client = new Client(URL);

export const { Notifications, pushNotification } = makeNotifications();

render(
  () => [
    <DimensionProvider>
      <UserProvider>
        <LoginGuard>
          <AppStateProvider>
            <App />
          </AppStateProvider>
        </LoginGuard>
      </UserProvider>
    </DimensionProvider>,
    <Notifications />,
  ],
  document.getElementById("root") as HTMLElement
);
