import { Page, Section, ResourceCard, ResourceRow } from "@components/layouts";
import { ResourceComponents } from "@components/resources";
import { useRead, useResourceParamType } from "@lib/hooks";
import { cn } from "@lib/utils";
import { Button } from "@ui/button";
import { Input } from "@ui/input";
import { useState } from "react";

export const Resources = () => {
  const type = useResourceParamType();
  const Components = ResourceComponents[type];

  const list = useRead(`List${type}s`, {}).data;

  const [search, set] = useState("");
  const [view, setView] = useState<"cards" | "rows">("cards");

  return (
    <Page
      title={`${type}s`}
      actions={
        <div className="flex gap-4">
          <Button
            variant="outline"
            onClick={() => setView((v) => (v === "cards" ? "rows" : "cards"))}
          >
            show as {view === "cards" ? "rows" : "cards"}
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
        <div
          className={cn(
            "grid gap-4",
            view === "cards" ? "md:grid-cols-2 lg:grid-cols-3" : "items-center"
          )}
        >
          {list?.map(({ id }) =>
            view === "cards" ? (
              <ResourceCard key={id} target={{ type, id }} />
            ) : (
              <ResourceRow key={id} target={{ type, id }} />
            )
          )}
        </div>
      </Section>
    </Page>
  );
};
