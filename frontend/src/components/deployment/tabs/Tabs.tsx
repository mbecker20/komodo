import {
  Component,
  createEffect,
  createSignal,
  onCleanup,
  Show,
} from "solid-js";
import { Tab } from "../../shared/tabs/Tabs";
import Config from "./config/Config";
import Log from "./log/Log";
import { useAppState } from "../../../state/StateProvider";
import Icon from "../../shared/Icon";
import Flex from "../../shared/layout/Flex";
import { combineClasses } from "../../../util/helpers";
import { useParams } from "@solidjs/router";
import {
  DockerContainerState,
  Log as LogType,
  Operation,
  ServerStatus,
} from "../../../types";
import { client } from "../../..";
import SimpleTabs from "../../shared/tabs/SimpleTabs";
import { ConfigProvider } from "./config/Provider";
import Permissions from "./Permissions";
import { useUser } from "../../../state/UserProvider";

const DeploymentTabs: Component<{}> = () => {
  const { user } = useUser();
  const { deployments, ws, servers } = useAppState();
  const params = useParams();
  const deployment = () => deployments.get(params.id);
  const server = () => deployment() && servers.get(deployment()!.deployment.server_id)
  const [logTail, setLogTail] = createSignal(50);
  const [log, setLog] = createSignal<LogType>();
  const status = () =>
    deployment()!.state === DockerContainerState.NotDeployed
      ? "not deployed"
      : deployment()!.container?.state;
  const log_available = () =>
    server()?.status === ServerStatus.Ok &&
    deployment()?.state !== DockerContainerState.NotDeployed;
  const loadLog = async () => {
    if (log_available()) {
      console.log("load log");
      const log = await client.get_deployment_container_log(
        params.id,
        logTail()
      );
      setLog(log);
    } else {
      setLog();
    }
  };
  createEffect(loadLog);
  onCleanup(
    ws.subscribe(
      [
        Operation.DeployContainer,
        Operation.StartContainer,
        Operation.StopContainer,
        Operation.RemoveContainer,
      ],
      (update) => {
        if (update.target.id === params.id) {
          // console.log("updating log");
          setTimeout(() => {
            client.get_deployment_container_log(params.id).then(setLog);
          }, 2000);
        }
      }
    )
  );
  return (
    <Show when={deployment()}>
      <ConfigProvider>
        <SimpleTabs
          containerClass={combineClasses("card tabs shadow")}
          containerStyle={{ gap: "0.5rem" }}
          tabs={
            [
              {
                title: "config",
                element: () => <Config />,
              },
              log_available() && [
                {
                  title: "log",
                  element: () => (
                    <Log
                      reload={loadLog}
                      log={log()}
                      logTail={logTail()}
                      setLogTail={setLogTail}
                    />
                  ),
                },
                {
                  title: "error log",
                  titleElement: () => (
                    <Flex gap="0.5rem" alignItems="center">
                      error log{" "}
                      <Show
                        when={
                          deployment()!.state !==
                            DockerContainerState.NotDeployed && log()?.stderr
                        }
                      >
                        <Icon type="error" />
                      </Show>
                    </Flex>
                  ),
                  element: () => (
                    <Log
                      reload={loadLog}
                      log={log()}
                      logTail={logTail()}
                      setLogTail={setLogTail}
                      error
                    />
                  ),
                },
              ],
              user().admin && {
                title: "collaborators",
                element: () => <Permissions />,
              },
            ]
              .flat()
              .filter((e) => e) as Tab[]
          }
          localStorageKey="deployment-tab"
        />
      </ConfigProvider>
    </Show>
  );
};

export default DeploymentTabs;
