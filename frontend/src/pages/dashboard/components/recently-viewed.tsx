import { useGetRecentlyViewed, useUser } from "@hooks";
import { BuildCard } from "@resources/build/card";
import { useState } from "react";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuGroup,
  DropdownMenuItem,
  DropdownMenuLabel,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from "@ui/dropdown";
import { Button } from "@ui/button";
import { ChevronDown, Hand, History, PlusCircle } from "lucide-react";
import { DeploymentCard } from "@resources/deployment/card";
import { NewDeployment } from "@resources/deployment/new";
import { ServerCard } from "@resources/server/card";
import { NewBuild } from "@resources/build/new";

const NewButton = () => {
  const [open, set] = useState<"deployment" | "build" | "server" | boolean>(
    false
  );
  return (
    <>
      <DropdownMenu>
        <DropdownMenuTrigger asChild>
          <Button
            className="w-[200px] flex items-center justify-between"
            variant="outline"
          >
            <div className="flex items-center gap-2">
              <PlusCircle className="w-4 h-4 text-green-500" />
              Create Resource
            </div>
            <ChevronDown className="w-4 h-4" />
          </Button>
        </DropdownMenuTrigger>
        <DropdownMenuContent className="w-[200px]">
          <DropdownMenuLabel className="text-xs">
            Resource Type
          </DropdownMenuLabel>
          <DropdownMenuSeparator />
          <DropdownMenuGroup>
            <DropdownMenuItem
              className="cursor-pointer"
              onClick={() => set("deployment")}
            >
              Deployment
            </DropdownMenuItem>
            <DropdownMenuItem
              className="cursor-pointer"
              onClick={() => set("build")}
            >
              Build
            </DropdownMenuItem>
            <DropdownMenuItem className="cursor-pointer">
              Server
            </DropdownMenuItem>
          </DropdownMenuGroup>
        </DropdownMenuContent>
      </DropdownMenu>
      <NewDeployment open={open === "deployment"} set={set} />
      <NewBuild open={open === "build"} set={set} />
    </>
  );
};

export const RecentlyViewed = () => {
  const user = useUser().data;

  const recents = useGetRecentlyViewed();
  return (
    <div className="w-full flex flex-col gap-6">
      <div className="flex justify-between">
        <div>
          <h1 className="text-4xl"> Hello, {user?.username}.</h1>
          {!!recents.length && (
            <div className="flex items-center gap-2 text-muted-foreground">
              <History className="w-4 h-4" />
              <h2 className="text-xl ">Recently Viewed</h2>
            </div>
          )}
        </div>
        <NewButton />
      </div>
      <div className="grid md:grid-cols-2 lg:grid-cols-3 gap-4">
        {recents.map(({ type, id }) => {
          if (type === "Deployment") return <DeploymentCard key={id} id={id} />;
          if (type === "Build") return <BuildCard key={id} id={id} />;
          if (type === "Server") return <ServerCard key={id} id={id} />;
        })}
      </div>
    </div>
  );
};
