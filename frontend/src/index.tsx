/* @refresh reload */
import { render } from "solid-js/web";

import "./index.css";
import App from "./components/app/App";
import Client from "./util/client";
import makeNotifications from "./components/util/notification/Notifications";
import { UserProvider } from "./state/UserProvider";
import { WidthProvider } from "./state/WidthProvider";

export const URL = "http://localhost:9000";
export const WS_URL = "ws://localhost:9000/ws"
export const client = new Client(URL);

export const { Notifications, pushNotification } = makeNotifications();

render(
  () => [
    <WidthProvider>
      <UserProvider>
        <App />
      </UserProvider>
    </WidthProvider>,
    <Notifications />,
  ],
  document.getElementById("root") as HTMLElement
);
