import { useUser } from "@hooks";
import { Login } from "@pages/auth/login";
import { Toaster } from "@ui/toast";
import { Header } from "@components/header";
import { WebsocketProvider } from "@util/socket";
import { Outlet } from "react-router-dom";

export const Layout = () => {
  const { isLoading, isError } = useUser();
  if (isLoading) return null;
  if (isError) return <Login />;

  return (
    <WebsocketProvider>
      <div className="relative flex min-h-screen flex-col">
        <Header />
        <div className="container pt-12 pb-16">
          <Outlet />
        </div>
      </div>
      <Toaster />
    </WebsocketProvider>
  );
};
