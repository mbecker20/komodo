import { Config } from "@components/config";
import { ConfigItem, InputList } from "@components/config/util";
import { TextUpdateMenu } from "@components/util";
import { useRead, useWrite } from "@lib/hooks";
import { cn } from "@lib/utils";
import { Types } from "@monitor/client";
import { Button } from "@ui/button";
import { Card } from "@ui/card";
import {
  Command,
  CommandEmpty,
  CommandGroup,
  CommandInput,
  CommandItem,
  CommandList,
} from "@ui/command";
import {
  Dialog,
  DialogContent,
  DialogFooter,
  DialogHeader,
  DialogTrigger,
} from "@ui/dialog";
import { Input } from "@ui/input";
import { Popover, PopoverContent, PopoverTrigger } from "@ui/popover";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@ui/select";
import { ChevronsUpDown, MinusCircle, PlusCircle, SearchX } from "lucide-react";
import { useState } from "react";

export const HetznerServerTemplateConfig = ({ id }: { id: string }) => {
  const perms = useRead("GetPermissionLevel", {
    target: { type: "ServerTemplate", id },
  }).data;
  const config = useRead("GetServerTemplate", { server_template: id }).data
    ?.config.params as Types.HetznerServerTemplateConfig;
  const [update, set] = useState<Partial<Types.HetznerServerTemplateConfig>>(
    {}
  );
  const { mutateAsync } = useWrite("UpdateServerTemplate");
  if (!config) return null;

  const disabled = perms !== Types.PermissionLevel.Write;

  return (
    <Config
      disabled={disabled}
      config={config}
      update={update}
      set={set}
      onSave={async () => {
        await mutateAsync({ id, config: { type: "Hetzner", params: update } });
      }}
      components={{
        general: [
          {
            label: "General",
            components: {
              datacenter: (datacenter, set) => (
                <DatacenterSelector
                  selected={datacenter}
                  set={set}
                  disabled={disabled}
                />
              ),
              server_type: (server_type, set) => (
                <ServerTypeSelector
                  datacenter={
                    update.datacenter ??
                    config.datacenter ??
                    Types.HetznerDatacenter.Nuremberg1Dc3
                  }
                  selected={server_type}
                  set={set}
                  disabled={disabled}
                />
              ),
              image: true,
              port: true,
              enable_public_ipv4: true,
              enable_public_ipv6: true,
              use_public_ip:
                update.enable_public_ipv4 ?? config.enable_public_ipv4,
            },
          },
          {
            label: "Private Network Ids",
            contentHidden:
              (update.private_network_ids ?? config.private_network_ids)
                ?.length === 0,
            actions: !disabled && (
              <Button
                variant="secondary"
                onClick={() =>
                  set((update) => ({
                    ...update,
                    private_network_ids: [
                      ...(update.private_network_ids ??
                        config.private_network_ids ??
                        []),
                      0,
                    ],
                  }))
                }
                className="flex items-center gap-2 w-[200px]"
              >
                <PlusCircle className="w-4 h-4" />
                Add Network Id
              </Button>
            ),
            components: {
              private_network_ids: (values, set) => (
                <InputList
                  field="private_network_ids"
                  values={values?.map((val) => val.toString()) ?? []}
                  set={({ private_network_ids }) =>
                    set({
                      private_network_ids: (
                        private_network_ids as string[]
                      ).map((id) => Number(id)),
                    })
                  }
                  disabled={disabled}
                  placeholder="Private Network Id"
                />
              ),
            },
          },
          {
            label: "Firewall Ids",
            contentHidden:
              (update.firewall_ids ?? config.firewall_ids)?.length === 0,
            actions: !disabled && (
              <Button
                variant="secondary"
                onClick={() =>
                  set((update) => ({
                    ...update,
                    firewall_ids: [
                      ...(update.firewall_ids ?? config.firewall_ids ?? []),
                      0,
                    ],
                  }))
                }
                className="flex items-center gap-2 w-[200px]"
              >
                <PlusCircle className="w-4 h-4" />
                Add Firewall Id
              </Button>
            ),
            components: {
              firewall_ids: (values, set) => (
                <InputList
                  field="firewall_ids"
                  values={values?.map((val) => val.toString()) ?? []}
                  set={({ firewall_ids }) =>
                    set({
                      firewall_ids: (firewall_ids as string[]).map((id) =>
                        Number(id)
                      ),
                    })
                  }
                  disabled={disabled}
                  placeholder="Firewall Id"
                />
              ),
            },
          },
          {
            label: "Volumes",
            contentHidden:
              ((update.volumes ?? config.volumes)?.length ?? 0) === 0,
            actions: !disabled && (
              <Button
                variant="secondary"
                onClick={() =>
                  set((update) => ({
                    ...update,
                    volumes: [
                      ...(update.volumes ?? config.volumes ?? []),
                      newVolume(
                        (update.volumes ?? config.volumes)?.length ?? 0
                      ),
                    ],
                  }))
                }
                className="flex items-center gap-2 w-[200px]"
              >
                <PlusCircle className="w-4 h-4" />
                Add Volume
              </Button>
            ),
            components: {
              volumes: (volumes, set) => {
                return (
                  <div className="w-full flex justify-end">
                    <div className="flex flex-col gap-4 w-full max-w-[400px]">
                      {volumes?.map((_, index) => (
                        <div
                          key={index}
                          className="flex items-center justify-between gap-4"
                        >
                          <HetznerVolumeDialog
                            volumes={volumes}
                            index={index}
                            set={set}
                            disabled={disabled}
                          />
                          {!disabled && (
                            <Button
                              variant="secondary"
                              disabled={disabled}
                              onClick={() =>
                                set({
                                  volumes: volumes.filter(
                                    (_, i) => i !== index
                                  ),
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
              },
            },
          },
          {
            label: "User Data",
            labelHidden: true,
            components: {
              user_data: (user_data, set) => (
                <ConfigItem label="User Data">
                  <TextUpdateMenu
                    title="Update User Data"
                    placeholder="Set User Data"
                    value={user_data}
                    onUpdate={(user_data) => set({ user_data })}
                    triggerClassName="min-w-[300px] max-w-[400px]"
                    disabled={disabled}
                  />
                </ConfigItem>
              ),
            },
          },
          {
            label: "SSH Keys",
            contentHidden: (update.ssh_keys ?? config.ssh_keys)?.length === 0,
            actions: !disabled && (
              <Button
                variant="secondary"
                onClick={() =>
                  set((update) => ({
                    ...update,
                    ssh_keys: [
                      ...(update.ssh_keys ?? config.ssh_keys ?? []),
                      "",
                    ],
                  }))
                }
                className="flex items-center gap-2 w-[200px]"
              >
                <PlusCircle className="w-4 h-4" />
                Add SSH Key
              </Button>
            ),
            components: {
              ssh_keys: (values, set) => (
                <InputList
                  field="ssh_keys"
                  values={values?.map((val) => val.toString()) ?? []}
                  set={set}
                  disabled={disabled}
                  placeholder="SSH Key ID or Name"
                />
              ),
            },
          },
        ],
      }}
    />
  );
};

const DatacenterSelector = ({
  disabled,
  selected,
  set,
}: {
  disabled: boolean;
  selected: Types.HetznerDatacenter | undefined;
  set: (value: Partial<Types.HetznerServerTemplateConfig>) => void;
}) => {
  return (
    <ConfigItem label="Datacenter">
      <Select
        value={selected}
        onValueChange={(value) =>
          set({ datacenter: value as Types.HetznerDatacenter })
        }
        disabled={disabled}
      >
        <SelectTrigger
          className="w-full lg:w-[200px] max-w-[50%]"
          disabled={disabled}
        >
          <SelectValue />
        </SelectTrigger>
        <SelectContent>
          {Object.values(Types.HetznerDatacenter).map((datacenter) => (
            <SelectItem key={datacenter} value={datacenter}>
              {datacenter}
            </SelectItem>
          ))}
        </SelectContent>
      </Select>
    </ConfigItem>
  );
};

const ServerTypeSelector = ({
  datacenter,
  disabled,
  selected,
  set,
}: {
  datacenter: Types.HetznerDatacenter;
  disabled: boolean;
  selected: Types.HetznerServerType | undefined;
  set: (value: Partial<Types.HetznerServerTemplateConfig>) => void;
}) => {
  const [open, setOpen] = useState(false);
  const [input, setInput] = useState("");
  // The US based datacenters only have Amd servers
  const filter =
    datacenter === Types.HetznerDatacenter.HillsboroDc1 ||
    datacenter === Types.HetznerDatacenter.AshburnDc1
      ? (st: string) => st.includes("Amd")
      : () => true;
  const server_types = Object.values(Types.HetznerServerType).filter(filter);
  return (
    <ConfigItem label="Server Type">
      <Popover open={open} onOpenChange={setOpen}>
        <PopoverTrigger asChild>
          <Button
            variant="secondary"
            className="flex gap-2"
            disabled={disabled}
          >
            {selected ?? "Select Server Type"}
            {!disabled && <ChevronsUpDown className="w-3 h-3" />}
          </Button>
        </PopoverTrigger>
        <PopoverContent className="w-[300px] max-h-[200px] p-0" align="end">
          <Command>
            <CommandInput
              placeholder="Search Server Types"
              className="h-9"
              value={input}
              onValueChange={setInput}
            />
            <CommandList>
              <CommandEmpty className="flex justify-evenly items-center">
                No Server Types Found
                <SearchX className="w-3 h-3" />
              </CommandEmpty>

              <CommandGroup>
                {server_types.map((server_type) => (
                  <CommandItem
                    key={server_type}
                    onSelect={() => {
                      set({ server_type });
                      setOpen(false);
                    }}
                    className="flex items-center justify-between cursor-pointer"
                  >
                    <div className="p-1">{server_type}</div>
                  </CommandItem>
                ))}
              </CommandGroup>
            </CommandList>
          </Command>
        </PopoverContent>
      </Popover>
    </ConfigItem>
  );
};

const HetznerVolumeDialog = ({
  volumes,
  index,
  set,
  disabled,
}: {
  volumes: Types.HetznerVolumeSpecs[];
  index: number;
  set: (value: Partial<Types.HetznerServerTemplateConfig>) => void;
  disabled?: boolean;
}) => {
  const volume = volumes[index];
  const [open, setOpen] = useState(false);
  return (
    <Dialog open={open} onOpenChange={setOpen}>
      <DialogTrigger asChild>
        <Card className="px-3 py-2 hover:bg-accent/50 transition-colors cursor-pointer w-full">
          <div
            className={cn(
              "flex gap-2 text-sm text-nowrap overflow-hidden overflow-ellipsis"
            )}
          >
            <div className="flex gap-2">
              <div className="text-muted-foreground">device:</div> {volume.name}
            </div>
            <div className="flex gap-2">
              <div className="text-muted-foreground">size:</div>{" "}
              {volume.size_gb} GB
            </div>
            <div className="flex gap-2">
              <div className="text-muted-foreground">type:</div> {volume.format}
            </div>
          </div>
        </Card>
      </DialogTrigger>
      <DialogContent>
        <DialogHeader>Hetzner Volume</DialogHeader>
        <div className="flex flex-col gap-4">
          {/* Volume Name */}
          <div className="flex items-center justify-between w-fill">
            <div className="text-nowrap">Volume Name</div>
            <Input
              value={volume.name}
              onChange={(e) =>
                set({
                  volumes: volumes.map((volume, i) =>
                    i === index ? { ...volume, name: e.target.value } : volume
                  ),
                })
              }
              disabled={disabled}
              className="w-[300px]"
            />
          </div>

          {/* Size GB */}
          <div className="flex items-center justify-between w-fill">
            <div className="text-nowrap">Size (GB)</div>
            <Input
              type="number"
              value={volume.size_gb}
              onChange={(e) =>
                set({
                  volumes: volumes.map((volume, i) =>
                    i === index
                      ? { ...volume, size_gb: Number(e.target.value || 0) }
                      : volume
                  ),
                })
              }
              disabled={disabled}
              className="w-[300px]"
            />
          </div>

          {/* Volume Type */}
          <div className="flex items-center justify-between w-fill">
            <div className="text-nowrap">Volume Format</div>
            <Select
              value={volume.format}
              onValueChange={(value) =>
                set({
                  volumes: volumes.map((volume, i) =>
                    i === index
                      ? {
                          ...volume,
                          format: value as Types.HetznerVolumeFormat,
                        }
                      : volume
                  ),
                })
              }
            >
              <SelectTrigger className="w-[300px]">
                <SelectValue />
              </SelectTrigger>
              <SelectContent>
                {Object.values(Types.HetznerVolumeFormat).map((format) => (
                  <SelectItem key={format} value={format}>
                    {format}
                  </SelectItem>
                ))}
              </SelectContent>
            </Select>
          </div>
        </div>
        <DialogFooter>
          <Button onClick={() => setOpen(false)}>Confirm</Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
};

const newVolume: (index: number) => Types.HetznerVolumeSpecs = (index) => {
  return {
    name: `volume-${index}`,
    size_gb: 20,
    format: Types.HetznerVolumeFormat.Xfs,
    labels: {},
  };
};
