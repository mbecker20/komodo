import { useParams } from "@solidjs/router";
import { Component, createResource, For, Show } from "solid-js";
import { client } from "../../..";
import { useAppDimensions } from "../../../state/DimensionProvider";
import { useAppState } from "../../../state/StateProvider";
import { readableStorageAmount } from "../../../util/helpers";
import Flex from "../../shared/layout/Flex";
import Grid from "../../shared/layout/Grid";
import Loading from "../../shared/loading/Loading";

const Info: Component<{}> = (p) => {
  const { isMobile } = useAppDimensions();
  const { serverInfo } = useAppState();
  const params = useParams();
  const [stats] = createResource(() => client.get_server_stats(params.id, { disks: true }));
  const info = () => serverInfo.get(params.id)!;
  return (
    <Grid class="config">
      <Grid class="config-items">
        <Show when={info()} fallback={<Loading type="three-dot" />}>
          <Show when={info().host_name}>
            <Flex
              class="config-item shadow"
              alignItems="center"
              justifyContent="space-between"
            >
              <h1>hostname</h1> <h2>{info().host_name}</h2>
            </Flex>
          </Show>
          <Show when={info().os}>
            <Flex
              class="config-item shadow"
              alignItems="center"
              justifyContent="space-between"
            >
              <h1>os</h1> <h2>{info().os}</h2>
            </Flex>
          </Show>
          <Show when={info().kernel}>
            <Flex
              class="config-item shadow"
              alignItems="center"
              justifyContent="space-between"
            >
              <h1>kernel</h1> <h2>{info().kernel}</h2>
            </Flex>
          </Show>
          <Flex
            class="config-item shadow"
            alignItems="center"
            justifyContent="space-between"
          >
            <h1>cpu</h1>{" "}
            <h2>
              {info().cpu_brand}
              {info().core_count
                ? `, ${info().core_count} core${
                    info().core_count! > 1 ? "s" : ""
                  }`
                : ""}
            </h2>
          </Flex>
        </Show>
        <Show when={stats()} fallback={<Loading type="three-dot" />}>
          <Show when={stats()!.disk.disks}>
            <Grid class="config-item shadow" gap="0.5rem">
              <h1>disks</h1>
              <For each={stats()!.disk.disks}>
                {(disk) => (
                  <Flex
                    class="grey-no-hover"
                    style={{
                      padding: "0.5rem",
                    }}
                    alignItems="center"
                    justifyContent="space-between"
                  >
                    <Flex alignItems="center">
                      <div style={{ "white-space": "nowrap" }}>
                        mount point:
                      </div>
                      <h2
                        class="ellipsis"
                        style={{
                          "max-width": isMobile() ? "50px" : "200px",
                        }}
                      >
                        {disk.mount}
                      </h2>
                    </Flex>
                    <Flex alignItems="center">
                      <div>{readableStorageAmount(disk.used_gb)} used</div>
                      <div>{readableStorageAmount(disk.total_gb)} total</div>
                      <div>
                        {((100 * disk.used_gb) / disk.total_gb).toFixed()}% full
                      </div>
                    </Flex>
                  </Flex>
                )}
              </For>
            </Grid>
          </Show>
        </Show>
      </Grid>
    </Grid>
  );
};

export default Info;
