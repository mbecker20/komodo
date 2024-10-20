import { Section } from "@components/layouts";
import { ReactNode } from "react";
import { Card, CardContent, CardHeader } from "@ui/card";
import { cn, getUpdateQuery, updateLogToHtml } from "@lib/utils";
import { useRead } from "@lib/hooks";
import { text_color_class_by_intention } from "@lib/color";

export const ActionInfo = ({
  id,
  titleOther,
}: {
  id: string;
  titleOther: ReactNode;
}) => {
  const update = useRead("ListUpdates", {
    query: {
      ...getUpdateQuery({ type: "Action", id }, undefined),
      operation: "RunAction",
    },
  }).data?.updates[0];

  const full_update = useRead(
    "GetUpdate",
    { id: update?.id! },
    { enabled: !!update?.id }
  ).data;

  const log = full_update?.logs.find((log) => log.stage === "Execute Action");

  return (
    <Section titleOther={titleOther}>
      {!log?.stdout && !log?.stderr && (
        <Card className="flex flex-col gap-4">
          <CardHeader
            className={cn(
              "flex flex-row justify-between items-center",
              text_color_class_by_intention("Neutral")
            )}
          >
            Never run
          </CardHeader>
        </Card>
      )}
      {/* Last run */}
      {log?.stdout && (
        <Card className="flex flex-col gap-4">
          <CardHeader
            className={cn(
              "flex flex-row justify-between items-center pb-0",
              text_color_class_by_intention("Good")
            )}
          >
            Stdout
          </CardHeader>
          <CardContent className="pr-8">
            <pre
              dangerouslySetInnerHTML={{
                __html: updateLogToHtml(log.stdout),
              }}
              className="max-h-[500px] overflow-y-auto"
            />
          </CardContent>
        </Card>
      )}
      {log?.stderr && (
        <Card className="flex flex-col gap-4">
          <CardHeader
            className={cn(
              "flex flex-row justify-between items-center pb-0",
              text_color_class_by_intention("Critical")
            )}
          >
            Stderr
          </CardHeader>
          <CardContent className="pr-8">
            <pre
              dangerouslySetInnerHTML={{
                __html: updateLogToHtml(log.stderr),
              }}
              className="max-h-[500px] overflow-y-auto"
            />
          </CardContent>
        </Card>
      )}
    </Section>
  );
};
