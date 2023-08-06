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
import { PlusCircle, ChevronDown } from "lucide-react";
import { useState } from "react";
import { RESOURCE_TYPES } from "@util/config";
import { NewResourceDialog } from "@layouts/resources";

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
            {RESOURCE_TYPES.map((target) => (
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
      {open && (
        <NewResourceDialog type={open} open={open === "Deployment"} set={set} />
      )}
    </>
  );
};
