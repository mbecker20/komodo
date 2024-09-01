import { Config } from "@components/config";
import { InputList } from "@components/config/util";
import { TextUpdateMenu } from "@components/util";
import { useRead, useWrite } from "@lib/hooks";
import { cn } from "@lib/utils";
import { Types } from "@komodo/client";
import { Button } from "@ui/button";
import { Card } from "@ui/card";
import {
  Dialog,
  DialogContent,
  DialogFooter,
  DialogHeader,
  DialogTrigger,
} from "@ui/dialog";
import { Input } from "@ui/input";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@ui/select";
import { MinusCircle, PlusCircle } from "lucide-react";
import { useState } from "react";

export const AwsServerTemplateConfig = ({
  id,
  disabled,
}: {
  id: string;
  disabled: boolean;
}) => {
  const config = useRead("GetServerTemplate", { server_template: id }).data
    ?.config?.params as Types.AwsServerTemplateConfig;
  const [update, set] = useState<Partial<Types.AwsServerTemplateConfig>>({});
  const { mutateAsync } = useWrite("UpdateServerTemplate");
  if (!config) return null;

  return (
    <Config
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
              ami_id: true,
              subnet_id: true,
              key_pair_name: true,
              port: true,
              assign_public_ip: true,
              use_public_ip: true,
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
                  values={values ?? []}
                  set={set}
                  disabled={disabled}
                  placeholder="Security Group Id"
                />
              ),
            },
          },
          {
            label: "Volumes",
            contentHidden: (update.volumes ?? config.volumes)?.length === 0,
            actions: !disabled && (
              <Button
                variant="secondary"
                onClick={() =>
                  set((update) => ({
                    ...update,
                    volumes: [
                      ...(update.volumes ?? config.volumes ?? []),
                      newVolume((update.volumes ?? config.volumes).length),
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
                      {volumes.map((_, index) => (
                        <div
                          key={index}
                          className="flex items-center justify-between gap-4"
                        >
                          <AwsVolumeDialog
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
            contentHidden: true,
            actions: (
              <TextUpdateMenu
                title="Update User Data"
                placeholder="Set User Data"
                value={update.user_data ?? config.user_data}
                onUpdate={(user_data) => set({ ...update, user_data })}
                triggerClassName="min-w-[300px] max-w-[400px]"
                disabled={disabled}
              />
            ),
            components: {},
          },
        ],
      }}
    />
  );
};

const AwsVolumeDialog = ({
  volumes,
  index,
  set,
  disabled,
}: {
  volumes: Types.AwsVolume[];
  index: number;
  set: (value: Partial<Types.AwsServerTemplateConfig>) => void;
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
              <div className="text-muted-foreground">device:</div>{" "}
              {volume.device_name}
            </div>
            <div className="flex gap-2">
              <div className="text-muted-foreground">size:</div>{" "}
              {volume.size_gb} GB
            </div>
            <div className="flex gap-2">
              <div className="text-muted-foreground">type:</div>{" "}
              {volume.volume_type}
            </div>
          </div>
        </Card>
      </DialogTrigger>
      <DialogContent>
        <DialogHeader>AWS Volume</DialogHeader>
        <div className="flex flex-col gap-4">
          {/* Device Name */}
          <div className="flex items-center justify-between w-fill">
            <div className="text-nowrap">Device Name</div>
            <Input
              value={volume.device_name}
              onChange={(e) =>
                set({
                  volumes: volumes.map((volume, i) =>
                    i === index
                      ? { ...volume, device_name: e.target.value }
                      : volume
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
            <div className="text-nowrap">Volume Type</div>
            <Select
              value={volume.volume_type}
              onValueChange={(value) =>
                set({
                  volumes: volumes.map((volume, i) =>
                    i === index
                      ? { ...volume, volume_type: value as Types.AwsVolumeType }
                      : volume
                  ),
                })
              }
            >
              <SelectTrigger className="w-[300px]">
                <SelectValue />
              </SelectTrigger>
              <SelectContent>
                {Object.entries(Types.AwsVolumeType).map(([name, item]) => (
                  <SelectItem key={name} value={item}>
                    {name}
                  </SelectItem>
                ))}
              </SelectContent>
            </Select>
          </div>

          {/* IOPS */}
          <div className="flex items-center justify-between w-fill">
            <div className="text-nowrap">Iops</div>
            <Input
              type="number"
              placeholder="Set Custom Iops"
              value={volume.iops ? volume.iops : undefined}
              onChange={(e) =>
                set({
                  volumes: volumes.map((volume, i) =>
                    i === index
                      ? { ...volume, iops: Number(e.target.value || 0) }
                      : volume
                  ),
                })
              }
              disabled={disabled}
              className="w-[300px]"
            />
          </div>

          {/* Throughput */}
          <div className="flex items-center justify-between w-fill">
            <div className="text-nowrap">Throughput</div>
            <Input
              type="number"
              placeholder="Set Custom Throughput"
              value={volume.throughput ? volume.throughput : undefined}
              onChange={(e) =>
                set({
                  volumes: volumes.map((volume, i) =>
                    i === index
                      ? { ...volume, throughput: Number(e.target.value || 0) }
                      : volume
                  ),
                })
              }
              disabled={disabled}
              className="w-[300px]"
            />
          </div>
        </div>
        <DialogFooter>
          <Button onClick={() => setOpen(false)}>Confirm</Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
};

const device_letters = ["b", "c", "d", "e", "f"];

const newVolume: (index: number) => Types.AwsVolume = (index) => {
  const device_name =
    index === 0 ? "/dev/sda1" : `/dev/sd${device_letters[index - 1]}`;
  return {
    device_name,
    size_gb: 20,
    volume_type: Types.AwsVolumeType.Gp2,
    iops: 0,
    throughput: 0,
  };
};
