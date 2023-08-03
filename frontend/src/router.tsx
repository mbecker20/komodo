import { RouterProvider, createBrowserRouter } from "react-router-dom";
import { Layout } from "@layouts/layout";
import { Login } from "@pages/auth/login";
import { Signup } from "@pages/auth/signup";
import { Dashboard } from "@pages/dashboard";
import {
  Deployments,
  Builds,
  Servers,
  Builders,
  Alerters,
  Repos,
} from "@resources/pages";

import { ServerPage } from "@resources/server";
import { DeploymentPage } from "@resources/deployment";
import { BuildPage } from "@resources/build";
import { BuilderPage } from "@resources/builder";
import { AlerterPage } from "@resources/alerter";
import { RepoPage } from "@resources/repo";

const router = createBrowserRouter([
  {
    path: "/",
    element: <Layout />,
    children: [
      { path: "", element: <Dashboard /> },
      { path: "login", element: <Login /> },
      { path: "signup", element: <Signup /> },
      {
        path: "deployments",
        children: [
          { path: "", element: <Deployments /> },
          { path: ":deploymentId", element: <DeploymentPage /> },
        ],
      },
      {
        path: "servers",
        children: [
          { path: "", element: <Servers /> },
          { path: ":serverId", element: <ServerPage /> },
        ],
      },
      {
        path: "builds",
        children: [
          { path: "", element: <Builds /> },
          { path: ":buildId", element: <BuildPage /> },
        ],
      },
      {
        path: "builders",
        children: [
          { path: "", element: <Builders /> },
          { path: ":builderId", element: <BuilderPage /> },
        ],
      },
      {
        path: "alerters",
        children: [
          { path: "", element: <Alerters /> },
          { path: ":builderId", element: <AlerterPage /> },
        ],
      },
      {
        path: "repos",
        children: [
          { path: "", element: <Repos /> },
          { path: ":builderId", element: <RepoPage /> },
        ],
      },
    ],
  },
]);

const Router = () => <RouterProvider router={router} />;
export default Router;
