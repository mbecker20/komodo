import { ExportButton } from "@components/export";
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
import { has_minimum_permissions } from "@lib/utils";
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
  const ui_write_disabled =
    useRead("GetCoreInfo", {}).data?.ui_write_disabled ?? false;

  if (!type || !id) return null;

  const Components = ResourceComponents[type];

  const canExecute = has_minimum_permissions(
    perms,
    Types.PermissionLevel.Execute
  );
  const canWrite = !ui_write_disabled && perms === Types.PermissionLevel.Write;

  const infoEntries = Object.entries(Components.Info);

  return (
    <Page
      title={name}
      icon={<Components.BigIcon id={id} />}
      titleRight={
        <div className="flex gap-4 items-center">
          {Object.entries(Components.Status).map(([key, Status]) => (
            <Status key={key} id={id} />
          ))}
        </div>
      }
      subtitle={
        <div className="flex gap-4 items-center text-muted-foreground">
          {infoEntries.map(([key, Info], i) => (
            <Fragment key={key}>
              {i !== 0 && "| "}
              <Info id={id} />
            </Fragment>
          ))}
          {infoEntries.length ? "| " : ""}
          <ExportButton targets={[{ type, id }]} />
        </div>
      }
      actions={
        <div className="flex flex-col gap-4 items-end">
          <div className="flex gap-2 items-center lg:justify-end">
            <div className="text-muted-foreground">tags:</div>
            <ResourceTags
              target={{ id, type }}
              className="text-sm"
              disabled={!canWrite}
              click_to_delete
            />
            {canWrite && <AddTags target={{ id, type }} />}
          </div>
          <ResourceDescription type={type} id={id} disabled={!canWrite} />
        </div>
      }
    >
      {/* Actions */}
      {canExecute && Object.keys(Components.Actions).length > 0 && (
        <Section title="Actions" icon={<Clapperboard className="w-4 h-4" />}>
          <div className="flex gap-4 items-center">
            {Object.entries(Components.Actions).map(([key, Action]) => (
              <Action key={key} id={id} />
            ))}
          </div>
        </Section>
      )}

      {/* Updates */}
      <ResourceUpdates type={type} id={id} />

      {/* Resource specific */}
      {Object.entries(Components.Page).map(([key, Component]) => (
        <Component key={key} id={id} />
      ))}

      {/* Config and Danger Zone */}
      <Components.Config id={id} />
      {canWrite && (
        <Section
          title="Danger Zone"
          icon={<AlertTriangle className="w-4 h-4" />}
          actions={type !== "Server" && <CopyResource type={type} id={id} />}
        >
          <Components.DangerZone id={id} />
        </Section>
      )}
    </Page>
  );
};
