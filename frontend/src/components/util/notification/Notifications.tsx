import { createSignal, For, Setter } from "solid-js";
import Notif from "./Notification";
import s from "./Notifications.module.css";

export type NotificationType = "good" | "bad" | "ok";

export type Notification = {
  id: number;
  type: NotificationType;
  message: string;
};

export const NOTIF_TIMEOUT = 5000;
export const NOTIF_HEIGHT = 30;

export default function makeNotifications() {
  const state: { notifications: Notification[]; count: number } = {
    notifications: [],
    count: 0,
  };

  let setNotifs: Setter<Notification[]>;

  const pushNotification = (type: NotificationType, message: string) => {
    const notif = { type, message, id: state.count };
    state.count++;
    state.notifications = [notif, ...state.notifications];
    setNotifs(state.notifications);
    console.log(state.notifications);
    setTimeout(() => {
      state.notifications = state.notifications.slice(
        0,
        state.notifications.length - 1
      );
      setNotifs(state.notifications);
    }, NOTIF_TIMEOUT);
  };

  const onClose = (id: number) => () => {
    state.notifications = state.notifications.filter(
      (notif) => notif.id !== id
    );
    setNotifs(state.notifications);
  };

  return {
    Notifications: () => {
      const [notifs, setN] = createSignal<Notification[]>([]);
      setNotifs = setN;
      return (
        <div class={s.NotificationProvider}>
          <For each={notifs()}>
            {(notif, i) => (
              <Notif notif={notif} i={i()} onClose={onClose(notif.id)} />
            )}
          </For>
        </div>
      );
    },
    pushNotification,
  };
}
