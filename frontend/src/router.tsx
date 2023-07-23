// import { Build } from "@pages/resource/build";
// import { Server } from "@pages/resource/server";
import { RouterProvider, createBrowserRouter } from "react-router-dom";
// import { Deployment } from "@pages/resource/deployment";
// import { Builds, Dashboard, Deployments, Servers } from "@pages/resources";
import { Layout } from "@layouts/layout";
import { Login } from "@pages/auth/login";
import { Signup } from "@pages/auth/signup";
import { Dashboard } from "@pages/dashboard";
import { Server } from "@pages/server";

const router = createBrowserRouter([
  {
    path: "/",
    element: <Layout />,
    children: [
      { path: "", element: <Dashboard /> },
      { path: "login", element: <Login /> },
      { path: "signup", element: <Signup /> },

      // {
      //   path: "deployments",
      //   children: [
      //     { path: "", element: <Deployments /> },
      //     { path: ":deploymentId", element: <Deployment /> },
      //   ],
      // },
      // {
      //   path: "builds",
      //   children: [
      //     { path: "", element: <Builds /> },
      //     { path: ":buildId", element: <Build /> },
      //   ],
      // },
      {
        path: "servers",
        children: [
          { path: "", element: "servers" },
          { path: ":serverId", element: <Server /> },
        ],
      },
    ],
  },
]);

const Router = () => <RouterProvider router={router} />;
export default Router;
