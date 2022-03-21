/* @refresh reload */
import { render } from "solid-js/web";

import "./index.css";
import App from "./components/App/App";
import Client from "./util/client";
import makeNotifications from "./components/util/notification/Notifications";
import { UserProvider } from "./state/UserProvider";

export const URL = "http://localhost:9000";
export const WS_URL = "ws://localhost:9000/ws"
export const client = new Client(URL);

export const { Notifications, pushNotification } = makeNotifications();

render(
  () => [
    <UserProvider>
      <App />
    </UserProvider>,
    <Notifications />,
  ],
  document.getElementById("root") as HTMLElement
);
