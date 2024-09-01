import { Section } from "@components/layouts";
import { ReactNode } from "react";
import { Card, CardHeader } from "@ui/card";
import { useFullStack, useStack } from ".";
import { Types } from "@komodo/client";
import { updateLogToHtml } from "@lib/utils";

export const StackInfo = ({
  id,
  titleOther,
}: {
  id: string;
  titleOther: ReactNode;
}) => {
  const stack = useFullStack(id);
  const state = useStack(id)?.info.state ?? Types.StackState.Unknown;
  const is_down = [Types.StackState.Down, Types.StackState.Unknown].includes(
    state
  );
  return (
    <Section titleOther={titleOther}>
      {!is_down && stack?.info?.deployed_contents && (
        <Card>
          <CardHeader className="flex flex-col gap-2">
            deployed contents:{" "}
            {stack?.info?.deployed_contents?.map((content, i) => (
              <pre key={i} className="flex flex-col gap-2">
                path: {content.path}
                <pre>{content.contents}</pre>
              </pre>
            ))}
          </CardHeader>
        </Card>
      )}

      {stack?.config?.file_contents ? (
        <Card>
          <CardHeader className="flex flex-col gap-2">
            latest contents:{" "}
            <pre className="flex flex-col gap-2">
              defined in UI:
              <pre>{stack?.config?.file_contents}</pre>
            </pre>
          </CardHeader>
        </Card>
      ) : (
        stack?.info?.remote_contents &&
        stack?.info?.remote_contents.length > 0 && (
          <Card>
            <CardHeader className="flex flex-col gap-2">
              latest contents:{" "}
              {stack?.info?.remote_contents?.map((content, i) => (
                <pre key={i} className="flex flex-col gap-2">
                  path: {content.path}
                  <pre>{content.contents}</pre>
                </pre>
              ))}
            </CardHeader>
          </Card>
        )
      )}
      {stack?.info?.remote_errors && stack?.info?.remote_errors.length > 0 && (
        <Card>
          <CardHeader className="flex flex-col gap-2">
            remote errors:{" "}
            {stack?.info?.remote_errors?.map((content, i) => (
              <pre key={i} className="flex flex-col gap-2">
                path: {content.path}
                <pre
                  dangerouslySetInnerHTML={{
                    __html: updateLogToHtml(content.contents),
                  }}
                  className="max-h-[500px] overflow-y-auto"
                />
              </pre>
            ))}
          </CardHeader>
        </Card>
      )}
    </Section>
  );
};
