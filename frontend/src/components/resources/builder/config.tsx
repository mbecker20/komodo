import { Config } from "@components/config";
import { InputList } from "@components/config/util";
import { useRead, useWrite } from "@lib/hooks";
import { Types } from "@komodo/client";
import { useState } from "react";
import { ResourceSelector } from "../common";
import { Button } from "@ui/button";
import { MinusCircle, PlusCircle } from "lucide-react";
import {
  Dialog,
  DialogContent,
  DialogFooter,
  DialogHeader,
  DialogTrigger,
} from "@ui/dialog";
import { Card } from "@ui/card";
import { cn } from "@lib/utils";
import { Input } from "@ui/input";

export const BuilderConfig = ({ id }: { id: string }) => {
  const config = useRead("GetBuilder", { builder: id }).data?.config;
  if (config?.type === "Aws") return <AwsBuilderConfig id={id} />;
  if (config?.type === "Server") return <ServerBuilderConfig id={id} />;
};

const AwsBuilderConfig = ({ id }: { id: string }) => {
  const perms = useRead("GetPermissionLevel", {
    target: { type: "Builder", id },
  }).data;
  const config = useRead("GetBuilder", { builder: id }).data?.config
    ?.params as Types.AwsBuilderConfig;
  const global_disabled =
    useRead("GetCoreInfo", {}).data?.ui_write_disabled ?? false;
  const [update, set] = useState<Partial<Types.AwsBuilderConfig>>({});
  const { mutateAsync } = useWrite("UpdateBuilder");
  if (!config) return null;

  const disabled = global_disabled || perms !== Types.PermissionLevel.Write;

  return (
    <Config
      resource_id={id}
      resource_type="Builder"
      disabled={disabled}
      config={config}
      update={update}
      set={set}
      onSave={async () => {
        await mutateAsync({ id, config: { type: "Aws", params: update } });
      }}
      components={{
        general: [
          {
            label: "General",
            components: {
              region: true,
              instance_type: true,
              volume_gb: true,
              ami_id: true,
              subnet_id: true,
              key_pair_name: true,
              assign_public_ip: true,
              use_public_ip: true,
              port: true,
            },
          },
          {
            label: "Security Group Ids",
            contentHidden:
              (update.security_group_ids ?? config.security_group_ids)
                ?.length === 0,
            actions: !disabled && (
              <Button
                variant="secondary"
                onClick={() =>
                  set((update) => ({
                    ...update,
                    security_group_ids: [
                      ...(update.security_group_ids ??
                        config.security_group_ids ??
                        []),
                      "",
                    ],
                  }))
                }
                className="flex items-center gap-2 w-[200px]"
              >
                <PlusCircle className="w-4 h-4" />
                Add Security Group Id
              </Button>
            ),
            components: {
              security_group_ids: (values, set) => (
                <InputList
                  field="security_group_ids"
                  values={values}
                  set={set}
                  disabled={disabled}
                  placeholder="Security Group Id"
                />
              ),
            },
          },
          {
            label: "Additional Git Providers",
            contentHidden:
              (update.git_providers ?? config.git_providers)?.length === 0,
            actions: !disabled && (
              <Button
                variant="secondary"
                onClick={() =>
                  set((update) => ({
                    ...update,
                    git_providers: [
                      ...(update.git_providers ?? config.git_providers ?? []),
                      { domain: "github.com", https: true, accounts: [] },
                    ],
                  }))
                }
                className="flex items-center gap-2 w-[200px]"
              >
                <PlusCircle className="w-4 h-4" />
                Add Git Provider
              </Button>
            ),
            components: {
              git_providers: (providers, set) =>
                providers && (
                  <ProvidersConfig
                    type="git"
                    providers={providers}
                    set={set}
                    disabled={disabled}
                  />
                ),
            },
          },
          {
            label: "Additional Docker Registries",
            contentHidden:
              (update.docker_registries ?? config.docker_registries)?.length ===
              0,
            actions: !disabled && (
              <Button
                variant="secondary"
                onClick={() =>
                  set((update) => ({
                    ...update,
                    docker_registries: [
                      ...(update.docker_registries ??
                        config.docker_registries ??
                        []),
                      { domain: "docker.io", accounts: [], organizations: [] },
                    ],
                  }))
                }
                className="flex items-center gap-2 w-[200px]"
              >
                <PlusCircle className="w-4 h-4" />
                Add Docker Registry
              </Button>
            ),
            components: {
              docker_registries: (providers, set) =>
                providers && (
                  <ProvidersConfig
                    type="docker"
                    providers={providers}
                    set={set}
                    disabled={disabled}
                  />
                ),
            },
          },
          {
            label: "Additional Secret Keys",
            contentHidden: (update.secrets ?? config.secrets)?.length === 0,
            actions: !disabled && (
              <Button
                variant="secondary"
                onClick={() =>
                  set((update) => ({
                    ...update,
                    secrets: [...(update.secrets ?? config.secrets ?? []), ""],
                  }))
                }
                className="flex items-center gap-2 w-[200px]"
              >
                <PlusCircle className="w-4 h-4" />
                Add Secret Key
              </Button>
            ),
            components: {
              secrets: (secrets, set) => (
                <InputList
                  field="secrets"
                  values={secrets ?? []}
                  set={set}
                  disabled={disabled}
                  placeholder="SECRET_KEY"
                />
              ),
            },
          },
        ],
      }}
    />
  );
};

