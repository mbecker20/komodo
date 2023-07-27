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
              { path: "config", element: <>deployment config!</> },
            ],
          },
        ],
      },

      // Servers
      {
        path: "servers",
        children: [
          { path: "", element: <Servers /> },
          { path: ":serverId", element: <Server /> },
        ],
      },

      // Builds
      {
        path: "builds",
        children: [
          { path: "", element: <Builds /> },
          { path: ":buildId", element: <Build /> },
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
