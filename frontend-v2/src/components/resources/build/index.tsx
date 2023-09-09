import { ConfigInner } from "@components/config";
import { ResourceSelector, AccountSelector } from "@components/config/util";
import { NewResource } from "@components/layouts";
import { useRead, useWrite } from "@lib/hooks";
import { fmt_verison } from "@lib/utils";
import { Types } from "@monitor/client";
import { RequiredResourceComponents } from "@types";
import { Input } from "@ui/input";
import { Hammer, History } from "lucide-react";
import { useState } from "react";

const useBuild = (id?: string) =>
  useRead("ListBuilds", {}).data?.find((d) => d.id === id);

const NewBuild = () => {
  const { mutateAsync } = useWrite("CreateBuild");
  const [name, setName] = useState("");
  return (
    <NewResource
      type="Build"
      onSuccess={() => mutateAsync({ name, config: {} })}
      enabled={!!name}
    >
      <div className="grid md:grid-cols-2">
        Build Name
        <Input
          placeholder="build-name"
          value={name}
          onChange={(e) => setName(e.target.value)}
        />
      </div>
    </NewResource>
  );
};

export const BuildConfig = ({ id }: { id: string }) => {
  const config = useRead("GetBuild", { id }).data?.config;
  const [update, set] = useState<Partial<Types.BuildConfig>>({});
  const { mutate } = useWrite("UpdateBuild");

  if (!config) return null;

  return (
    <ConfigInner
      config={config}
      update={update}
      set={set}
      onSave={() => mutate({ id, config: update })}
      components={{
        general: {
          general: {
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
          },
        },
        docker: {
          docker: {
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
            use_buildx: true,
            // docker_organization,
          },
        },
      }}
    />
  );
};

export const Build: RequiredResourceComponents = {
  Name: ({ id }) => <>{useBuild(id)?.name}</>,
  Description: ({ id }) => <>{fmt_verison(useBuild(id)?.info.version)}</>,
  Info: ({ id }) => {
    const ts = useBuild(id)?.info.last_built_at;
    return (
      <div className="flex items-center gap-2">
        <History className="w-4 h-4" />
        {ts ? new Date(ts).toLocaleString() : "Never Built"}
      </div>
    );
  },
  Icon: () => <Hammer className="w-4 h-4" />,
  Page: {
    Config: ({ id }) => <BuildConfig id={id} />,
  },
  Actions: () => null,
  New: () => <NewBuild />,
};
