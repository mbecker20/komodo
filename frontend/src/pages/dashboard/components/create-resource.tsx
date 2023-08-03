import { Types } from "@monitor/client";
import { ResourceTarget } from "@monitor/client/dist/types";
import {
  DropdownMenu,
  DropdownMenuTrigger,
  DropdownMenuContent,
  DropdownMenuLabel,
  DropdownMenuSeparator,
  DropdownMenuGroup,
  DropdownMenuItem,
} from "@ui/dropdown";
import { Button } from "@ui/button";
import { NewBuild } from "@resources/build/new";
import { NewBuilder } from "@resources/builder/new";
import { NewDeployment } from "@resources/deployment/new";
import { PlusCircle, ChevronDown } from "lucide-react";
import { useState } from "react";

export const CreateResource = () => {
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
