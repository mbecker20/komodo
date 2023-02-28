import { Component, Show } from "solid-js";
import { pushNotification, MONITOR_BASE_URL } from "../../../..";
import { copyToClipboard, getId } from "../../../../util/helpers";
import ConfirmButton from "../../../shared/ConfirmButton";
import Icon from "../../../shared/Icon";
import Flex from "../../../shared/layout/Flex";
import Grid from "../../../shared/layout/Grid";
import { useConfig } from "../Provider";

const ListenerUrl: Component<{}> = (p) => {
	const { build, userCanUpdate } = useConfig();
  const listenerUrl = () => `${MONITOR_BASE_URL}/api/listener/build/${getId(build)}`;
	return (
    <Show when={userCanUpdate()}>
      <Grid class="config-item shadow">
        <h1>webhook url</h1>
        <Flex justifyContent="space-between" alignItems="center">
          <div class="ellipsis" style={{ width: "250px" }}>
            {listenerUrl()}
          </div>
          <ConfirmButton
            class="blue"
            onFirstClick={() => {
              copyToClipboard(listenerUrl());
              pushNotification("good", "copied url to clipboard");
            }}
            confirm={<Icon type="check" />}
          >
            <Icon type="clipboard" />
          </ConfirmButton>
        </Flex>
      </Grid>
    </Show>
  );
}

export default ListenerUrl;