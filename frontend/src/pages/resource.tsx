import { Page } from "@components/layouts";
import { ResourceComponents } from "@components/resources";
import { AddTags, ResourceTags } from "@components/tags";
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
      titleRight={
        <div className="flex gap-2">
          <ResourceTags target={{ id, type }} click_to_delete />
          <AddTags target={{ id, type }} />
        </div>
      }
      subtitle={
        <div className="text-sm text-muted-foreground flex flex-col gap-2">
          <div className="flex gap-4 items-center">
            <div className="flex gap-2 items-center">
              <Components.Icon id={id} />
              <Components.Status id={id} />
            </div>
            |
            <Components.Info id={id} />
          </div>
        </div>
      }
      actions={<Components.Actions id={id} />}
    >
      <ResourceUpdates type={type} id={id} />
      {/* <ResourcePermissions type={type} id={id} /> */}
      {Object.entries(Components.Page).map(([section, Component]) => (
        <Component id={id} key={section} />
      ))}
    </Page>
  );
};
