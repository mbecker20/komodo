/* @refresh reload */
import { render } from "solid-js/web";

import "./index.css";
import App from "./App";
import Client from "./util/client";
import makeNotifications from "./components/util/notification/Notifications";
import { UserProvider } from "./state/UserProvider";
import { DimensionProvider } from "./state/DimensionProvider";
import LoginGuard from "./components/login/LoginGuard";
import { AppStateProvider } from "./state/StateProvider";

export const URL = "http://localhost:9000";
export const WS_URL = "ws://localhost:9000/ws";
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
