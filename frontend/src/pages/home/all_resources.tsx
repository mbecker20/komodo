import { ExportButton } from "@components/export";
import { Page, Section } from "@components/layouts";
import { ResourceComponents } from "@components/resources";
import { ShowHideButton } from "@components/util";
import {
  useFilterResources,
  useNoResources,
  useRead,
  useUser,
} from "@lib/hooks";

import { RequiredResourceComponents, UsableResource } from "@types";
import { Input } from "@ui/input";
import { AlertTriangle } from "lucide-react";
import { useState } from "react";
import { TagSelector } from "@components/tags/tags-2";
import { ResourceListItemTable } from "@components/resource-list-item-table";
import { useSearchParams } from "react-router-dom";

export const AllResources = () => {
  const [search, setSearch] = useState("");
  const noResources = useNoResources();
  const user = useUser().data!;

  const tags = useSearchParams()[0].getAll("tag");

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
    <div className="p-4 border rounded-md bg-accent/25">
      <Section
        title={type + "s"}
        icon={<Components.Icon />}
        actions={
          <div className="flex gap-4">
            <Components.GroupActions />
            <ShowHideButton show={show} setShow={setShow} />
          </div>
        }
      >
        {show && (
          <ResourceListItemTable
            type={type as UsableResource}
            data={filtered as any}
          />
        )}
      </Section>
    </div>
  );
};
