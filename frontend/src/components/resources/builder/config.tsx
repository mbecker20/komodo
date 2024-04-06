import { Config } from "@components/config";
import { InputList, ResourceSelector } from "@components/config/util";
import { ActionWithDialog } from "@components/util";
import { useRead, useWrite } from "@lib/hooks";
import { Types } from "@monitor/client";
import { Trash } from "lucide-react";
import { useState } from "react";
import { useNavigate } from "react-router-dom";

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
            github_accounts: (accounts, set) => (
              <InputList
                field="github_accounts"
                values={accounts ?? []}
                set={set}
              />
            ),
            docker_accounts: (accounts, set) => (
              <InputList
                field="docker_accounts"
                values={accounts ?? []}
                set={set}
              />
            ),
            port: true,
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

export const DeleteBuilder = ({ id }: { id: string }) => {
  const nav = useNavigate();
  const builder = useRead("GetBuilder", { builder: id }).data;
  const { mutateAsync, isPending } = useWrite("DeleteBuilder");

  if (!builder) return null;

  return (
    <div className="flex items-center justify-between">
      <div className="w-full">Delete Builder</div>
      <ActionWithDialog
        name={builder.name}
        title="Delete"
        icon={<Trash className="h-4 w-4" />}
        onClick={async () => {
          await mutateAsync({ id });
          nav("/");
        }}
        disabled={isPending}
        loading={isPending}
      />
    </div>
  );
};
