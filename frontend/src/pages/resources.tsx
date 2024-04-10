import { Page, Section, ResourceCard } from "@components/layouts";
import { ResourceComponents } from "@components/resources";
import { TagsFilter, useTagsFilter } from "@components/tags";
import { useRead, useResourceParamType, useSetTitle } from "@lib/hooks";
import { Button } from "@ui/button";
import { Input } from "@ui/input";
import { useState } from "react";
import { atomWithStorage } from "jotai/utils";
import { useAtom } from "jotai";

const viewAtom = atomWithStorage<"cards" | "table">("list-show-as-v0", "table");

export const Resources = () => {
  const type = useResourceParamType()!;
  useSetTitle(type + "s");
  const Components = ResourceComponents[type];

  const tags = useTagsFilter();

  const list = useRead(`List${type}s`, { query: { tags } }).data;

  const [search, set] = useState("");
  const [view, setView] = useAtom(viewAtom);

  return (
    <Page
      title={`${type}s`}
      actions={
        <div className="grid gap-4 justify-items-end">
          <div className="flex gap-4">
            <Button
              variant="outline"
              onClick={() =>
                setView((v) => (v === "cards" ? "table" : "cards"))
              }
            >
              show as {view === "cards" ? "table" : "cards"}
            </Button>
            <Input
              value={search}
              onChange={(e) => set(e.target.value)}
              placeholder="search..."
              className="w-96"
            />
            <Components.New />
          </div>
          <TagsFilter />
        </div>
      }
    >
      <Section title="">
        {view === "cards" ? (
          <div className="grid gap-4 md:grid-cols-2 lg:grid-cols-3">
            {list?.map(({ id }) => (
              <ResourceCard key={id} target={{ type, id }} />
            ))}
          </div>
        ) : (
          <Components.Table />
        )}
      </Section>
    </Page>
  );
};
