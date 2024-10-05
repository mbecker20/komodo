import { Section } from "@components/layouts";
import { ReactNode, useState } from "react";
import { Card, CardContent, CardHeader } from "@ui/card";
import { useFullStack } from ".";
import { updateLogToHtml } from "@lib/utils";
import { MonacoEditor } from "@components/monaco";
import { useEditPermissions } from "@pages/resource";
import { ConfirmUpdate } from "@components/config/util";
import { useWrite } from "@lib/hooks";
import { Button } from "@ui/button";
import { History } from "lucide-react";

export const StackInfo = ({
  id,
  titleOther,
}: {
  id: string;
  titleOther: ReactNode;
}) => {
  const [edits, setEdits] = useState<Record<string, string | undefined>>({});
  const { canWrite } = useEditPermissions({ type: "Stack", id });
  const { mutateAsync } = useWrite("WriteStackFileContents");

  const stack = useFullStack(id);
  // const state = useStack(id)?.info.state ?? Types.StackState.Unknown;
  // const is_down = [Types.StackState.Down, Types.StackState.Unknown].includes(
  //   state
  // );

  const file_on_host = stack?.config?.files_on_host;
  const canEdit = canWrite && file_on_host;
  const editFileCallback = (path: string) => (contents: string) =>
    setEdits({ ...edits, [path]: contents });

  // Collect deployed / latest contents, joining
  // them by path.
  // Only unmatched latest contents end up in latest_contents.
  // const deployed_contents: {
  //   path: string;
  //   deployed: string;
  //   modified: string | undefined;
  // }[] = [];

  // if (!is_down) {
  //   for (const content of stack?.info?.deployed_contents ?? []) {
  //     const latest = stack?.info?.remote_contents?.find(
  //       (latest) => latest.path === content.path
  //     );
  //     const modified =
  //       latest?.contents &&
  //       (latest.contents !== content.contents ? latest.contents : undefined);
  //     deployed_contents.push({
  //       path: content.path,
  //       deployed: content.contents,
  //       modified,
  //     });
  //   }
  // }

  const latest_contents = stack?.info?.remote_contents;

  return (
    <Section titleOther={titleOther}>
      {/* ERRORS */}
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
      
      {/* Update deployed contents with diff */}
      {/* {!is_down && deployed_contents.length > 0 && (
        <Card>
          <CardHeader className="flex flex-col gap-2">
            deployed contents:{" "}
          </CardHeader>
          <CardContent>
            {deployed_contents.map((content) => {
              return (
                <pre key={content.path} className="flex flex-col gap-2">
                  <div className="flex justify-between items-center">
                    <div>path: {content.path}</div>
                    {canEdit && (
                      <div className="flex items-center gap-2">
                        <Button
                          variant="outline"
                          onClick={() =>
                            setEdits({ ...edits, [content.path]: undefined })
                          }
                          className="flex items-center gap-2"
                          disabled={!edits[content.path]}
                        >
                          <History className="w-4 h-4" />
                          Reset
                        </Button>
                        <ConfirmUpdate
                          previous={{
                            contents: content.modified ?? content.deployed,
                          }}
                          content={{ contents: edits[content.path] }}
                          onConfirm={() => {
                            if (stack) {
                              mutateAsync({
                                stack: stack.name,
                                file_path: content.path,
                                contents: edits[content.path]!,
                              }).then(() =>
                                setEdits({
                                  ...edits,
                                  [content.path]: undefined,
                                })
                              );
                            }
                          }}
                          disabled={!edits[content.path]}
                        />
                      </div>
                    )}
                  </div>
                  {content.modified ? (
                    <MonacoDiffEditor
                      original={"# Deployed contents\n" + content.deployed}
                      modified={edits[content.path] ?? content.modified}
                      language="yaml"
                      readOnly={!canEdit}
                      hideUnchangedRegions={false}
                      onModifiedValueChange={editFileCallback(content.path)}
                    />
                  ) : (
                    <MonacoEditor
                      value={edits[content.path] ?? content.deployed}
                      language="yaml"
                      readOnly={!canEdit}
                      onValueChange={editFileCallback(content.path)}
                    />
                  )}
                </pre>
              );
            })}
          </CardContent>
        </Card>
      )} */}

      {/* Update latest contents */}
      {latest_contents &&
        latest_contents.length > 0 &&
        latest_contents.map((content) => (
          <Card key={content.path} className="flex flex-col gap-2">
            <CardHeader className="flex flex-row justify-between items-center pb-0">
              <div className="font-mono">{content.path}</div>
              {canEdit && (
                <div className="flex items-center gap-2">
                  <Button
                    variant="outline"
                    onClick={() =>
                      setEdits({ ...edits, [content.path]: undefined })
                    }
                    className="flex items-center gap-2"
                    disabled={!edits[content.path]}
                  >
                    <History className="w-4 h-4" />
                    Reset
                  </Button>
                  <ConfirmUpdate
                    previous={{ contents: content.contents }}
                    content={{ contents: edits[content.path] }}
                    onConfirm={() => {
                      if (stack) {
                        mutateAsync({
                          stack: stack.name,
                          file_path: content.path,
                          contents: edits[content.path]!,
                        }).then(() =>
                          setEdits({ ...edits, [content.path]: undefined })
                        );
                      }
                    }}
                    disabled={!edits[content.path]}
                  />
                </div>
              )}
            </CardHeader>
            <CardContent className="pr-8">
              <MonacoEditor
                value={edits[content.path] ?? content.contents}
                language="yaml"
                readOnly={!canEdit}
                onValueChange={editFileCallback(content.path)}
              />
            </CardContent>
          </Card>
        ))}
    </Section>
  );
};
