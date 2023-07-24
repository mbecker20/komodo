import { Header } from "@components/header";
import { useUser } from "@hooks";
import { Toaster } from "@ui/toast";
import { Outlet, useLocation, useNavigate } from "react-router-dom";

export const Layout = () => {
  const { isError } = useUser();
  const path = useLocation().pathname;
  const nav = useNavigate();
  if (isError && !path.includes("login")) nav("/login");
  // const navigate = useNavigate();
  // const path = useLocation().pathname;
  // if (isError) return navigate("/login");
  // if ((isError && !path.includes("login")) || !path.includes("signup"))
  //   navigate("/login");

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
