import { Component, createSignal, JSX } from "solid-js";
import { pushNotification } from "../..";
import { useToggle } from "../../util/hooks";
import ConfirmButton from "./ConfirmButton";
import Input from "./Input";
import Grid from "./layout/Grid";
import CenterMenu from "./menu/CenterMenu";

const ConfirmMenuButton: Component<{
  onConfirm?: () => void;
  class?: string;
  style?: JSX.CSSProperties;
  title: string;
  match: string;
  children: JSX.Element;
}> = (p) => {
  const [show, toggleShow] = useToggle();

  return (
    <CenterMenu
      show={show}
      toggleShow={toggleShow}
      title={p.title}
      target={
        <button
          class={p.class || "green"}
          style={p.style}
          onClick={(e) => {
            e.stopPropagation();
            toggleShow();
          }}
        >
          {p.children}
        </button>
      }
      content={() => (
        <ConfirmMenuContent
          class={p.class}
          title={p.title}
          match={p.match}
          onConfirm={p.onConfirm}
        />
      )}
      position="center"
    />
  );
};

const ConfirmMenuContent: Component<{
  class?: string;
  title: string;
  match: string;
  onConfirm?: () => void;
}> = (p) => {
  const [input, setInput] = createSignal("");
  return (
    <Grid placeItems="center">
      <Input
        placeholder={`enter '${p.match}'`}
        onEdit={setInput}
        value={input()}
        autofocus
      />
      <ConfirmButton
        class={p.class}
        onConfirm={() => {
          if (input() === p.match) {
            p.onConfirm && p.onConfirm();
          } else {
            pushNotification("bad", "must enter value to confirm");
          }
        }}
      >
        {p.title}
      </ConfirmButton>
    </Grid>
  );
};

export default ConfirmMenuButton;
