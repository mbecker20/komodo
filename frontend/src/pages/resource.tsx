import { Page } from "@components/layouts";
import { ResourcePermissions } from "@components/permissions";
import { ResourceComponents } from "@components/resources";
import { ManageTags, ResourceTags } from "@components/tags";
import { ResourceUpdates } from "@components/updates/resource";
import { usePushRecentlyViewed, useResourceParamType } from "@lib/hooks";
import { useParams } from "react-router-dom";

export const Resource = () => {
  const type = useResourceParamType();
  const id = useParams().id as string;
  usePushRecentlyViewed({ type, id });

  if (!type || !id) return null;

  const Components = ResourceComponents[type];

  return (
    <Page
      title={<Components.Name id={id} />}
      subtitle={
        <div className="text-sm text-muted-foreground flex flex-col gap-2">
          <div className="flex gap-2">
            <Components.Icon id={id} />
            <Components.Description id={id} />
          </div>
          <div className="flex gap-8">
            <Components.Info id={id} />
          </div>
          <div className="flex gap-2">
            <ResourceTags target={{ id, type }} />
            <ManageTags target={{ id, type }} />
          </div>
        </div>
      }
      actions={<Components.Actions id={id} />}
    >
      <ResourceUpdates type={type} id={id} />
      <ResourcePermissions type={type} id={id} />
      {Object.keys(Components.Page).map((section) => {
        const Component = Components.Page[section];
        return <Component id={id} key={section} />;
      })}
    </Page>
  );
};
