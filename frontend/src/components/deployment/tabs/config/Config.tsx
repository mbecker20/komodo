import { Component, Show } from "solid-js";
import Grid from "../../../shared/layout/Grid";
import Image from "./container/Image";
import Network from "./container/Network";
import Mounts from "./container/Volumes";
import Env from "./container/Env";
import PostImage from "./container/PostImage";
import Ports from "./container/Ports";
import { useConfig } from "./Provider";
import Flex from "../../../shared/layout/Flex";
import Icon from "../../../shared/Icon";
import ConfirmButton from "../../../shared/ConfirmButton";
import Restart from "./container/Restart";
import DockerAccount from "./container/DockerAccount";
import Git from "./mount-repo/Git";
import { Tab } from "../../../shared/tabs/Tabs";
import RepoMount from "./mount-repo/RepoMount";
import { OnClone, OnPull } from "./mount-repo/OnGit";
import Loading from "../../../shared/loading/Loading";
import { useAppDimensions } from "../../../../state/DimensionProvider";
import SimpleTabs from "../../../shared/tabs/SimpleTabs";
import ExtraArgs from "./container/ExtraArgs";
import WebhookUrl from "./container/WebhookUrl";
import RedeployOnBuild from "./container/RedeployOnBuild";
import TerminationSignals, { DefaultTerminationSignal, DefaultTerminationTimeout } from "./termination/TerminationSignals";

const Config: Component<{}> = () => {
  const { deployment, reset, save, userCanUpdate } = useConfig();
  const { isMobile } = useAppDimensions();
  return (
    <Show when={deployment.loaded}>
      <Grid class="config">
        <SimpleTabs
          containerClass="config-items"
          tabsGap="0rem"
          localStorageKey="deployment-config-tab"
          tabs={
            [
              {
                title: "container",
                element: () => (
                  <Grid class="config-items scroller" placeItems="start center">
                    <Image />
                    <DockerAccount />
                    <Network />
                    <Restart />
                    <Env />
                    <Ports />
                    <Mounts />
                    <ExtraArgs />
                    <PostImage />
                    <RedeployOnBuild />
                    <Show when={isMobile()}>
                      <div style={{ height: "1rem" }} />
                    </Show>
                  </Grid>
                ),
              },
              {
                title: "termination",
                element: () => (
                  <Grid class="config-items" placeItems="start center" style={{ "margin-bottom": "" }}>
                    <TerminationSignals />
                    <DefaultTerminationSignal />
                    <DefaultTerminationTimeout />
                    <Show when={isMobile()}>
                      <div style={{ height: "1rem" }} />
                    </Show>
                  </Grid>
                ),
              },
              (userCanUpdate() || deployment.repo ? true : false) && {
                title: "frontend",
                element: () => (
                  <Grid class="config-items scroller" placeItems="start center">
                    <Git />
                    <Show when={userCanUpdate()}>
                      <WebhookUrl />
                    </Show>
                    <RepoMount />
                    <OnClone />
                    <OnPull />
                    <Show when={isMobile()}>
                      <div style={{ height: "1rem" }} />
                    </Show>
                  </Grid>
                ),
              },
            ].filter((e) => e) as Tab[]
          }
        />
        <Show when={deployment.updated}>
          <Show
            when={!deployment.updating}
            fallback={
              <button class="green">
                updating <Loading type="spinner" />
              </button>
            }
          >
            <Flex style={{ "place-self": "center", padding: "1rem" }}>
              <button onClick={reset}>
                reset
                <Icon type="reset" />
              </button>
              <ConfirmButton onConfirm={save} class="green">
                save
                <Icon type="floppy-disk" />
              </ConfirmButton>
            </Flex>
          </Show>
        </Show>
      </Grid>
    </Show>
  );
};

export default Config;
