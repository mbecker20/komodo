import { Component, For, Show, createEffect, createResource, onCleanup } from "solid-js";
import Grid from "../../shared/layout/Grid";
import { useParams } from "@solidjs/router";
import { client } from "../../..";
import Flex from "../../shared/layout/Flex";
import { readableBytes, readableImageNameTag, readableTimestamp } from "../../../util/helpers";
import { useAppState } from "../../../state/StateProvider";
import { Operation } from "../../../types";

const Images: Component<{}> = (p) => {
  const params = useParams();
	const { ws } = useAppState();
	const [images, { refetch }] = createResource(() => client.get_docker_images(params.id));
	let unsub = () => {};
  createEffect(() => {
    unsub();
    unsub = ws.subscribe([Operation.PruneImagesServer], (update) => {
      if (update.target.id === params.id) {
        refetch()
      }
    });
  });
  onCleanup(() => unsub());
  createEffect(() => console.log(images()));
	return (
    <Grid class="config">
      <Grid class="config-items">
        <Show when={images()}>
          <For each={images()}>
            {(image) => {
							const [name, tag] = readableImageNameTag(image.RepoTags, image.RepoDigests);
							return (
                <Flex
                  class="card light hover shadow"
                  alignItems="center"
                  justifyContent="space-between"
                >
                  <Flex alignItems="center">
                    <h2>{name}</h2>
										<h2 class="dimmed">{tag}</h2>
                  </Flex>
                  <Flex alignItems="center">
                    <div>{readableBytes(image.Size)}</div>
                    <div class="dimmed">
                      {readableTimestamp(image.Created * 1000)}
                    </div>
                  </Flex>
                </Flex>
              );
						}}
          </For>
        </Show>
      </Grid>
    </Grid>
  );
}

export default Images;