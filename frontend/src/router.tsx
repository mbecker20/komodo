import { RouterProvider, createBrowserRouter } from "react-router-dom";
import { Layout } from "@layouts/layout";
import { Login } from "@pages/auth/login";
import { Signup } from "@pages/auth/signup";
import { Dashboard } from "@pages/dashboard";
import { Deployment } from "@resources/deployment/page";
import { Server } from "@resources/server/page";
import { Build } from "@resources/build/page";
import { Deployments, Builds, Servers } from "@resources/pages";

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
          { path: ":deploymentId", element: <Deployment /> },
        ],
      },
      {
        path: "builds",
        children: [
          { path: "", element: <Builds /> },
          { path: ":buildId", element: <Build /> },
        ],
      },
      {
        path: "servers",
        children: [
          { path: "", element: <Servers /> },
          { path: ":serverId", element: <Server /> },
        ],
      },
    ],
  },
]);

const Router = () => <RouterProvider router={router} />;
export default Router;
