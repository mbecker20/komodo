import { Page, Section, ResourceCard } from "@components/layouts";
import { ResourceComponents } from "@components/resources";
import { useRead, useResourceParamType } from "@lib/hooks";
import { Button } from "@ui/button";
import { Input } from "@ui/input";
import { useState } from "react";

export const Resources = () => {
  const type = useResourceParamType();
  const Components = ResourceComponents[type];

  const list = useRead(`List${type}s`, {}).data;

  const [search, set] = useState("");
  const [view, setView] = useState<"cards" | "table">("table");

  return (
    <Page
      title={`${type}s`}
      actions={
        <div className="flex gap-4">
          <Button
            variant="outline"
            onClick={() => setView((v) => (v === "cards" ? "table" : "cards"))}
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
