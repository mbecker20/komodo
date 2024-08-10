import { Section } from "@components/layouts";
import { ReactNode } from "react";
import { Card, CardHeader } from "@ui/card";
import { useFullStack, useStack } from ".";
import { Types } from "@monitor/client";

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
          <CardHeader>
            deployed contents:{" "}
            {stack?.info?.deployed_contents?.map((content) => (
              <pre className="flex flex-col gap-2">
                path: {content.path}
                <pre>{content.contents}</pre>
              </pre>
            ))}
          </CardHeader>
        </Card>
      )}

      {stack?.config?.file_contents ? (
        <Card>
          <CardHeader>
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
            <CardHeader>
              latest contents:{" "}
              {stack?.info?.remote_contents?.map((content) => (
                <pre className="flex flex-col gap-2">
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
          <CardHeader>
            remote errors:{" "}
            {stack?.info?.remote_errors?.map((content) => (
              <pre className="flex flex-col gap-2">
                path: {content.path}
                <pre>{content.contents}</pre>
              </pre>
            ))}
          </CardHeader>
        </Card>
      )}
    </Section>
  );
};
