import { Page, Section } from "@components/layouts";
import { ResourceComponents } from "@components/resources";
import {
  CopyResource,
  ResourceDescription,
} from "@components/resources/common";
import { AddTags, ResourceTags } from "@components/tags";
import { ResourceUpdates } from "@components/updates/resource";
import {
  usePushRecentlyViewed,
  useRead,
  useResourceParamType,
  useSetTitle,
} from "@lib/hooks";
import { Types } from "@monitor/client";
import { AlertTriangle, Clapperboard } from "lucide-react";
import { Fragment } from "react";
import { useParams } from "react-router-dom";

export const Resource = () => {
  const type = useResourceParamType()!;
  const id = useParams().id as string;
  usePushRecentlyViewed({ type, id });
  const name = useRead(`List${type}s`, {}).data?.find((r) => r.id === id)?.name;
  useSetTitle(name);
  const perms = useRead("GetPermissionLevel", { target: { type, id } }).data;

  if (!type || !id) return null;

  const Components = ResourceComponents[type];

  const canExecute = perms
    ? [Types.PermissionLevel.Execute, Types.PermissionLevel.Write].includes(
        perms
      )
    : false;

  return (
    <Page
      title={<Components.Name id={id} />}
      titleRight={
        <div className="flex gap-4 items-center">
          {Object.entries(Components.Status).map(([key, Status]) => (
            <Status key={key} id={id} />
          ))}
        </div>
      }
      subtitle={
        <div className="flex flex-col gap-4">
          <div className="flex gap-4 items-center text-muted-foreground">
            <Components.Icon id={id} />
            {Object.entries(Components.Info).map(([key, Info], i) => (
              <Fragment key={key}>
                | <Info key={i} id={id} />
              </Fragment>
            ))}
          </div>
          <ResourceDescription type={type} id={id} />
        </div>
      }
      actions={
        <div className="flex gap-2 items-center">
          <div className="text-muted-foreground">tags:</div>
          <ResourceTags
            target={{ id, type }}
            className="text-sm"
            click_to_delete
          />
          <AddTags target={{ id, type }} />
        </div>
      }
    >
      {/* Actions and Updates */}
      {canExecute && (
        <Section title="Actions" icon={<Clapperboard className="w-4 h-4" />}>
          <div className="flex gap-4 items-center">
            {Object.entries(Components.Actions).map(([key, Action]) => (
              <Action key={key} id={id} />
            ))}
          </div>
        </Section>
      )}
      <ResourceUpdates type={type} id={id} />

      {/* Resource specific */}
      {Object.entries(Components.Page).map(([key, Component]) => (
        <Component key={key} id={id} />
      ))}

      {/* Config and Danger Zone */}
      <Components.Config id={id} />
      <Section
        title="Danger Zone"
        icon={<AlertTriangle className="w-4 h-4" />}
        actions={type !== "Server" && <CopyResource type={type} id={id} />}
      >
        <Components.DangerZone id={id} />
      </Section>
    </Page>
  );
};
