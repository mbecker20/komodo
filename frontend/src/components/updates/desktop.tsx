import { useRead } from "@hooks";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuGroup,
  DropdownMenuTrigger,
} from "@ui/dropdown";
import { Bell } from "lucide-react";
import { SingleUpdate } from "./updates";
import { Button } from "@ui/button";

export const DesktopUpdates = () => {
  const updates = useRead("ListUpdates", {}).data;

  return (
    <DropdownMenu>
      <DropdownMenuTrigger>
        <Button variant="ghost">
          <Bell className="w-4 h-4" />
        </Button>
      </DropdownMenuTrigger>
      <DropdownMenuContent className="w-[500px]">
        <DropdownMenuGroup>
          {updates?.map((update) => (
            <div className="p-2 hover:bg-muted transition-colors border-b last:border-none">
              <SingleUpdate update={update} />
            </div>
          ))}
        </DropdownMenuGroup>
      </DropdownMenuContent>
    </DropdownMenu>
  );
};
