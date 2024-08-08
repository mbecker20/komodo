import { Section } from "@components/layouts";
import { ReactNode } from "react";
import { useRead } from "@lib/hooks";
import { useStack } from ".";
import { Card, CardDescription, CardHeader } from "@ui/card";
import { sanitizeOnlySpan } from "@lib/utils";

export const StackInfo = ({
  id,
  titleOther,
}: {
  id: string;
  titleOther: ReactNode;
}) => {
  const stack = useRead("GetStack", { stack: id }).data;
  const info = useStack(id)?.info;
  return (
    <Section titleOther={titleOther}>
      <div>project missing: {info?.project_missing ? "true" : "false"}</div>
      {stack?.info?.deployed_message && (
        <div>deployed message: {stack?.info?.deployed_message}</div>
      )}
      {stack?.info?.deployed_contents && (
        <div>
          <CardDescription>Deployed File</CardDescription>
          {stack?.info?.deployed_contents?.map((content) => (
            <pre className="flex flex-col gap-2">
              path: {content.path}
              <pre
                dangerouslySetInnerHTML={{
                  __html: sanitizeOnlySpan(content.contents),
                }}
                className="max-h-[500px] overflow-y-auto"
              />
            </pre>
          ))}
        </div>
        // <Card>
        //   <CardHeader>
        //     deployed contents:{" "}
        //     {stack?.info?.deployed_contents?.map((content) => (
        //       <pre className="flex flex-col gap-2">
        //         path: {content.path}
        //         <pre>{content.contents}</pre>
        //       </pre>
        //     ))}
        //   </CardHeader>
        // </Card>
      )}

      {/* LATEST */}
      {stack?.info?.latest_message && (
        <div>latest message: {stack?.info?.latest_message}</div>
      )}
      {stack?.info?.remote_contents && (
        <Card>
          <CardHeader>
            remote contents:{" "}
            {stack?.info?.remote_contents?.map((content) => (
              <pre className="flex flex-col gap-2">
                path: {content.path}
                <pre>{content.contents}</pre>
              </pre>
            ))}
          </CardHeader>
        </Card>
      )}
      {stack?.info?.remote_errors && (
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
      {stack?.info?.latest_json_errors && (
        <pre>
          latest json error:{" "}
          {stack?.info?.latest_json_errors?.map((content) => (
            <div>
              path: {content.path}
              <pre>{content.contents}</pre>
            </div>
          ))}
        </pre>
      )}
    </Section>
  );
};
