import { ExportButton } from "@components/export";
import { Page } from "@components/layouts";
import { ResourceComponents } from "@components/resources";
import { TagsFilter } from "@components/tags";
import {
  useFilterResources,
  useRead,
  useResourceParamType,
  useSetTitle,
} from "@lib/hooks";
import { Types } from "@komodo/client";
import { Input } from "@ui/input";
import { useState } from "react";

export const Resources = () => {
  const type = useResourceParamType()!;
  const name =
    type === "ServerTemplate"
      ? "Server Template"
      : type === "ResourceSync"
      ? "Resource Sync"
      : type;
  useSetTitle(name + "s");
  const Components = ResourceComponents[type];
  const [search, set] = useState("");

  const resources = useRead(`List${type}s`, {}).data;

  const filtered = useFilterResources(resources as any, search);
  const targets = filtered?.map(
    (resource): Types.ResourceTarget => ({
      type,
      id: resource.id,
    })
  );

  return (
    <Page
      title={`${name}s`}
      subtitle={
        <div className="text-muted-foreground">
          <Components.Description />
        </div>
      }
      icon={<Components.BigIcon />}
      actions={
        <div className="flex items-center h-fit gap-4">
          <TagsFilter />
          <Components.New />
        </div>
      }
    >
      <div className="flex flex-col gap-4">
        <div className="flex items-center gap-4">
          <Input
            value={search}
            onChange={(e) => set(e.target.value)}
            placeholder="search..."
            className="w-[200px] lg:w-[300px]"
          />
          <ExportButton targets={targets} />
        </div>
        <Components.Table resources={filtered ?? []} />
      </div>
    </Page>
  );
};
