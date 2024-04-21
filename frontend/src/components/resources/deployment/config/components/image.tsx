/* eslint-disable @typescript-eslint/no-explicit-any */
import { ConfigItem } from "@components/config/util";
import { ResourceSelector } from "@components/resources/common";
import { fmt_version } from "@lib/formatting";
import { useRead } from "@lib/hooks";
import { Types } from "@monitor/client";
import { Input } from "@ui/input";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@ui/select";

const BuildVersionSelector = ({
  disabled,
  buildId,
  selected,
  onSelect,
}: {
  disabled: boolean;
  buildId: string | undefined;
  selected: string | undefined;
  onSelect: (version: string) => void;
}) => {
  const versions = useRead(
    "GetBuildVersions",
    { build: buildId! },
    { enabled: !!buildId }
  ).data;
  return (
    <Select
      value={selected || undefined}
      onValueChange={onSelect}
      disabled={disabled}
    >
      <SelectTrigger className="w-full lg:w-[150px]" disabled={disabled}>
        <SelectValue placeholder="Select Version" />
      </SelectTrigger>
      <SelectContent>
        <SelectItem value={JSON.stringify({ major: 0, minor: 0, patch: 0 })}>
          latest
        </SelectItem>
        {versions?.map((v) => (
          <SelectItem
            key={JSON.stringify(v.version) + v.ts}
            value={JSON.stringify(v.version)}
          >
            {fmt_version(v.version)}
          </SelectItem>
        ))}
      </SelectContent>
    </Select>
  );
};

const ImageTypeSelector = ({
  selected,
  onSelect,
  disabled,
}: {
  selected: Types.DeploymentImage["type"] | undefined;
  onSelect: (type: Types.DeploymentImage["type"]) => void;
  disabled: boolean;
}) => (
  <Select value={selected || undefined} onValueChange={onSelect} disabled={disabled}>
    <SelectTrigger className="max-w-[150px]" disabled={disabled}>
      <SelectValue placeholder="Select Type" />
    </SelectTrigger>
    <SelectContent>
      <SelectItem value={"Image"}>Image</SelectItem>
      <SelectItem value={"Build"}>Build</SelectItem>
    </SelectContent>
  </Select>
);

export const ImageConfig = ({
  image,
  set,
  disabled,
}: {
  image: Types.DeploymentImage | undefined;
  set: (input: Partial<Types.DeploymentConfig>) => void;
  disabled: boolean;
}) => (
  <ConfigItem label="Image">
    <div className="flex gap-4 w-full justify-end">
      <ImageTypeSelector
        selected={image?.type}
        disabled={disabled}
        onSelect={(type) =>
          set({
            image: {
              type: type,
              params:
                type === "Image"
                  ? { image: "" }
                  : ({
                      build_id: "",
                      version: { major: 0, minor: 0, patch: 0 },
                    } as any),
            },
          })
        }
      />
      {image?.type === "Build" && (
        <div className="flex gap-4">
          <ResourceSelector
            type="Build"
            selected={image.params.build_id}
            onSelect={(id) =>
              set({
                image: {
                  ...image,
                  params: { ...image.params, build_id: id },
                },
              })
            }
          />
          <BuildVersionSelector
            buildId={image.params.build_id}
            selected={JSON.stringify(image.params.version)}
            onSelect={(version) =>
              set({
                image: {
                  ...image,
                  params: {
                    ...image.params,
                    version: JSON.parse(version),
                  },
                },
              })
            }
            disabled={disabled}
          />
        </div>
      )}
      {image?.type === "Image" && (
        <Input
          value={image.params.image}
          onChange={(e) =>
            set({
              image: {
                ...image,
                params: { image: e.target.value },
              },
            })
          }
          className="w-full lg:w-[300px]"
          placeholder="image name"
          disabled={disabled}
        />
      )}
    </div>
  </ConfigItem>
);
