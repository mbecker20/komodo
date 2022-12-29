/* @refresh reload */
import "./style/colors.scss";
import "./style/index.scss";
import "./style/app.scss";
import { render } from "solid-js/web";
import App from "./App";
import LoginGuard from "./components/login/LoginGuard";
import makeNotifications from "./components/shared/notification/Notifications";
import { DimensionProvider } from "./state/DimensionProvider";
import { UserProvider } from "./state/UserProvider";
import { Client } from "./util/client";
import { Router } from "@solidjs/router";
import { AppStateProvider } from "./state/StateProvider";

export const TOPBAR_HEIGHT = 50;

export const URL =
  import.meta.env.MODE === "production"
    ? location.origin
    : (import.meta.env.VITE_MONITOR_HOST as string) || "http://localhost:9000";

export const WS_URL = URL.replace("http", "ws") + "/ws/update";

const token =
  (import.meta.env.VITE_ACCESS_TOKEN as string) ||
  localStorage.getItem("access_token") ||
  null;

export const client = new Client(URL, token);

export const { Notifications, pushNotification } = makeNotifications();

client.initialize().then(() => {
  render(
    () => [
      <DimensionProvider>
        <UserProvider>
          <LoginGuard>
            <Router>
              <AppStateProvider>
                <App />
              </AppStateProvider>
            </Router>
          </LoginGuard>
        </UserProvider>
      </DimensionProvider>,
      <Notifications />,
    ],
    document.getElementById("root") as HTMLElement
  );
});
