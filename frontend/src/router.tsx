import { RouterProvider, createBrowserRouter } from "react-router-dom";
import { Layout } from "@layouts/layout";
import { Login } from "@pages/auth/login";
import { Signup } from "@pages/auth/signup";
import { Dashboard } from "@pages/dashboard";
import { Server, ServerContent } from "@resources/server/page";
import { Build } from "@resources/build/page";
import { Deployments, Builds, Servers, Builders } from "@resources/pages";

import { ServerConfig } from "@resources/server/config";
import { BuildConfig } from "@resources/build/config";
import { DeploymentPage } from "@resources/dep";
import { ServerPage } from "@resources/ser";

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
          { path: ":deploymentId", element: <DeploymentPage /> },
        ],
      },

      // Servers
      {
        path: "servers",
        children: [
          { path: "", element: <Servers /> },
          { path: ":serverId", element: <ServerPage /> },
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
