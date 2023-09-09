/* eslint-disable @typescript-eslint/no-explicit-any */
import { ConfigItem, ResourceSelector } from "@components/config/util";
import { useRead } from "@lib/hooks";
import { fmt_verison } from "@lib/utils";
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
  buildId,
  selected,
  onSelect,
}: {
  buildId: string | undefined;
  selected: string | undefined;
  onSelect: (version: string) => void;
}) => {
  const versions = useRead(
    "GetBuildVersions",
    { id: buildId! },
    { enabled: !!buildId }
  ).data;
  return (
    <Select value={selected || undefined} onValueChange={onSelect}>
      <SelectTrigger className="w-full lg:w-[150px]">
        <SelectValue placeholder="Select Version" />
      </SelectTrigger>
      <SelectContent>
        <SelectItem value={JSON.stringify({ major: 0, minor: 0, patch: 0 })}>
          latest
        </SelectItem>
        {versions?.map((v) => (
          <SelectItem key={JSON.stringify(v)} value={JSON.stringify(v)}>
            {fmt_verison(v.version)}
          </SelectItem>
        ))}
      </SelectContent>
    </Select>
  );
};

const ImageTypeSelector = ({
  selected,
  onSelect,
}: {
  selected: Types.DeploymentImage["type"] | undefined;
  onSelect: (type: Types.DeploymentImage["type"]) => void;
}) => (
  <Select value={selected || undefined} onValueChange={onSelect}>
    <SelectTrigger className="max-w-[150px]">
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
}: {
  image: Types.DeploymentImage | undefined;
  set: (input: Partial<Types.DeploymentConfig>) => void;
}) => (
  <ConfigItem label="Image">
    <div className="flex gap-4 w-full justify-end">
      <ImageTypeSelector
        selected={image?.type}
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
        />
      )}
    </div>
  </ConfigItem>
);
