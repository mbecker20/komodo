import { ConfigAgain } from "@components/config/again";
import { useWrite, useRead } from "@hooks";
import { ConfigLayout } from "@layouts/page";
import { Types } from "@monitor/client";
import {
  Select,
  SelectTrigger,
  SelectValue,
  SelectContent,
  SelectItem,
} from "@ui/select";
import { Button } from "@ui/button";
import { Card, CardHeader, CardTitle, CardContent } from "@ui/card";
import { useState } from "react";
import { InputList, ResourceSelector } from "@components/config/util";

const BuilderTypeSelector = ({
  selected,
  onSelect,
}: {
  selected: Types.BuilderConfig["type"] | undefined;
  onSelect: (type: Types.BuilderConfig["type"]) => void;
}) => (
  <div className="flex justify-between items-center border-b pb-4">
    Builder Type
    <Select value={selected || undefined} onValueChange={onSelect}>
      <SelectTrigger className="max-w-[150px]">
        <SelectValue placeholder="Select Type" />
      </SelectTrigger>
      <SelectContent>
        <SelectItem value={"Aws"}>Aws</SelectItem>
        <SelectItem value={"Server"}>Server</SelectItem>
      </SelectContent>
    </Select>
  </div>
);

const default_aws_config: Types.AwsBuilderConfig = {
  ami_id: "",
  assign_public_ip: false,
  instance_type: "",
  key_pair_name: "",
  region: "",
  subnet_id: "",
  volume_gb: 0,
  security_group_ids: [],
};

const BuilderConfigInner = ({
  id,
  config,
}: {
  id: string;
  config: Types.BuilderConfig;
}) => {
  const [update, setUpdate] = useState({ type: config.type, params: {} });
  const { mutate } = useWrite("UpdateBuilder");
  return (
    <ConfigLayout
      content={update}
      onConfirm={() => mutate({ id, config: update })}
      onReset={() => setUpdate({ type: config.type, params: {} })}
    >
      <div className="flex gap-4">
        <div className="flex flex-col gap-4 w-[300px]">
          <Button variant="secondary">General</Button>
        </div>
        <Card className="w-full">
          <CardHeader className="border-b">
            <CardTitle>General</CardTitle>
          </CardHeader>
          <CardContent className="flex flex-col gap-4 mt-6">
            <BuilderTypeSelector
              selected={update.type}
              onSelect={(type) =>
                setUpdate({
                  type,
                  params: type === "Aws" ? default_aws_config : { id: "" },
                })
              }
            />

            {/* Server Builder */}
            {update.type === "Server" && (
              <ConfigAgain
                config={config.params as Types.ServerBuilderConfig}
                update={update.params}
                set={(u) =>
                  setUpdate((p) => ({ ...p, params: { ...p.params, ...u } }))
                }
                components={{
                  id: (id, set) => (
                    <div className="flex justify-between items-center border-b pb-4">
                      Select Server
                      <ResourceSelector
                        type="Server"
                        selected={id}
                        onSelect={(id) => set({ id })}
                      />
                    </div>
                  ),
                }}
              />
            )}

            {/* Aws Builder */}
            {update.type === "Aws" && (
              <ConfigAgain
                config={config.params as Types.AwsBuilderConfig}
                update={update.params}
                set={(u) =>
                  setUpdate((p) => ({ ...p, params: { ...p.params, ...u } }))
                }
                components={{
                  region: true,
                  instance_type: true,
                  volume_gb: true,
                  ami_id: true,
                  subnet_id: true,
                  key_pair_name: true,
                  assign_public_ip: true,
                  security_group_ids: (values, set) => (
                    <InputList
                      field="security_group_ids"
                      values={values}
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
                  github_accounts: (accounts, set) => (
                    <InputList
                      field="github_accounts"
                      values={accounts ?? []}
                      set={set}
                    />
                  ),
                }}
              />
            )}
          </CardContent>
        </Card>
      </div>
    </ConfigLayout>
  );
};

export const BuilderConfig = ({ id }: { id: string }) => {
  const config = useRead("GetBuilder", { id }).data?.config;
  if (!config) return null;
  return <BuilderConfigInner id={id} config={config} />;
};
