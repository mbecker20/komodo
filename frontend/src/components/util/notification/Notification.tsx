import { Component, createSignal } from "solid-js";
import { inPx } from "../../../util/helpers";
import Icon from "../icons/Icon";
import {
  Notification,
  NotificationType,
  NOTIF_HEIGHT,
  NOTIF_TIMEOUT,
} from "./Notifications";
import s from "./Notifications.module.css";

const getColor = (type: NotificationType) => {
  switch (type) {
    case "good":
      return "#23804D";
    case "bad":
      return "#A94C4F";
    case "ok":
      return "#026B79";
  }
};

const Notif: Component<{
  notif: Notification;
  i: number;
  onClose: () => void;
}> = (p) => {
  const [show, set] = createSignal(false);
  setTimeout(() => set(true), 100);
  setTimeout(() => set(false), NOTIF_TIMEOUT - 1000);

  return (
    <div
      class={s.Notification}
      style={{
        top: inPx(NOTIF_HEIGHT * 1.75 * p.i),
        height: inPx(NOTIF_HEIGHT),
        opacity: show() ? 1 : 0,
        "background-color": getColor(p.notif.type),
      }}
    >
      <div>{p.notif.message}</div>
      <button style={{ "background-color": "transparent" }} onClick={p.onClose}>
        <Icon type="cross" alt="close" width="1rem" />
      </button>
    </div>
  );
};

export default Notif;
