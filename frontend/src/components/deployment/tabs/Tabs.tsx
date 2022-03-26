import { Deployment } from "@monitor/types";
import { Component } from "solid-js";
import Tabs from "../../util/tabs/Tabs";
import Config from "./config/Config";
import ErrorLog from "./ErrorLog";
import Log from "./Log";
import s from "../deployment.module.css";
import { combineClasses } from "../../../util/helpers";
import { ConfigProvider } from "./config/Provider";

const DeploymentTabs: Component<{ deployment: Deployment }> = (p) => {
  return (
    <Tabs
      containerClass={combineClasses(s.Tabs, "shadow")}
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
          element: (
            <ConfigProvider deployment={p.deployment}>
              <Config />
            </ConfigProvider>
          ),
        },
      ]}
      localStorageKey="deployment-tab"
    />
  );
};

export default DeploymentTabs;
