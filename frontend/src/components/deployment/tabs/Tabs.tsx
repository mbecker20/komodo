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
} from "../../../types";
import { client } from "../../..";
import SimpleTabs from "../../shared/tabs/SimpleTabs";

const DeploymentTabs: Component<{}> = () => {
  const { deployments, ws } = useAppState();
  const params = useParams();
  const deployment = () => deployments.get(params.id);
  const [logTail, setLogTail] = createSignal(50);
  const [log, setLog] = createSignal<LogType>();
  const status = () =>
    deployment()!.state === DockerContainerState.NotDeployed
      ? "not deployed"
      : deployment()!.container?.state;
  const loadLog = async () => {
    console.log("load log");
    if (deployment()?.state !== DockerContainerState.NotDeployed) {
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
      <SimpleTabs
        containerClass={combineClasses("card tabs shadow")}
        containerStyle={{ gap: "0.5rem" }}
        tabs={
          [
            {
              title: "config",
              element: () => <Config />,
            },
            status() !== "not deployed" && [
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
              status() !== "not deployed" && {
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
          ]
            .flat()
            .filter((e) => e) as Tab[]
        }
        localStorageKey="deployment-tab"
      />
    </Show>
  );
};

export default DeploymentTabs;
