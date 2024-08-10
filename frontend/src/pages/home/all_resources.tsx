import { ExportButton } from "@components/export";
import { Page, Section } from "@components/layouts";
import { ResourceComponents } from "@components/resources";
import { TagsFilter } from "@components/tags";
import {
  useFilterResources,
  useNoResources,
  useRead,
  useTagsFilter,
  useUser,
} from "@lib/hooks";
import { cn } from "@lib/utils";
import { Types } from "@monitor/client";
import { RequiredResourceComponents, UsableResource } from "@types";
import { Button } from "@ui/button";
import { Input } from "@ui/input";
import { AlertTriangle, ChevronDown, ChevronUp } from "lucide-react";
import { useState } from "react";

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
            <TagsFilter />
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
