import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@ui/card";
import { Types } from "@monitor/client";
import { cn, version_to_string } from "@util/helpers";
import { Calendar, User } from "lucide-react";
import { UpdateDetails, UpdateUser } from "./update";

export const Updates = ({
  updates,
  className,
}: {
  updates?: Types.Update[];
  className?: string;
}) => (
  <Card className={cn("w-full h-fit", className)}>
    <CardHeader className="flex-row justify-between">
      <CardTitle className="text-xl">Updates</CardTitle>
    </CardHeader>
    <CardContent className="flex flex-col gap-2 max-h-[50vh] overflow-y-auto">
      {updates?.map((update) => (
        <Card key={update._id?.$oid}>
          <CardHeader className="flex flex-row items-end md:items-center justify-between gap-4">
            <div
              className="flex flex-col md:flex-row justify-between items-center w-full"
              style={{ placeItems: "center start" }}
            >
              <CardTitle className="whitespace-nowrap w-full">
                {update.operation
                  .split("_")
                  .map((s) => s[0].toUpperCase() + s.slice(1))
                  .join(" ")}{" "}
                {version_to_string(update.version)}
              </CardTitle>
              <div className="flex flex-col-reverse md:flex-row md:gap-4 w-full">
                <div className="flex gap-2 items-center md:w-[200px]">
                  <Calendar className="w-4 h-4" />
                  <CardDescription className="text-xs md:text-sm">
                    {update.end_ts
                      ? new Date(update.end_ts).toLocaleString()
                      : "ongoing"}
                  </CardDescription>
                </div>
                <div className="flex gap-2 items-center">
                  <User className="w-4 h-4" />
                  <CardDescription>
                    <UpdateUser userId={update.operator} />
                  </CardDescription>
                </div>
              </div>
            </div>
            <UpdateDetails update={update} />
          </CardHeader>
        </Card>
      ))}
    </CardContent>
  </Card>
);