const ServerBuilderConfig = ({ id }: { id: string }) => {
  const perms = useRead("GetPermissionLevel", {
    target: { type: "Builder", id },
  }).data;
  const config = useRead("GetBuilder", { builder: id }).data?.config;
  const [update, set] = useState<Partial<Types.ServerBuilderConfig>>({});
  const { mutateAsync } = useWrite("UpdateBuilder");
  if (!config) return null;

  const disabled = perms !== Types.PermissionLevel.Write;

  return (
    <Config
      resource_id={id}
      resource_type="Builder"
      disabled={disabled}
      config={config.params as Types.ServerBuilderConfig}
      update={update}
      set={set}
      onSave={async () => {
        await mutateAsync({ id, config: { type: "Server", params: update } });
      }}
      components={{
        general: [
          {
            label: "General",
            components: {
              server_id: (id, set) => (
                <div className="flex justify-between items-center border-b pb-4">
                  Select Server
                  <ResourceSelector
                    type="Server"
                    selected={id}
                    onSelect={(server_id) => set({ server_id })}
                    disabled={disabled}
                  />
                </div>
              ),
            },
          },
        ],
      }}
    />
  );
};

const ProvidersConfig = (params: {
  type: "git" | "docker";
  providers: Types.GitProvider[] | Types.DockerRegistry[];
  set: (input: Partial<Types.AwsBuilderConfig>) => void;
  disabled: boolean;
}) => {
  const arr_field =
    params.type === "git" ? "git_providers" : "docker_registries";
  return (
    <div className="w-full flex justify-end">
      <div className="flex flex-col gap-4 w-full max-w-[400px]">
        {params.providers?.map((_, index) => (
          <div key={index} className="flex items-center justify-between gap-4">
            <ProviderDialog {...params} index={index} />
            {!params.disabled && (
              <Button
                variant="secondary"
                onClick={() =>
                  params.set({
                    [arr_field]: params.providers.filter((_, i) => i !== index),
                  })
                }
              >
                <MinusCircle className="w-4 h-4" />
              </Button>
            )}
          </div>
        ))}
      </div>
    </div>
  );
};

