import { Component, Show } from "solid-js";
import Grid from "../../../util/layout/Grid";
import Image from "./container/Image";
import Network from "./container/Network";
import Mounts from "./container/Volumes";
import Env from "./container/Env";
import Ports from "./container/Ports";
import { useConfig } from "./Provider";
import Flex from "../../../util/layout/Flex";
import Icon from "../../../util/Icon";
import ConfirmButton from "../../../util/ConfirmButton";
import Restart from "./container/Restart";
import DockerAccount from "./container/DockerAccount";
import Git from "./mount-repo/Git";
import Tabs from "../../../util/tabs/Tabs";
import RepoMount from "./mount-repo/RepoMount";
import { OnClone, OnPull } from "./mount-repo/OnGit";
import Loading from "../../../util/loading/Loading";
import Owners from "./Owners";
import { pushNotification, URL } from "../../../..";
import { combineClasses, copyToClipboard } from "../../../../util/helpers";
import { useAppDimensions } from "../../../../state/DimensionProvider";
import { useTheme } from "../../../../state/ThemeProvider";

const Config: Component<{}> = (p) => {
  const { deployment, reset, save, userCanUpdate } = useConfig();
  const { isMobile } = useAppDimensions();
  const listenerUrl = () => `${URL}/api/listener/deployment/${deployment._id}`;
  const { themeClass } = useTheme();
  return (
    <Show when={deployment.loaded}>
      <Grid class="config">
        <Tabs
          containerStyle={{
            height: "100%",
            width: isMobile() ? undefined : "500px",
          }}
          tabsGap="0rem"
          localStorageKey="deployment-config-tab"
          tabs={[
            {
              title: "container",
              element: (
                <Grid class="config-items scroller" placeItems="start center">
                  <Image />
                  <Show when={deployment.image}>
                    <DockerAccount />
                  </Show>
                  <Network />
                  <Restart />
                  <Ports />
                  <Mounts />
                  <Env />
                  <Show when={isMobile()}>
                    <div style={{ height: "1rem" }} />
                  </Show>
                </Grid>
              ),
            },
            (userCanUpdate() || deployment.repo ? true : false) && {
              title: "repo mount",
              element: (
                <Grid class="config-items scroller" placeItems="start center">
                  <Git />
                  <Show when={userCanUpdate()}>
                    <Grid
                      class={combineClasses("config-item shadow", themeClass())}
                    >
                      <h1>webhook url</h1>
                      <Flex
                        justifyContent="space-between"
                        alignItems="center"
                        style={{ "flex-wrap": "wrap" }}
                      >
                        <div class="ellipsis" style={{ width: "250px" }}>
                          {listenerUrl()}
                        </div>
                        <ConfirmButton
                          color="blue"
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
                  <RepoMount />
                  <OnClone />
                  <OnPull />
                  <Show when={isMobile()}>
                    <div style={{ height: "1rem" }} />
                  </Show>
                </Grid>
              ),
            },
            userCanUpdate() && {
              title: "collaborators",
              element: (
                <Grid
                  class="config-items scroller"
                  style={{ height: "100%" }}
                  placeItems="start center"
                >
                  <Owners />
                </Grid>
              ),
            },
          ]}
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
              <ConfirmButton onConfirm={save} color="green">
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
