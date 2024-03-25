import {
  Sheet,
  SheetContent,
  SheetDescription,
  SheetHeader,
  SheetTitle,
  SheetTrigger,
} from "@ui/sheet";
import { Calendar, Clock, Milestone, User } from "lucide-react";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@ui/card";
import { ReactNode, useState } from "react";
import { useRead } from "@lib/hooks";
import { fmt_duration, fmt_version } from "@lib/utils";
import { ResourceComponents } from "@components/resources";
import { Link } from "react-router-dom";

export const UpdateUser = ({ user_id }: { user_id: string }) => {
  const username = useRead("GetUsername", { user_id }).data;
  if (user_id === "github") return <>GitHub</>;
  if (user_id === "auto redeploy") return <>Auto Redeploy</>;
  return <>{username?.username}</>;
};

export const UpdateDetails = ({
  id,
  children,
}: {
  id: string;
  children: ReactNode;
}) => {
  const [open, setOpen] = useState(false);

  const update = useRead("GetUpdate", { id }).data;
  if (!update) return null;

  const Components =
    update.target.type === "System"
      ? null
      : ResourceComponents[update.target.type];

  if (!Components) return null;

  return (
    <Sheet open={open} onOpenChange={setOpen}>
      <SheetTrigger asChild>{children}</SheetTrigger>
      <SheetContent className="overflow-y-auto w-[100vw] md:w-[75vw] lg:w-[50vw]">
        <SheetHeader className="mb-4">
          <SheetTitle>
            {update.operation
              .split("_")
              .map((s) => s[0].toUpperCase() + s.slice(1))
              .join(" ")}{" "}
            {fmt_version(update.version)}
          </SheetTitle>
          <SheetDescription className="flex flex-col gap-2">
            <div className="flex items-center gap-2">
              <User className="w-4 h-4" />
              <UpdateUser user_id={update.operator} />
            </div>
            <div className="flex gap-4">
              <Link to={`/${update.target.type.toLowerCase()}s/${update.target.id}`}>
                <div className="flex items-center gap-2" onClick={() => setOpen(false)}>
                  <Components.Icon id={update.target.id} />
                  <Components.Name id={update.target.id} />
                </div>
              </Link>
              {update.version && (
                <div className="flex items-center gap-2">
                  <Milestone className="w-4 h-4" />
                  {fmt_version(update.version)}
                </div>
              )}
            </div>
            <div className="flex gap-4">
              <div className="flex items-center gap-2">
                <Calendar className="w-4 h-4" />
                {new Date(update.start_ts).toLocaleString()}
              </div>
              <div className="flex items-center gap-2">
                <Clock className="w-4 h-4" />
                {update.end_ts
                  ? fmt_duration(update.start_ts, update.end_ts)
                  : "ongoing"}
              </div>
            </div>
          </SheetDescription>
        </SheetHeader>
        <div className="grid gap-2">
          {update.logs?.map((log, i) => (
            <Card key={i}>
              <CardHeader className="flex-col">
                <CardTitle>{log.stage}</CardTitle>
                <CardDescription className="flex gap-2">
                  <span>
                    Stage {i + 1} of {update.logs.length}
                  </span>
                  <span>|</span>
                  <span className="flex items-center gap-2">
                    <Clock className="w-4 h-4" />
                    {fmt_duration(log.start_ts, log.end_ts)}
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
