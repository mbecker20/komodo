import { Component, createResource, Show } from "solid-js";
import { useAppState } from "../../../../../state/StateProvider";
import { getId } from "../../../../../util/helpers";
import CopyClipboard from "../../../../shared/CopyClipboard";
import Flex from "../../../../shared/layout/Flex";
import Grid from "../../../../shared/layout/Grid";
import Loading from "../../../../shared/loading/Loading";
import { useConfig } from "../Provider";

const WebhookUrl: Component<{}> = (p) => {
  const { github_webhook_base_url } = useAppState();
  const { deployment } = useConfig();
  const listenerUrl = () => {
    if (github_webhook_base_url()) {
      return `${github_webhook_base_url()}/api/listener/deployment/${getId(
        deployment
      )}`;
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
          <div class="ellipsis" style={{ "max-width": "250px" }}>
            {listenerUrl()}
          </div>
        </Show>
        <CopyClipboard copyText={listenerUrl() || ""} copying="url" />
      </Flex>
    </Grid>
  );
};

export default WebhookUrl;
