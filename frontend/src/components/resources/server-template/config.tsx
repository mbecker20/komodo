import { Config } from "@components/config";
import { InputList } from "@components/config/util";
import { useRead, useWrite } from "@lib/hooks";
import { Types } from "@monitor/client";
import { useState } from "react";

export const ServerTemplateConfig = ({ id }: { id: string }) => {
  const config = useRead("GetServerTemplate", { server_template: id }).data
    ?.config;
  if (config?.type === "Aws") return <AwsServerTemplateConfig id={id} />;
};

export const AwsServerTemplateConfig = ({ id }: { id: string }) => {
  const perms = useRead("GetPermissionLevel", {
    target: { type: "ServerTemplate", id },
  }).data;
  const config = useRead("GetServerTemplate", { server_template: id }).data
    ?.config;
  const [update, set] = useState<Partial<Types.AwsServerTemplateConfig>>({});
  const { mutate } = useWrite("UpdateServerTemplate");
  if (!config) return null;

  const disabled = perms !== Types.PermissionLevel.Write;

  return (
    <Config
      disabled={disabled}
      config={config.params as Types.AwsServerTemplateConfig}
      update={update}
      set={set}
      onSave={() => mutate({ id, config: { type: "Aws", params: update } })}
      components={{
        general: {
          general: {
            region: true,
            instance_type: true,
            ami_id: true,
            subnet_id: true,
            key_pair_name: true,
            assign_public_ip: true,
            use_public_ip: true,
            security_group_ids: (values, set) => (
              <InputList
                field="security_group_ids"
                values={values}
                set={set}
                disabled={disabled}
              />
            ),
            port: true,
          },
        },
      }}
    />
  );
};
