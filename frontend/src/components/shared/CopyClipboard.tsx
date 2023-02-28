import { Component } from "solid-js";
import { pushNotification } from "../..";
import { copyToClipboard } from "../../util/helpers";
import ConfirmButton from "./ConfirmButton";
import Icon from "./Icon";

const CopyClipboard: Component<{ copyText: string; copying?: string; }> = (p) => {
	return (
    <ConfirmButton
      class="blue"
      onFirstClick={() => {
        copyToClipboard(p.copyText);
        pushNotification("good", `copied ${p.copying || "text"} to clipboard`);
      }}
      confirm={<Icon type="check" />}
    >
      <Icon type="clipboard" />
    </ConfirmButton>
  );
}

export default CopyClipboard;