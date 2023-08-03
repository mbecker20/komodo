import { useRead, useUser } from "@hooks";
import { BuildCard } from "@resources/build";
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
import { ChevronDown, History, PlusCircle } from "lucide-react";
import { DeploymentCard } from "@resources/deployment";
import { NewDeployment } from "@resources/deployment/new";
import { ServerCard } from "@resources/server";
import { NewBuild } from "@resources/build/new";
import { Types } from "@monitor/client";
import { NewBuilder } from "@resources/builder/new";
import { ResourceTarget } from "@monitor/client/dist/types";
import { BuilderCard } from "@resources/builder";

const NewResource = () => {
  const [open, set] = useState<Types.ResourceTarget["type"] | false>(false);

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
            {["Deployment", "Build", "Server", "Builder"].map((target) => (
              <DropdownMenuItem
                className="cursor-pointer"
                onClick={() => set(target as ResourceTarget["type"])}
                key={target}
              >
                {target}
              </DropdownMenuItem>
            ))}
          </DropdownMenuGroup>
        </DropdownMenuContent>
      </DropdownMenu>
      <NewDeployment open={open === "Deployment"} set={set} />
      <NewBuild open={open === "Build"} set={set} />
      <NewBuilder open={open === "Builder"} set={set} />
    </>
  );
};

export const RecentlyViewed = () => {
  const user = useUser().data;
  const recents = useRead("GetUser", {}).data?.recently_viewed;

  return (
    <div className="w-full flex flex-col gap-12">
      <div className="flex justify-between">
        <div>
          <h1 className="text-4xl"> Hello, {user?.username}.</h1>
          {!!recents?.length && (
            <div className="flex items-center gap-2 text-muted-foreground">
              <History className="w-4 h-4" />
              <h2 className="text-xl">Recently Viewed</h2>
            </div>
          )}
        </div>
        <NewResource />
      </div>
      <div className="grid md:grid-cols-2 lg:grid-cols-3 gap-4">
        {recents?.map(({ type, id }) => {
          if (type === "Deployment") return <DeploymentCard key={id} id={id} />;
          if (type === "Build") return <BuildCard key={id} id={id} />;
          if (type === "Server") return <ServerCard key={id} id={id} />;
          if (type === "Builder") return <BuilderCard key={id} id={id} />;
        })}
      </div>
    </div>
  );
};
