import { Component } from "solid-js";
import Tabs from "../../util/tabs/Tabs";
import Config from "./Config";
import ErrorLog from "./ErrorLog";
import Log from "./Log";

const DeploymentTabs: Component<{}> = (p) => {
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
          element: <Config />,
        },
      ]}
      localStorageKey="deployment-tab"
    />
  );
};

export default DeploymentTabs;
