import { Page, Section } from "@components/layouts";
import { ResourceComponents } from "@components/resources";
import { TagsFilter, useTagsFilter } from "@components/tags";
import { useRead } from "@lib/hooks";
import { RequiredResourceComponents, UsableResource } from "@types";
import { Input } from "@ui/input";
import { useState } from "react";

export const AllResources = () => {
  const [search, setSearch] = useState("");
  return (
    <Page
      title="Resources"
      actions={
        <div className="grid gap-4 justify-items-end">
          <div className="flex gap-4">
            <Input
              value={search}
              onChange={(e) => setSearch(e.target.value)}
              placeholder="search..."
              className="w-96"
            />
          </div>
          <TagsFilter />
        </div>
      }
    >
      {Object.entries(ResourceComponents).map(([type, Components]) => (
        <TableSection type={type} Components={Components} search={search} />
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
  const tags = useTagsFilter();
  const count = useRead(`List${type as UsableResource}s`, {}).data?.filter(
    (resource) => tags.every((tag) => resource.tags.includes(tag))
  ).length;

  if (!count) return;

  return (
    <Section key={type} title={type + "s"} actions={<Components.New />}>
      <Components.Table search={search} />
    </Section>
  );
};
