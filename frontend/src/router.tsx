import { Layout } from "@components/layouts";
import { useRead } from "@lib/hooks";
import { Dashboard } from "@pages/dashboard";
import { Login } from "@pages/login";
import { Resource } from "@pages/resource";
import { Resources } from "@pages/resources";
import { Keys }  from "@pages/keys";
import { RouterProvider, createBrowserRouter } from "react-router-dom";

const router = createBrowserRouter([
  {
    path: "/",
    element: <Layout />,
    children: [
      { path: "", element: <Dashboard /> },
      { path: "keys", element: <Keys /> },
      {
        path: ":type",
        children: [
          { path: "", element: <Resources /> },
          { path: ":id", element: <Resource /> },
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
