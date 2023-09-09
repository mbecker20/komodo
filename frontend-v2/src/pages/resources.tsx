import { Page, Section, ResourceCard } from "@components/layouts";
import { ResourceComponents } from "@components/resources";
import { useRead, useResourceParamType } from "@lib/hooks";
import { Input } from "@ui/input";
import { useState } from "react";

export const Resources = () => {
  const type = useResourceParamType();
  const Components = ResourceComponents[type];

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
          <Components.New />
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
