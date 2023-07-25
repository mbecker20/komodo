import { Header } from "@components/header";
import { useUser } from "@hooks";
import { Toaster } from "@ui/toast";
import { Outlet, useLocation, useNavigate } from "react-router-dom";

export const Layout = () => {
  const { isError } = useUser();
  const path = useLocation().pathname;
  const nav = useNavigate();
  if (isError && !path.includes("login")) nav("/login");

  return (
    <>
      <div className="relative flex min-h-screen flex-col">
        <Header />
        <div className="container pt-8">
          <Outlet />
        </div>
      </div>
      <Toaster />
    </>
  );
};
