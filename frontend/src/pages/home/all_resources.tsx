import { ExportButton } from "@components/export";
import { Page, Section } from "@components/layouts";
import { ResourceComponents } from "@components/resources";
import { TagsFilter } from "@components/tags";
import { useFilterResources, useRead, useTagsFilter } from "@lib/hooks";
import { cn } from "@lib/utils";
import { Types } from "@monitor/client";
import { RequiredResourceComponents, UsableResource } from "@types";
import { Button } from "@ui/button";
import { Input } from "@ui/input";
import { ChevronDown, ChevronUp } from "lucide-react";
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
            <TagsFilter />
          </div>
          <ExportButton tags={tags} />
        </div>
      }
    >
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

  const [show, setShow] = useState(true);

  if (!count) return;

  return (
    <Section
      key={type}
      title={type + "s"}
      icon={<Components.Icon />}
      actions={
        <Button
          size="sm"
          variant="outline"
          className="gap-4"
          onClick={() => setShow(!show)}
        >
          {show ? "Hide" : "Show"}
          {show ? (
            <ChevronUp className="w-4" />
          ) : (
            <ChevronDown className="w-4" />
          )}
        </Button>
      }
    >
      <div className={cn("border-b", show && "pb-16")}>
        {show && <Components.Table resources={filtered ?? []} />}
      </div>
    </Section>
  );
};
