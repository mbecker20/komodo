import { useRead, useUser, useUserInvalidate, useWrite } from "@lib/hooks";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuGroup,
  DropdownMenuTrigger,
} from "@ui/dropdown-menu";
import { Bell, Check, Circle, Loader2, X } from "lucide-react";
import { Button } from "@ui/button";
import { Calendar, User } from "lucide-react";
import { UpdateDetails, UpdateUser } from "./details";
import { ResourceComponents } from "@components/resources";
import { cn, version_is_none } from "@lib/utils";
import { Types } from "@monitor/client";
import { fmt_date, fmt_version } from "@lib/formatting";

export const SingleUpdate = ({ update }: { update: Types.UpdateListItem }) => {
  const Components =
    update.target.type !== "System"
      ? ResourceComponents[update.target.type]
      : null;

  const Icon = () => {
    if (update.status === Types.UpdateStatus.Complete) {
      if (update.success) return <Check className="w-4 h-4 stroke-green-500" />;
      else return <X className="w-4 h-4 stroke-red-500" />;
    } else return <Loader2 className="w-4 h-4 animate-spin" />;
  };

  return (
    <UpdateDetails id={update.id}>
      <div className="px-2 py-4 hover:bg-muted transition-colors border-b last:border-none cursor-pointer">
        <div className="flex items-center justify-between">
          <div className="text-sm w-full">
            <div className="flex items-center gap-2">
              <Icon />
              {update.operation.match(/[A-Z][a-z]+|[0-9]+/g)?.join(" ")}
              <div className="text-xs text-muted-foreground">
                {!version_is_none(update.version) &&
                  fmt_version(update.version)}
              </div>
            </div>
            <div className="flex items-center gap-2 text-muted-foreground">
              {Components && <Components.Icon />}
              {Components && <Components.Name id={update.target.id} />}
            </div>
          </div>
          <div className="text-xs text-muted-foreground w-48">
            <div className="flex items-center gap-2 h-[20px]">
              <Calendar className="w-4 h-4" />
              <div>
                {update.status === Types.UpdateStatus.InProgress
                  ? "ongoing"
                  : fmt_date(new Date(update.start_ts))}
              </div>
            </div>
            <div className="flex items-center gap-2 h-[20px]">
              <User className="w-4 h-4" />
              <UpdateUser user_id={update.operator} />
            </div>
          </div>
        </div>
      </div>
    </UpdateDetails>
  );
};

export const HeaderUpdates = () => {
  const updates = useRead("ListUpdates", {}).data;

  const last_opened = useUser().data?.last_update_view;
  const unseen_update = updates?.updates.some(
    (u) => u.start_ts > (last_opened ?? Number.MAX_SAFE_INTEGER)
  );

  const userInvalidate = useUserInvalidate();
  const { mutate } = useWrite("SetLastSeenUpdate", {
    onSuccess: userInvalidate,
  });

  return (
    <DropdownMenu onOpenChange={(o) => o && mutate({})}>
      <DropdownMenuTrigger asChild>
        <Button variant="ghost" size="icon" className="relative">
          <Bell className="w-4 h-4" />
          <Circle
            className={cn(
              "absolute top-2 right-2 w-2 h-2 stroke-red-500 fill-red-500 transition-opacity",
              unseen_update ? "opacity-1" : "opacity-0"
            )}
          />
        </Button>
      </DropdownMenuTrigger>
      <DropdownMenuContent className="w-[100vw] md:w-[500px] h-[500px] overflow-auto">
        <DropdownMenuGroup>
          {updates?.updates.map((update) => (
            <SingleUpdate update={update} key={update.id} />
          ))}
        </DropdownMenuGroup>
      </DropdownMenuContent>
    </DropdownMenu>
  );
};
