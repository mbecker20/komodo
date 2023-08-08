import { ThemeToggle } from "@components/util";

import { Button } from "@ui/button";
import { ChevronRight, LogOut } from "lucide-react";
import { Link, useLocation, useParams } from "react-router-dom";

import { useUser } from "@hooks";
import { ServerName } from "@resources/server/util";
import { DeploymentName } from "@resources/deployment/util";
import { DesktopUpdates } from "@components/updates/desktop";
import { BuildName } from "@resources/build/util";
import { Omnibar } from "./omnibar";
import { WsStatusIndicator } from "@util/socket";

export const Paths = () => {
  const path = useLocation().pathname.split("/")[1];
  const { serverId, deploymentId, buildId } = useParams();

  return (
    <div className="hidden md:flex items-center gap-2">
      {path && (
        <>
          <ChevronRight className="w-4 h-4" />
          <Link to={`/${path}`} className="capitalize">
            {path}
          </Link>
        </>
      )}
      {serverId && (
        <>
          <ChevronRight className="w-4 h-4" />
          <Link to={`/servers/${serverId}`}>
            <ServerName serverId={serverId} />
          </Link>
        </>
      )}
      {deploymentId && (
        <>
          <ChevronRight className="w-4 h-4" />
          <Link to={`/deployments/${deploymentId}`}>
            <DeploymentName deploymentId={deploymentId} />
          </Link>
        </>
      )}
      {buildId && (
        <>
          <ChevronRight className="w-4 h-4" />
          <Link to={`/deployments/${buildId}`}>
            <BuildName id={buildId} />
          </Link>
        </>
      )}
    </div>
  );
};

export const Header = () => {
  const user = useUser().data;

  const logout = () => {
    localStorage.removeItem("monitor-auth-token");
    window.location.reload();
  };

  return (
    <header className="sticky top-0 z-40 w-full border-b bg-background">
      <div className="container flex h-16 items-center justify-between">
        <div className="flex gap-4">
          <Link to="/" className="font-bold text-xl cursor-pointer">
            Monitor
          </Link>
          <Paths />
        </div>
        <div className="flex">
          {user && (
            <>
              <Omnibar />
              <WsStatusIndicator />
              <DesktopUpdates />
            </>
          )}
          <ThemeToggle />
          {user && (
            <Button variant="ghost" onClick={logout}>
              <LogOut className="w-4 h-4" />
            </Button>
          )}
        </div>
      </div>
    </header>
  );
};
