import { Button } from "@ui/button";
import {
  Sheet,
  SheetContent,
  SheetDescription,
  SheetHeader,
  SheetTitle,
  SheetTrigger,
} from "@ui/sheet";
import { Update } from "@monitor/client/dist/types";
import {
  readableDuration,
  readableVersion,
  version_to_string,
} from "@util/helpers";
import { Calendar, Clock, Milestone, Search, User } from "lucide-react";
// import { UpdateUser } from ".";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@ui/card";
// import { useRead } from "@hooks";

export const UpdateUser = ({ userId }: { userId: string }) => {
  // const { data } = useRead({ type: "GetUser", params: {} });
  if (userId === "github") return <>GitHub</>;
  if (userId === "auto redeploy") return <>Auto Redeploy</>;
  return <>{userId.slice(0, 5)}...</>;
};

export const UpdateDetails = ({ update }: { update: Update }) => {
  return (
    <Sheet>
      <SheetTrigger asChild>
        <Button
          variant="outline"
          size="sm"
          className="flex items-center gap-2 w-full max-w-fit"
        >
          <Search className="w-4 h-4" />
        </Button>
      </SheetTrigger>
      <SheetContent position="right" size="lg">
        <SheetHeader className="mb-4">
          <SheetTitle>
            {update.operation
              .split("_")
              .map((s) => s[0].toUpperCase() + s.slice(1))
              .join(" ")}{" "}
            {version_to_string(update.version)}
          </SheetTitle>
          <SheetDescription className="flex flex-col gap-2">
            <div className="flex items-center gap-2">
              <Calendar className="w-4 h-4" />
              {new Date(update.start_ts).toLocaleString()}
            </div>
            <div className="flex items-center gap-2">
              <Clock className="w-4 h-4" />
              {update.end_ts
                ? readableDuration(update.start_ts, update.end_ts)
                : "ongoing"}
            </div>
            <div className="flex items-center gap-2">
              <User className="w-4 h-4" />
              <UpdateUser userId={update.operator} />
            </div>
            {update.version && (
              <div className="flex items-center gap-2">
                <Milestone className="w-4 h-4" />
                {readableVersion(update.version)}
              </div>
            )}
          </SheetDescription>
        </SheetHeader>
        <div className="max-h-[80vh] overflow-y-auto grid gap-2">
          {update.logs.map((log, i) => (
            <Card>
              <CardHeader>
                <CardTitle>{log.stage}</CardTitle>
                <CardDescription className="flex gap-2">
                  <span>
                    Stage {i + 1} of {update.logs.length}
                  </span>
                  <span>|</span>
                  <span className="flex items-center gap-2">
                    <Clock className="w-4 h-4" />
                    {readableDuration(log.start_ts, log.end_ts)}
                  </span>
                </CardDescription>
              </CardHeader>
              <CardContent className="flex flex-col gap-2">
                {log.command && (
                  <div>
                    <CardDescription>command</CardDescription>
                    <pre className="max-h-[500px] overflow-y-auto">
                      {log.command}
                    </pre>
                  </div>
                )}
                {log.stdout && (
                  <div>
                    <CardDescription>stdout</CardDescription>
                    <pre className="max-h-[500px] overflow-y-auto">
                      {log.stdout}
                    </pre>
                  </div>
                )}
                {log.stderr && (
                  <div>
                    <CardDescription>stdout</CardDescription>
                    <pre className="max-h-[500px] overflow-y-auto">
                      {log.stderr}
                    </pre>
                  </div>
                )}
              </CardContent>
            </Card>
          ))}
        </div>
      </SheetContent>
    </Sheet>
  );
};
