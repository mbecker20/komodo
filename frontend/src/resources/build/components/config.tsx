import { ConfigAgain } from "@components/config/again";
import { AccountSelector, ResourceSelector } from "@components/config/util";
import { useWrite, useRead } from "@hooks";
import { ConfigLayout } from "@layouts/page";
import { Types } from "@monitor/client";
import { Button } from "@ui/button";
import { Card, CardHeader, CardTitle, CardContent } from "@ui/card";
import { useState } from "react";

const BuildConfigInner = ({
  id,
  config,
}: {
  id: string;
  config: Types.BuildConfig;
}) => {
  const [update, set] = useState<Partial<Types.BuildConfig>>({});
  const [show, setShow] = useState("general");
  const { mutate } = useWrite("UpdateBuild");

  return (
    <ConfigLayout
      content={update}
      onConfirm={() => mutate({ id, config: update })}
      onReset={() => set({})}
    >
      <div className="flex gap-4">
        <div className="flex flex-col gap-4 w-[300px]">
          {["general", "docker", "volumes"].map((item) => (
            <Button
              variant={show === item ? "secondary" : "outline"}
              onClick={() => setShow(item)}
              className="capitalize"
            >
              {item}
            </Button>
          ))}
        </div>
        <Card className="w-full">
          <CardHeader className="border-b">
            <CardTitle className="capitalize">{show}</CardTitle>
          </CardHeader>
          <CardContent className="flex flex-col gap-4 mt-4">
            {/* General Config */}
            {show === "general" && (
              <ConfigAgain
                config={config}
                update={update}
                set={(u) => set((p) => ({ ...p, ...u }))}
                components={{
                  builder_id: (id, set) => (
                    <div className="flex justify-between items-center border-b pb-4 min-h-[60px]">
                      <div>Builder</div>
                      <ResourceSelector
                        type="Builder"
                        selected={id}
                        onSelect={(builder_id) => set({ builder_id })}
                      />
                    </div>
                  ),
                  repo: true,
                  branch: true,
                  github_account: (account, set) => (
                    <AccountSelector
                      id={update.builder_id ?? config.builder_id ?? undefined}
                      type="Builder"
                      account_type="github"
                      selected={account}
                      onSelect={(github_account) => set({ github_account })}
                    />
                  ),
                }}
              />
            )}

            {/* Docker Config */}
            {show === "docker" && (
              <ConfigAgain
                config={config}
                update={update}
                set={(u) => set((p) => ({ ...p, ...u }))}
                components={{
                  build_path: true,
                  dockerfile_path: true,
                  docker_account: (account, set) => (
                    <AccountSelector
                      id={update.builder_id ?? config.builder_id ?? undefined}
                      type="Builder"
                      account_type="docker"
                      selected={account}
                      onSelect={(docker_account) => set({ docker_account })}
                    />
                  ),
                  // docker_organization,
                  use_buildx: true,
                }}
              />
            )}
          </CardContent>
        </Card>
      </div>
    </ConfigLayout>
  );
};

export const BuildConfig = ({ id }: { id: string }) => {
  const config = useRead("GetBuild", { id }).data?.config;
  if (!config) return null;
  return <BuildConfigInner id={id} config={config} />;
};
