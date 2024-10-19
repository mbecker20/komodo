import { useState } from "react";
import { useRead, useWrite } from "@lib/hooks";
import { Types } from "komodo_client";
import { Config } from "@components/config";
import { MonacoEditor } from "@components/monaco";
import { SecretsSearch } from "@components/config/env_vars";

export const ActionConfig = ({ id }: { id: string }) => {
  const perms = useRead("GetPermissionLevel", {
    target: { type: "Action", id },
  }).data;
  const config = useRead("GetAction", { action: id }).data?.config;
  const global_disabled =
    useRead("GetCoreInfo", {}).data?.ui_write_disabled ?? false;
  const [update, set] = useState<Partial<Types.ActionConfig>>({});
  const { mutateAsync } = useWrite("UpdateAction");

  if (!config) return null;

  const disabled = global_disabled || perms !== Types.PermissionLevel.Write;

  return (
    <Config
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
                    <SecretsSearch />
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
