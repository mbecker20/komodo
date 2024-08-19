import { Layout } from "@components/layouts";
import { useUser } from "@lib/hooks";
import { Login } from "@pages/login";
import { Resource } from "@pages/resource";
import { Resources } from "@pages/resources";
import { RouterProvider, createBrowserRouter } from "react-router-dom";
import { Tree } from "@pages/home/tree";
import { Updates } from "@pages/updates";
import { AllResources } from "@pages/home/all_resources";
import { UserDisabled } from "@pages/user_disabled";
import { Home } from "@pages/home";
import { ResourceStats } from "@pages/resource_stats";
import { Alerts } from "@pages/alerts";
import { UserPage } from "@pages/user";
import { UserGroupPage } from "@pages/user-group";
import { Settings } from "@pages/settings";
import { StackServicePage } from "@pages/stack-service";
import { NetworkPage } from "@pages/server-info/network";

const ROUTER = createBrowserRouter([
  {
    path: "/",
    element: <Layout />,
    children: [
      { path: "", element: <Home /> },
      { path: "settings", element: <Settings /> },
      // { path: "keys", element: <Keys /> },
      // { path: "tags", element: <Tags /> },
      { path: "tree", element: <Tree /> },
      { path: "alerts", element: <Alerts /> },
      { path: "updates", element: <Updates /> },
      // { path: "variables", element: <Variables /> },
      { path: "resources", element: <AllResources /> },
      { path: "user-groups/:id", element: <UserGroupPage /> },
      {
        path: "users",
        children: [
          // { path: "", element: <UsersPage /> },
          { path: ":id", element: <UserPage /> },
        ],
      },
      {
        path: ":type",
        children: [
          { path: "", element: <Resources /> },
          { path: ":id", element: <Resource /> },
          { path: ":id/stats", element: <ResourceStats /> },
          { path: ":id/updates", element: <Updates /> },
          { path: ":id/alerts", element: <Alerts /> },
          {
            path: ":id/service/:service",
            element: <StackServicePage />,
          },
          {
            path: ":id/network/:network",
            element: <NetworkPage />,
          },
        ],
      },
    ],
  },
]);

export const Router = () => {
  const { data: user, isLoading } = useUser();

  if (isLoading && !user) return null;
  if (!user) return <Login />;
  if (!user.enabled) return <UserDisabled />;

  return <RouterProvider router={ROUTER} />;
};
