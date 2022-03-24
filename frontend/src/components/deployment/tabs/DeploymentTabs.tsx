import { Deployment } from "@monitor/types";
import { Component } from "solid-js";
import Tabs from "../../util/tabs/Tabs";
import Config from "./config/Config";
import ErrorLog from "./ErrorLog";
import Log from "./Log";

const DeploymentTabs: Component<{ deployment: Deployment }> = (p) => {
  return (
    <Tabs
      tabs={[
        {
          title: "log",
          element: <Log />,
        },
        {
          title: "error log",
          element: <ErrorLog />,
        },
        {
          title: "config",
          element: <Config deployment={p.deployment} />,
        },
      ]}
      localStorageKey="deployment-tab"
    />
  );
};

export default DeploymentTabs;
