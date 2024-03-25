import { Page } from "@components/layouts";
import { ResourceComponents } from "@components/resources";
import { AddTags, ResourceTags } from "@components/tags";
import { UpdatesTable } from "@components/updates/table";
import { useRead, useResourceParamType } from "@lib/hooks";
import { useParams } from "react-router-dom";

export const ResourceUpdates = () => {
  const type = useResourceParamType();
  const id = useParams().id as string;
  const updates = useRead("ListUpdates", {
    query: {
      "target.type": type,
      "target.id": id,
    },
  }).data;
  const Components = ResourceComponents[type];
  return (
    <Page
      title={<Components.Name id={id} />}
      titleRight={
        <div className="flex gap-2">
          <ResourceTags target={{ id, type }} click_to_delete />
          <AddTags target={{ id, type }} />
        </div>
      }
    >
      <UpdatesTable updates={updates?.updates ?? []} />
    </Page>
  );
};
