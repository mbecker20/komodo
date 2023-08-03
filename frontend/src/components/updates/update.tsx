import { version_to_string } from "@util/helpers";
import { Calendar, User } from "lucide-react";
import { UpdateDetails, UpdateUser } from "./details";
import { Update } from "@monitor/client/dist/types";

const fmt_date = (d: Date) =>
  `${d.getDate()}/${d.getMonth() + 1} @ ${d.getHours()}:${d.getMinutes()}`;

export const SingleUpdate = ({ update }: { update: Update }) => (
  <UpdateDetails update={update}>
    <div className="px-2 py-4 hover:bg-muted transition-colors border-b last:border-none cursor-pointer">
      <div
        className="grid gap-4 justify-start items-center"
        style={{ gridTemplateColumns: "1fr 1.75fr 1fr" }}
      >
        <div className="flex items-center gap-2">
          <Calendar className="w-4 h-4" />
          <div className="text-xs">
            {update.end_ts ? fmt_date(new Date(update.end_ts)) : "ongoing"}
          </div>
        </div>

        <div className="text-sm w-full">
          {update.operation.match(/[A-Z][a-z]+|[0-9]+/g)?.join(" ")}{" "}
          {version_to_string(update.version)}
        </div>

        <div className="flex items-center gap-2 text-sm">
          <User className="w-4 h-4" />
          <UpdateUser userId={update.operator} />
        </div>
      </div>
    </div>
  </UpdateDetails>
);
