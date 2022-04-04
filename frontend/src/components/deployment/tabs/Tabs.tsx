import { Log as LogType, Update } from "@monitor/types";
import {
  Component,
  createEffect,
  createSignal,
  onCleanup,
  Show,
} from "solid-js";
import Tabs from "../../util/tabs/Tabs";
import Config from "./config/Config";
import Log from "./log/Log";
import { ConfigProvider } from "./config/Provider";
import { useAppState } from "../../../state/StateProvider";
import { getDeploymentLog } from "../../../util/query";
import Icon from "../../util/icons/Icon";
import Flex from "../../util/layout/Flex";
import {
  ADD_UPDATE,
  DELETE_CONTAINER,
  DEPLOY,
  START_CONTAINER,
  STOP_CONTAINER,
} from "../../../state/actions";

const DeploymentTabs: Component<{}> = (p) => {
  const { selected, deployments, ws } = useAppState();
  const deployment = () => deployments.get(selected.id());
  const [log, setLog] = createSignal<LogType>({});
  const load = async () => {
    console.log("load log")
    if (deployment()?.status !== "not deployed") {
      const log = await getDeploymentLog(selected.id());
      setLog(log);
    } else {
      setLog({});
    }
  };
  createEffect(load);
  const unsub = ws.subscribe([ADD_UPDATE], ({ update }: { update: Update }) => {
    if (
      update.deploymentID === selected.id() &&
      (update.operation === DEPLOY ||
        update.operation === START_CONTAINER ||
        update.operation === STOP_CONTAINER ||
        update.operation === DELETE_CONTAINER)
    ) {
      // console.log("updating log");
      setTimeout(() => {
        getDeploymentLog(selected.id()).then(setLog);
      }, 2000);
    }
  });
  onCleanup(unsub);
  return (
    <Show when={deployment()}>
      <ConfigProvider deployment={deployment()!}>
        <Tabs
          containerClass="card tabs shadow"
          containerStyle={{ gap: "0.5rem" }}
          tabs={[
            {
              title: "config",
              element: <Config />,
            },
            {
              title: "log",
              element: <Log reload={load} log={log()} />,
            },
            {
              title: "error log",
              titleElement: (
                <Flex gap="0.5rem" alignItems="center">
                  error log{" "}
                  <Show
                    when={
                      deployment()!.status !== "not deployed" && log().stderr
                    }
                  >
                    <Icon type="error" />
                  </Show>
                </Flex>
              ),
              element: <Log reload={load} log={log()} error />,
            },
          ].filter((val) => val)}
          localStorageKey="deployment-tab"
        />
      </ConfigProvider>
    </Show>
  );
};

export default DeploymentTabs;
