import { Header } from "@components/header";
import { useUser } from "@hooks";
import { Toaster } from "@ui/toast";
import { useLocation } from "react-router-dom";
import { Outlet, useNavigate } from "react-router-dom";

export const Layout = () => {
  const { data, isError } = useUser();
  const navigate = useNavigate();
  const path = useLocation().pathname;
  // if (isError && !path.includes("login")) navigate("/login");

  console.log(data);

  return (
    <>
      <div className="relative flex min-h-screen flex-col">
        <Header />
        <div className="container px-2 md:px-8 grid gap-6 pb-8 pt-6 md:py-10">
          <Outlet />
        </div>
      </div>
      <Toaster />
    </>
  );
};
