import { Config } from "@components/config";
import { InputList, ResourceSelector } from "@components/config/util";
import { useRead, useWrite } from "@lib/hooks";
import { Types } from "@monitor/client";
import { useState } from "react";

export const BuilderConfig = ({ id }: { id: string }) => {
  const config = useRead("GetBuilder", { builder: id }).data?.config;
  if (config?.type === "Aws") return <AwsBuilderConfig id={id} />;
  if (config?.type === "Server") return <ServerBuilderConfig id={id} />;
};

const AwsBuilderConfig = ({ id }: { id: string }) => {
  const config = useRead("GetBuilder", { builder: id }).data?.config;
  const [update, set] = useState<Partial<Types.AwsBuilderConfig>>({});
  const { mutate } = useWrite("UpdateBuilder");
  if (!config) return null;

  return (
    <Config
      config={config.params as Types.AwsBuilderConfig}
      update={update}
      set={set}
      onSave={() => mutate({ id, config: { type: "Aws", params: update } })}
      components={{
        general: {
          general: {
            region: true,
            instance_type: true,
            volume_gb: true,
            ami_id: true,
            subnet_id: true,
            key_pair_name: true,
            assign_public_ip: true,
            use_public_ip: true,
            security_group_ids: (values, set) => (
              <InputList field="security_group_ids" values={values} set={set} />
            ),
            docker_accounts: (accounts, set) => (
              <InputList
                field="docker_accounts"
                values={accounts ?? []}
                set={set}
              />
            ),
            github_accounts: (accounts, set) => (
              <InputList
                field="github_accounts"
                values={accounts ?? []}
                set={set}
              />
            ),
          },
        },
      }}
    />
  );
};

const ServerBuilderConfig = ({ id }: { id: string }) => {
  const config = useRead("GetBuilder", { builder: id }).data?.config;
  const [update, set] = useState<Partial<Types.ServerBuilderConfig>>({});
  const { mutate } = useWrite("UpdateBuilder");
  if (!config) return null;

  return (
    <Config
      config={config.params as Types.ServerBuilderConfig}
      update={update}
      set={set}
      onSave={() => mutate({ id, config: { type: "Server", params: update } })}
      components={{
        general: {
          general: {
            server_id: (id, set) => (
              <div className="flex justify-between items-center border-b pb-4">
                Select Server
                <ResourceSelector
                  type="Server"
                  selected={id}
                  onSelect={(server_id) => set({ server_id })}
                />
              </div>
            ),
          },
        },
      }}
    />
  );
};
