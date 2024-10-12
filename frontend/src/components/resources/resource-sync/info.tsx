import { Section } from "@components/layouts";
import { ReactNode, useState } from "react";
import { Card, CardContent, CardHeader } from "@ui/card";
import { useFullResourceSync } from ".";
import { updateLogToHtml } from "@lib/utils";
import { MonacoEditor } from "@components/monaco";
import { useEditPermissions } from "@pages/resource";
import { useWrite } from "@lib/hooks";
import { useToast } from "@ui/use-toast";
import { Button } from "@ui/button";
import { History } from "lucide-react";
import { ConfirmUpdate } from "@components/config/util";

export const ResourceSyncInfo = ({
  id,
  titleOther,
}: {
  id: string;
  titleOther: ReactNode;
}) => {
  const [edits, setEdits] = useState<Record<string, string | undefined>>({});
  const { canWrite } = useEditPermissions({ type: "ResourceSync", id });
  const { toast } = useToast();
  const { mutateAsync } = useWrite("WriteSyncFileContents", {
    onSuccess: (res) => {
      toast({
        title: res.success ? "Contents written." : "Failed to write contents.",
        variant: res.success ? undefined : "destructive",
      });
    },
  });
  const sync = useFullResourceSync(id);
  const file_on_host = sync?.config?.files_on_host ?? false;
  const git_repo = sync?.config?.repo ? true : false;
  const canEdit = canWrite && (file_on_host || git_repo);
  const editFileCallback = (path: string) => (contents: string) =>
    setEdits({ ...edits, [path]: contents });

  const latest_contents = sync?.info?.remote_contents;

  return (
    <Section titleOther={titleOther}>
      {/* ERRORS */}
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
                      if (sync) {
                        mutateAsync({
                          sync: sync.name,
                          file_path: content.path,
                          contents: edits[content.path]!,
                        }).then(() =>
                          setEdits({ ...edits, [content.path]: undefined })
                        );
                      }
                    }}
                    disabled={!edits[content.path]}
                    language="toml"
                  />
                </div>
              )}
            </CardHeader>
            <CardContent className="pr-8">
              <MonacoEditor
                value={edits[content.path] ?? content.contents}
                language="toml"
                readOnly={!canEdit}
                onValueChange={editFileCallback(content.path)}
              />
            </CardContent>
          </Card>
        ))}
    </Section>
  );
};
