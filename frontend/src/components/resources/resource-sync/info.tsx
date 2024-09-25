import { Section } from "@components/layouts";
import { ReactNode } from "react";
import { Card, CardHeader } from "@ui/card";
import { useFullResourceSync } from ".";
import { updateLogToHtml } from "@lib/utils";
import { MonacoEditor } from "@components/monaco";

export const ResourceSyncInfo = ({
  id,
  titleOther,
}: {
  id: string;
  titleOther: ReactNode;
}) => {
  const sync = useFullResourceSync(id);
  return (
    <Section titleOther={titleOther}>
      {sync?.info?.remote_contents &&
        sync?.info?.remote_contents.length > 0 && (
          <Card>
            <CardHeader className="flex flex-col gap-2">
              latest contents:{" "}
              {sync?.info?.remote_contents?.map((content, i) => (
                <pre key={i} className="flex flex-col gap-2">
                  path: {content.path}
                  <MonacoEditor
                    value={content.contents}
                    language="yaml"
                    readOnly
                  />
                </pre>
              ))}
            </CardHeader>
          </Card>
        )}
      {sync?.info?.remote_errors && sync?.info?.remote_errors.length > 0 && (
        <Card>
          <CardHeader className="flex flex-col gap-2">
            remote errors:{" "}
            {sync?.info?.remote_errors?.map((content, i) => (
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