const ProviderDialog = ({
  type,
  providers,
  set,
  disabled,
  index,
}: {
  type: "git" | "docker";
  providers: Types.GitProvider[] | Types.DockerRegistry[];
  index: number;
  set: (input: Partial<Types.AwsBuilderConfig>) => void;
  disabled: boolean;
}) => {
  const [open, setOpen] = useState(false);
  const provider = providers[index];
  const arr_field = type === "git" ? "git_providers" : "docker_registries";
  const example_domain = type === "git" ? "github.com" : "docker.io";
  const update_domain = (domain: string) =>
    set({
      [arr_field]: providers.map((provider, i) =>
        i === index ? { ...provider, domain } : provider
      ),
    });
  const add_account = () =>
    set({
      [arr_field]: providers.map(
        (provider: Types.GitProvider | Types.DockerRegistry, i) =>
          i === index
            ? {
                ...provider,
                accounts: [...(provider.accounts ?? []), { username: "" }],
              }
            : provider
      ) as Types.GitProvider[] | Types.DockerRegistry[],
    });
  const update_username = (username: string, account_index: number) =>
    set({
      [arr_field]: providers.map(
        (provider: Types.GitProvider | Types.DockerRegistry, provider_index) =>
          provider_index === index
            ? {
                ...provider,
                accounts: provider.accounts?.map((account, i) =>
                  account_index === i ? { username } : account
                ),
              }
            : provider
      ) as Types.GitProvider[] | Types.DockerRegistry[],
    });
  const remove_account = (account_index) =>
    set({
      [arr_field]: providers.map(
        (provider: Types.GitProvider | Types.DockerRegistry, provider_index) =>
          provider_index === index
            ? {
                ...provider,
                accounts: provider.accounts?.filter(
                  (_, i) => account_index !== i
                ),
              }
            : provider
      ) as Types.GitProvider[] | Types.DockerRegistry[],
    });
  const add_organization = () =>
    set({
      [arr_field]: providers.map((provider: Types.DockerRegistry, i) =>
        i === index
          ? {
              ...provider,
              organizations: [...(provider.organizations ?? []), ""],
            }
          : provider
      ) as Types.DockerRegistry[],
    });
  const update_organization = (name: string, organization_index: number) =>
    set({
      [arr_field]: providers.map(
        (provider: Types.DockerRegistry, provider_index) =>
          provider_index === index
            ? {
                ...provider,
                organizations: provider.organizations?.map((organization, i) =>
                  organization_index === i ? name : organization
                ),
              }
            : provider
      ) as Types.GitProvider[] | Types.DockerRegistry[],
    });
  const remove_organization = (organization_index) =>
    set({
      [arr_field]: providers.map(
        (provider: Types.DockerRegistry, provider_index) =>
          provider_index === index
            ? {
                ...provider,
                organizations: provider.organizations?.filter(
                  (_, i) => organization_index !== i
                ),
              }
            : provider
      ) as Types.DockerRegistry[],
    });
  return (
    <Dialog open={open} onOpenChange={setOpen}>
      <DialogTrigger asChild>
        <Card className="px-3 py-2 hover:bg-accent/50 transition-colors cursor-pointer w-full">
          <div
            className={cn(
              "flex gap-2 text-sm text-nowrap overflow-hidden overflow-ellipsis"
            )}
          >
            <div className="flex gap-2">{provider.domain}</div>
            <div className="flex gap-2">
              <div className="text-muted-foreground">accounts:</div>{" "}
              {provider.accounts?.length || 0}
            </div>
            {(provider as Types.DockerRegistry).organizations !== undefined && (
              <div className="flex gap-2">
                <div className="text-muted-foreground">organizations:</div>{" "}
                {(provider as Types.DockerRegistry).organizations?.length || 0}
              </div>
            )}
          </div>
        </Card>
      </DialogTrigger>
      <DialogContent>
        <DialogHeader>
          {type === "git" ? "Git Provider" : "Docker Registry"}
        </DialogHeader>
        <div className="flex flex-col gap-4">
          {/* Domain */}
          <div className="flex items-center justify-between w-fill">
            <div className="text-nowrap">Domain</div>
            <Input
              value={provider.domain}
              onChange={(e) => update_domain(e.target.value)}
              disabled={disabled}
              className="w-[300px]"
              placeholder={example_domain}
            />
          </div>

          {/* Accounts */}
          <div className="flex flex-col gap-2 w-fill">
            <div className="flex items-center justify-between w-fill">
              <div className="text-nowrap">Available Accounts</div>
              <Button variant="secondary" onClick={add_account}>
                Add
              </Button>
            </div>
            <div className="flex flex-col gap-2">
              {provider.accounts?.map((account, account_index) => {
                return (
                  <div
                    key={account_index}
                    className="flex gap-2 items-center justify-end"
                  >
                    <Input
                      placeholder="Account Username"
                      value={account.username}
                      onChange={(e) =>
                        update_username(e.target.value, account_index)
                      }
                    />
                    {!disabled && (
                      <Button
                        variant="secondary"
                        onClick={() => remove_account(account_index)}
                      >
                        <MinusCircle className="w-4 h-4" />
                      </Button>
                    )}
                  </div>
                );
              })}
            </div>
          </div>

          {/* Organizations */}
          {type === "docker" && (
            <div className="flex flex-col gap-2 w-fill">
              <div className="flex items-center justify-between w-fill">
                <div className="text-nowrap">Available Organizations</div>
                <Button variant="secondary" onClick={add_organization}>
                  Add
                </Button>
              </div>
              <div className="flex flex-col gap-2">
                {(provider as Types.DockerRegistry).organizations?.map(
                  (organization, organization_index) => {
                    return (
                      <div
                        key={organization_index}
                        className="flex gap-2 items-center justify-end"
                      >
                        <Input
                          value={organization}
                          onChange={(e) =>
                            update_organization(
                              e.target.value,
                              organization_index
                            )
                          }
                          placeholder="Organization Name"
                        />
                        {!disabled && (
                          <Button
                            variant="secondary"
                            onClick={() =>
                              remove_organization(organization_index)
                            }
                          >
                            <MinusCircle className="w-4 h-4" />
                          </Button>
                        )}
                      </div>
                    );
                  }
                )}
              </div>
            </div>
          )}
        </div>
        <DialogFooter>
          <Button onClick={() => setOpen(false)}>Confirm</Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
};
