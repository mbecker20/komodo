import { ExportButton } from "@components/export";
import { Page, Section } from "@components/layouts";
import { ResourceComponents } from "@components/resources";
// import { TagsFilter } from "@components/tags";
import { ShowHideButton } from "@components/util";
import {
  useFilterResources,
  useNoResources,
  useRead,
  useTagsFilter,
  useUser,
} from "@lib/hooks";
import { cn } from "@lib/utils";

import { RequiredResourceComponents, UsableResource } from "@types";
import { Input } from "@ui/input";
import { AlertTriangle } from "lucide-react";
import { useState } from "react";
import { TagSelector } from "@components/tags/tags-2";
import { ResourceListItemTable } from "@components/resource-list-item-table";

export const AllResources = () => {
  const [search, setSearch] = useState("");
  const tags = useTagsFilter();
  const noResources = useNoResources();
  const user = useUser().data!;
  return (
    <Page
      titleOther={
        <div className="flex items-center justify-between">
          <Input
            value={search}
            onChange={(e) => setSearch(e.target.value)}
            placeholder="search..."
            className="w-[200px] lg:w-[300px]"
          />

          <div className="flex items-center gap-2">
            {/* <TagsFilter /> */}
            <TagSelector />
            <ExportButton tags={tags} />
          </div>
        </div>
      }
    >
      {noResources && (
        <div className="flex items-center gap-4 px-2 text-muted-foreground">
          <AlertTriangle className="w-4 h-4" />
          <p className="text-lg">
            No resources found.{" "}
            {user.admin
              ? "To get started, create a server."
              : "Contact an admin for access to resources."}
          </p>
        </div>
      )}
      <div className="flex flex-col gap-6">
        {Object.entries(ResourceComponents).map(([type, Components]) => (
          <TableSection
            key={type}
            type={type as UsableResource}
            Components={Components}
            search={search}
          />
        ))}
      </div>
    </Page>
  );
};

const TableSection = ({
  type,
  Components,
  search,
}: {
  type: UsableResource;
  Components: RequiredResourceComponents;
  search?: string;
}) => {
  const resources = useRead(`List${type}s`, {}).data;
  const [show, setShow] = useState(true);

  const filtered = useFilterResources(resources as any, search);
  if (!filtered.length) return;

  return (
    <Section
      key={type}
      title={type + "s"}
      icon={<Components.Icon />}
      actions={<ShowHideButton show={show} setShow={setShow} />}
    >
      <div className={cn("border-b", show && "pb-8")}>
        {/* {show && <Components.Table resources={filtered ?? []} />} */}
        {show && (
          <ResourceListItemTable
            type={type as UsableResource}
            data={filtered as any}
          />
        )}
      </div>
    </Section>
  );
};
