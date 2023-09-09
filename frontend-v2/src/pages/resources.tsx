import { Page, Section } from "@components/layouts";
import { ResourceCard } from "@components/resources";
import { useRead, useResourceParamType } from "@lib/hooks";
import { Button } from "@ui/button";
import { Input } from "@ui/input";
import { PlusCircle } from "lucide-react";
import { useState } from "react";

export const Resources = () => {
  const type = useResourceParamType();
  const list = useRead(`List${type}s`, {}).data;

  const [search, set] = useState("");

  return (
    <Page
      title={`${type}s`}
      actions={
        <div className="flex gap-4">
          <Input
            value={search}
            onChange={(e) => set(e.target.value)}
            placeholder="search..."
            className="w-96"
          />
          <Button className="gap-2">
            New <PlusCircle className="w-4 h-4" />
          </Button>
        </div>
      }
    >
      <Section title="">
        <div className="grid grid-cols-3 gap-4">
          {list?.map(({ id }) => (
            <ResourceCard key={id} target={{ type, id }} />
          ))}
        </div>
      </Section>
    </Page>
  );
};
