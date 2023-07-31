import { RouterProvider, createBrowserRouter } from "react-router-dom";
import { Layout } from "@layouts/layout";
import { Login } from "@pages/auth/login";
import { Signup } from "@pages/auth/signup";
import { Dashboard } from "@pages/dashboard";
import { Server } from "@resources/server/page";
import { Build } from "@resources/build/page";
import { Deployments, Builds, Servers, Builders } from "@resources/pages";
import { DeploymentUpdates } from "@resources/deployment/updates";
import { DeploymentLayout } from "@resources/deployment/layout";
import { DeploymentPage } from "@resources/deployment/page";
import { DeploymentConfig } from "@resources/deployment/config";
import { ServerConfig } from "@resources/server/config";
import { BuildConfig } from "@resources/build/config";
import { ServerStatsPage } from "@resources/server/stats";

const router = createBrowserRouter([
  {
    path: "/",
    element: <Layout />,
    children: [
      { path: "", element: <Dashboard /> },
      { path: "login", element: <Login /> },
      { path: "signup", element: <Signup /> },

      // Deployments
      {
        path: "deployments",
        children: [
          { path: "", element: <Deployments /> },
          {
            path: ":deploymentId",
            element: <DeploymentLayout />,
            children: [
              { path: "", element: <DeploymentPage /> },
              { path: "updates", element: <DeploymentUpdates /> },
              { path: "config", element: <DeploymentConfig /> },
            ],
          },
        ],
      },

      // Servers
      {
        path: "servers",
        children: [
          { path: "", element: <Servers /> },
          {
            path: ":serverId",
            element: <Server />,
            children: [
              { path: "", element: <ServerStatsPage /> },
              { path: "config", element: <ServerConfig /> },
            ],
          },
        ],
      },

      // Builds
      {
        path: "builds",
        children: [
          { path: "", element: <Builds /> },
          {
            path: ":buildId",
            element: <Build />,
            children: [{ path: "config", element: <BuildConfig /> }],
          },
        ],
      },

      // Builders
      {
        path: "builders",
        children: [
          { path: "", element: <Builders /> },
          // { path: ":builderId", element: <Build /> },
        ],
      },
    ],
  },
]);

const Router = () => <RouterProvider router={router} />;
export default Router;
