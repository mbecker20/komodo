import { ContainerStatus, Deployment, Log as LogType } from "@monitor/types";
import {
  Component,
  createEffect,
  createResource,
  createSignal,
  Show,
} from "solid-js";
import Tabs from "../../util/tabs/Tabs";
import Config from "./config/Config";
import Log from "./Log";
import s from "../deployment.module.css";
import { combineClasses } from "../../../util/helpers";
import { ConfigProvider } from "./config/Provider";
import { useAppState } from "../../../state/StateProvider";
import { getDeploymentLog } from "../../../util/query";
import Icon from "../../util/icons/Icon";
import Flex from "../../util/layout/Flex";

const DeploymentTabs: Component<{}> = (p) => {
  const { selected, deployments } = useAppState();
  const deployment = () => deployments.get(selected.id());
  const [log, setLog] = createSignal<LogType>({});
  createEffect(() => {
    if (deployment()?.status !== "not created") {
      getDeploymentLog(selected.id()).then(setLog);
    } else {
      setLog({});
    };
  });
  return (
    <Show when={deployment()}>
      <Tabs
        containerClass={combineClasses(s.Tabs, "shadow")}
        tabs={[
          {
            title: "config",
            element: (
              <ConfigProvider deployment={deployment()!}>
                <Config />
              </ConfigProvider>
            ),
          },
          {
            title: "log",
            element: <Log log={log()} />,
          },
          {
            title: "error log",
            titleElement: (
              <Flex gap="0.5rem" alignItems="center">
                error log{" "}
                <Show when={log().stderr}>
                  <Icon type="error" />
                </Show>
              </Flex>
            ),
            element: <Log log={log()} error />,
          },
        ].filter((val) => val)}
        localStorageKey="deployment-tab"
      />
    </Show>
  );
};

export default DeploymentTabs;
