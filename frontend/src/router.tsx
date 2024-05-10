import { Layout } from "@components/layouts";
import { useUser } from "@lib/hooks";
import { Login } from "@pages/login";
import { Resource } from "@pages/resource";
import { Resources } from "@pages/resources";
import { Keys } from "@pages/keys";
import { RouterProvider, createBrowserRouter } from "react-router-dom";
import { Tree } from "@pages/home/tree";
import { Tags } from "@pages/tags";
import { Updates } from "@pages/updates";
import { UsersPage } from "@pages/users";
import { AllResources } from "@pages/home/all_resources";
import { UserDisabled } from "@pages/user_disabled";
import { Home } from "@pages/home";
import { ResourceStats } from "@pages/resource_stats";
import { Alerts } from "@pages/alerts";
import { UserPage } from "@pages/user";
import { UserGroupPage } from "@pages/user-group";

const ROUTER = createBrowserRouter([
  {
    path: "/",
    element: <Layout />,
    children: [
      { path: "", element: <Home /> },
      { path: "keys", element: <Keys /> },
      { path: "tags", element: <Tags /> },
      { path: "tree", element: <Tree /> },
      { path: "resources", element: <AllResources /> },
      { path: "alerts", element: <Alerts /> },
      { path: "updates", element: <Updates /> },
      {
        path: "users",
        children: [
          { path: "", element: <UsersPage /> },
          { path: ":id", element: <UserPage /> },
        ],
      },
      { path: "user-groups/:id", element: <UserGroupPage /> },
      {
        path: ":type",
        children: [
          { path: "", element: <Resources /> },
          { path: ":id", element: <Resource /> },
          { path: ":id/stats", element: <ResourceStats /> },
          { path: ":id/updates", element: <Updates /> },
          { path: ":id/alerts", element: <Alerts /> },
        ],
      },
    ],
  },
]);

export const Router = () => {
  const { data: user, isLoading } = useUser();

  if (isLoading) return null;
  if (!user) return <Login />;
  if (!user.enabled) return <UserDisabled />;

  return <RouterProvider router={ROUTER} />;
};
