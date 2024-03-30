import { Layout } from "@components/layouts";
import { useUser } from "@lib/hooks";
import { Dashboard } from "@pages/dashboard";
import { Login2 } from "@pages/login";
import { Resource } from "@pages/resource";
import { Resources } from "@pages/resources";
import { Keys } from "@pages/keys";
import { RouterProvider, createBrowserRouter } from "react-router-dom";
import { Tree } from "@pages/tree";
import { Tags } from "@pages/tags";
import { ResourceUpdates } from "@pages/resource_update";
import { UsersPage } from "@pages/users";
import { AllResources } from "@pages/all_resources";
import { UserDisabled } from "@pages/user_disabled";

const router = createBrowserRouter([
  {
    path: "/",
    element: <Layout />,
    children: [
      { path: "", element: <Dashboard /> },
      { path: "users", element: <UsersPage /> },
      { path: "keys", element: <Keys /> },
      { path: "tags", element: <Tags /> },
      { path: "tree", element: <Tree /> },
      { path: "resources", element: <AllResources /> },
      {
        path: ":type",
        children: [
          { path: "", element: <Resources /> },
          { path: ":id", element: <Resource /> },
          { path: ":id/updates", element: <ResourceUpdates /> },
        ],
      },
    ],
  },
]);

export const Router = () => {
  const { data: user, isLoading } = useUser();

  if (isLoading) return null;
  if (!user) return <Login2 />;
  if (!user.enabled) return <UserDisabled />;

  return <RouterProvider router={router} />;
};
