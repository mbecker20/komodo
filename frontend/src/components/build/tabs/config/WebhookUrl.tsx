import { Component, createResource, Show } from "solid-js";
import { pushNotification, MONITOR_BASE_URL, client } from "../../../..";
import { copyToClipboard, getId } from "../../../../util/helpers";
import ConfirmButton from "../../../shared/ConfirmButton";
import Icon from "../../../shared/Icon";
import Flex from "../../../shared/layout/Flex";
import Grid from "../../../shared/layout/Grid";
import Loading from "../../../shared/loading/Loading";
import { useConfig } from "../Provider";

const ListenerUrl: Component<{}> = (p) => {
  const { build } = useConfig();
  const [github_base_url] = createResource(() =>
    client.get_github_webhook_base_url()
  );
  const listenerUrl = () => {
    if (github_base_url()) {
      return `${github_base_url()}/api/listener/build/${getId(build)}`;
    }
  };
  return (
    <Grid class="config-item shadow">
      <h1>webhook url</h1>
      <Flex
        justifyContent="space-between"
        alignItems="center"
        style={{ "flex-wrap": "wrap" }}
      >
        <Show when={listenerUrl()} fallback={<Loading type="three-dot" />}>
          <div class="ellipsis" style={{ width: "250px" }}>
            {listenerUrl()}
          </div>
        </Show>
        <ConfirmButton
          class="blue"
          onFirstClick={() => {
            copyToClipboard(listenerUrl() || "");
            pushNotification("good", "copied url to clipboard");
          }}
          confirm={<Icon type="check" />}
        >
          <Icon type="clipboard" />
        </ConfirmButton>
      </Flex>
    </Grid>
  );
};

export default ListenerUrl;
