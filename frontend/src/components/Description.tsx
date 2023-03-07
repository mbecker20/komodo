import { Component, createSignal, onMount, Show } from "solid-js";
import { client, pushNotification } from "..";
import { useAppState } from "../state/StateProvider";
import { UpdateTarget } from "../types";
import { useToggle } from "../util/hooks";
import Grid from "./shared/layout/Grid";
import Loading from "./shared/loading/Loading";
import CenterMenu from "./shared/menu/CenterMenu";
import TextArea from "./shared/TextArea";

const Description: Component<{
  name: string;
  target: UpdateTarget;
  description?: string;
  userCanUpdate: boolean;
}> = (p) => {
  const [show, toggleShow] = useToggle();
  const description = () => {
    if (p.description) {
      let [description] = p.description.split("\n");
      return description;
    } else {
      return "add a description";
    }
  };
  const [width, setWidth] = createSignal<number>();
  onMount(() => {
    setWidth(ref!?.clientWidth);
  });
  let ref: HTMLDivElement;
  return (
    <CenterMenu
      show={show}
      toggleShow={toggleShow}
      title={`description | ${p.name}`}
      targetClass="card grey"
      targetStyle={{ width: "100%", "justify-content": "flex-start" }}
      target={
        <div
          ref={ref! as any}
          class="ellipsis"
          style={{
            opacity: 0.7,
            width: width() ? `${width()}px` : "100%",
            "box-sizing": "border-box",
            "text-align": "left"
          }}
        >
          {width() ? description() : ""}
        </div>
      }
      content={() => (
        <DescriptionMenu
          target={p.target}
          description={p.description}
          userCanUpdate={p.userCanUpdate}
          toggleShow={toggleShow}
        />
      )}
    />
  );
};

const DescriptionMenu: Component<{
  target: UpdateTarget;
  description?: string;
  userCanUpdate: boolean;
  toggleShow: () => void;
}> = (p) => {
  const { builds, servers, deployments } = useAppState();
  let ref: HTMLTextAreaElement;
  onMount(() => {
    ref?.focus();
  });
  const [desc, setDesc] = createSignal(p.description);
  const [loading, setLoading] = createSignal(false);
  const update_description = () => {
    if (!p.userCanUpdate) return;
    setLoading(true);
    client
      .update_description({ target: p.target, description: desc() || "" })
      .then(() => {
        if (p.target.type === "Build") {
          builds.update({ ...builds.get(p.target.id)!, description: desc() });
        } else if (p.target.type === "Deployment") {
          const deployment = deployments.get(p.target.id)!;
          deployments.update({
            ...deployment,
            deployment: { ...deployment.deployment, description: desc() },
          });
        } else if (p.target.type === "Server") {
          const server = servers.get(p.target.id)!;
          servers.update({
            ...server,
            server: { ...server.server, description: desc() },
          });
        }
        p.toggleShow();
      })
      .catch(() => {
        pushNotification("bad", "failed to update description");
        p.toggleShow();
      });
  };
  return (
    <Grid placeItems="center">
      <TextArea
        ref={ref! as any}
        placeholder="add a description"
        value={desc()}
        onEdit={setDesc}
        style={{ width: "900px", "max-width": "90vw", height: "70vh", padding: "1rem" }}
        disabled={!p.userCanUpdate}
      />
      <Show when={p.userCanUpdate}>
        <Show when={!loading()} fallback={<Loading />}>
          <button class="green" onClick={update_description}>
            update
          </button>
        </Show>
      </Show>
    </Grid>
  );
};

export default Description;
