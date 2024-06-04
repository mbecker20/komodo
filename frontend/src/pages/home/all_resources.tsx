import { OpenAlerts } from "@components/alert";
import { ExportButton } from "@components/export";
import { Page, Section } from "@components/layouts";
import { ResourceComponents } from "@components/resources";
import { TagsFilter } from "@components/tags";
import { useFilterResources, useRead, useTagsFilter } from "@lib/hooks";
import { Types } from "@monitor/client";
import { RequiredResourceComponents, UsableResource } from "@types";
import { Input } from "@ui/input";
import { useState } from "react";

export const AllResources = () => {
  const [search, setSearch] = useState("");
  const tags = useTagsFilter();
  return (
    <Page
      titleOther={
        <div className="flex items-center justify-between">
          <div className="flex gap-4 items-center">
            <Input
              value={search}
              onChange={(e) => setSearch(e.target.value)}
              placeholder="search..."
              className="w-[200px] lg:w-[300px]"
            />
            <ExportButton tags={tags} />
          </div>
          <TagsFilter />
        </div>
      }
    >
      <OpenAlerts />
      {Object.entries(ResourceComponents).map(([type, Components]) => (
        <TableSection
          key={type}
          type={type}
          Components={Components}
          search={search}
        />
      ))}
    </Page>
  );
};

const TableSection = ({
  type,
  Components,
  search,
}: {
  type: string;
  Components: RequiredResourceComponents;
  search?: string;
}) => {
  const resources = useRead(`List${type as UsableResource}s`, {}).data;

  const filtered = useFilterResources(
    resources as Types.ResourceListItem<unknown>[],
    search
  );

  let count = filtered.length;

  if (!count) return;

  return (
    <Section key={type} title={type + "s"}>
      <Components.Table resources={resources ?? []} />
    </Section>
  );
};
