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
      <DropdownMenuContent>
        {updates?.map((update) => (
          <DropdownMenuGroup>
            <div className="p-2 hover:bg-muted transition-colors">
              <SingleUpdate update={update} />
            </div>
          </DropdownMenuGroup>
        ))}
      </DropdownMenuContent>
    </DropdownMenu>
  );
};
