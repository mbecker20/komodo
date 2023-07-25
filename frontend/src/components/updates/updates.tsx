import { version_to_string } from "@util/helpers";
import { Calendar, User } from "lucide-react";
import { UpdateDetails, UpdateUser } from "./update";
import { Update } from "@monitor/client/dist/types";

export const SingleUpdate = ({ update }: { update: Update }) => (
  <div className="flex items-center justify-between">
    <div>
      {update.operation
        .split("_")
        .map((s) => s[0].toUpperCase() + s.slice(1))
        .join(" ")}{" "}
      {version_to_string(update.version)}
    </div>
    <div>
      <div className="flex gap-2 items-center md:w-[200px]">
        <Calendar className="w-4 h-4" />
        <div className="text-xs md:text-sm">
          {update.end_ts ? new Date(update.end_ts).toLocaleString() : "ongoing"}
        </div>
      </div>
    </div>
    <div className="flex gap-2 items-center">
      <User className="w-4 h-4" />
      <div>
        <UpdateUser userId={update.operator} />
      </div>
    </div>
    <UpdateDetails update={update} />
  </div>
);
