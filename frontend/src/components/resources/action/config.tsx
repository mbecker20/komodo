import { useLocalStorage, useRead, useWrite } from "@lib/hooks";
import { Types } from "komodo_client";
import { Config } from "@components/config";
import { MonacoEditor } from "@components/monaco";
import { SecretsSearch } from "@components/config/env_vars";
import { Button } from "@ui/button";
import { ReactNode } from "react";

export const ActionConfig = ({ id, titleOther }: { id: string; titleOther: ReactNode }) => {
  const perms = useRead("GetPermissionLevel", {
    target: { type: "Action", id },
  }).data;
  const config = useRead("GetAction", { action: id }).data?.config;
  const global_disabled =
    useRead("GetCoreInfo", {}).data?.ui_write_disabled ?? false;
  const [update, set] = useLocalStorage<Partial<Types.ActionConfig>>(
    `action-${id}-update-v1`,
    {}
  );
  const { mutateAsync } = useWrite("UpdateAction");

  if (!config) return null;

  const disabled = global_disabled || perms !== Types.PermissionLevel.Write;

  return (
    <Config
      titleOther={titleOther}
      resource_id={id}
      resource_type="Action"
      disabled={disabled}
      config={config}
      update={update}
      set={set}
      onSave={async () => {
        await mutateAsync({ id, config: update });
      }}
      components={{
        "": [
          {
            label: "Action File",
            description: "Manage the action file contents here.",
            // actions: (
            //   <ShowHideButton
            //     show={show.file}
            //     setShow={(file) => setShow({ ...show, file })}
            //   />
            // ),
            // contentHidden: !show.file,
            components: {
              file_contents: (file_contents, set) => {
                return (
                  <div className="flex flex-col gap-4">
                    <div className="flex items-center justify-between">
                      <SecretsSearch />
                      <div className="hidden lg:flex items-center">
                        <div className="text-muted-foreground text-sm mr-2">
                          Docs:
                        </div>
                        {["read", "execute", "write"].map((api) => (
                          <a
                            key={api}
                            href={`https://docs.rs/komodo_client/latest/komodo_client/api/${api}/index.html`}
                            target="_blank"
                          >
                            <Button
                              className="capitalize px-1"
                              size="sm"
                              variant="link"
                            >
                              {api}
                            </Button>
                          </a>
                        ))}
                      </div>
                    </div>
                    <MonacoEditor
                      value={file_contents}
                      onValueChange={(file_contents) => set({ file_contents })}
                      language="typescript"
                      readOnly={disabled}
                    />
                  </div>
                );
              },
            },
          },
        ],
      }}
    />
  );
};
