import { RouterProvider, createBrowserRouter } from "react-router-dom";
import { Layout } from "@layouts/layout";
import { Login } from "@pages/auth/login";
import { Signup } from "@pages/auth/signup";
import { Dashboard } from "@pages/dashboard";
import { Deployments, Builds, Servers, Builders } from "@resources/pages";

import { ServerPage } from "@resources/server";
import { DeploymentPage } from "@resources/deployment";
import { BuildPage } from "@resources/build";
import { BuilderPage } from "@resources/builder/page";

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
          { path: ":buildId", element: <BuildPage /> },
        ],
      },

      // Builders
      {
        path: "builders",
        children: [
          { path: "", element: <Builders /> },
          { path: ":builderId", element: <BuilderPage /> },
        ],
      },
    ],
  },
]);

const Router = () => <RouterProvider router={router} />;
export default Router;
