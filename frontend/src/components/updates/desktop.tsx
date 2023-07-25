import { useRead } from "@hooks";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
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
          <DropdownMenuItem key={update._id?.$oid}>
            <SingleUpdate update={update} />
          </DropdownMenuItem>
        ))}
      </DropdownMenuContent>
    </DropdownMenu>
  );
};
