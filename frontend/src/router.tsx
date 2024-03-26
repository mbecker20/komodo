import { Layout } from "@components/layouts";
import { useRead } from "@lib/hooks";
import { Dashboard } from "@pages/dashboard";
import { Login } from "@pages/login";
import { Resource } from "@pages/resource";
import { Resources } from "@pages/resources";
import { Keys } from "@pages/keys";
import { RouterProvider, createBrowserRouter } from "react-router-dom";
import { Tree } from "@pages/tree";
import { Tags } from "@pages/tags";
import { ResourceUpdates } from "@pages/resource_update";
import { UsersPage } from "@pages/users";

const router = createBrowserRouter([
  {
    path: "/",
    element: <Layout />,
    children: [
      { path: "", element: <Dashboard /> },
      { path: "tree", element: <Tree /> },
      { path: "keys", element: <Keys /> },
      { path: "tags", element: <Tags /> },
      { path: "users", element: <UsersPage /> },
      {
        path: ":type",
        children: [
          { path: "", element: <Resources /> },
          { path: ":id", element: <Resource /> },
          { path: ":id/updates", element: <ResourceUpdates /> }
        ],
      },
    ],
  },
]);

export const Router = () => {
  const { data: user, isLoading } = useRead("GetUser", {});

  if (isLoading) return null;
  if (!user) return <Login />;

  return <RouterProvider router={router} />;
};
