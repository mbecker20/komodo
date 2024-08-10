import { ExportButton } from "@components/export";
import { Section } from "@components/layouts";
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
import { has_minimum_permissions, usableResourcePath } from "@lib/utils";
import { Types } from "@monitor/client";
import { UsableResource } from "@types";
import { Button } from "@ui/button";
import { AlertTriangle, ChevronLeft, Clapperboard } from "lucide-react";
import { useNavigate, useParams } from "react-router-dom";

const useEditPermissions = ({ type, id }: Types.ResourceTarget) => {
  const perms = useRead("GetPermissionLevel", { target: { type, id } }).data;
  const ui_write_disabled =
    useRead("GetCoreInfo", {}).data?.ui_write_disabled ?? false;

  const canWrite = !ui_write_disabled && perms === Types.PermissionLevel.Write;
  const canExecute = has_minimum_permissions(
    perms,
    Types.PermissionLevel.Execute
  );

  return { canWrite, canExecute };
};

export const Resource = () => {
  const type = useResourceParamType()!;
  const id = useParams().id as string;
  usePushRecentlyViewed({ type, id });

  const { canWrite, canExecute } = useEditPermissions({ type, id });

  const resources = useRead(`List${type}s`, {}).data;
  const resource = resources?.find((resource) => resource.id === id);

  if (!type || !id) return null;

  if (resources && !resource) {
    return <NotFound type={type} />;
  }

  const Components = ResourceComponents[type];

  return (
    <div className="flex flex-col gap-16">
      {/* Header */}
      <ResourceHeader type={type} id={id} />

      {/* Actions */}
      {canExecute && Object.keys(Components.Actions).length > 0 && (
        <Section title="Actions" icon={<Clapperboard className="w-4 h-4" />}>
          <div className="flex gap-4 items-center flex-wrap">
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
          icon={<AlertTriangle className="w-6 h-6" />}
          actions={type !== "Server" && <CopyResource type={type} id={id} />}
        >
          <Components.DangerZone id={id} />
        </Section>
      )}
    </div>
  );
};

const ResourceHeader = ({ type, id }: { type: UsableResource; id: string }) => {
  const name = useRead(`List${type}s`, {}).data?.find((r) => r.id === id)?.name;
  useSetTitle(name);

  const Components = ResourceComponents[type];
  const infoEntries = Object.entries(Components.Info);

  const { canWrite } = useEditPermissions({ type, id });
  const nav = useNavigate();

  return (
    <div className="flex flex-col gap-4">
      <div className="flex items-center justify-between mb-4">
        <Button
          className="gap-2"
          variant="secondary"
          onClick={() => nav("/" + usableResourcePath(type))}
        >
          <ChevronLeft className="w-4" /> Back
        </Button>
        <ExportButton targets={[{ type, id }]} />
      </div>

      <div className="flex flex-col gap-4">
        <div className="flex gap-4 justify-between flex-wrap">
          <div className="flex items-center gap-4">
            <div className="mt-1">
              <Components.BigIcon id={id} />
            </div>
            <h1 className="text-3xl">{name}</h1>
            <div className="flex items-center gap-4 flex-wrap">
              {Object.entries(Components.Status).map(([key, Status]) => (
                <Status key={key} id={id} />
              ))}
            </div>
          </div>

          <div className="flex items-center gap-2">
            <p className="text-sm text-muted-foreground">Description: </p>
            <ResourceDescription type={type} id={id} disabled={!canWrite} />
          </div>
        </div>

        <div className="flex gap-4 justify-between flex-wrap">
          <div className="flex items-center gap-4 flex-wrap">
            {infoEntries.map(([key, Info]) => (
              <div
                key={key}
                className="pr-4 border-r last:pr-0 last:border-none"
              >
                <Info id={id} />
              </div>
            ))}
          </div>

          <div className="flex items-center gap-2 h-7 lg:justify-self-end">
            <p className="text-sm text-muted-foreground">Tags:</p>
            <ResourceTags
              target={{ id, type }}
              className="text-sm"
              disabled={!canWrite}
              click_to_delete
            />
            {canWrite && <AddTags target={{ id, type }} />}
          </div>
        </div>
      </div>
    </div>
  );
};

const NotFound = ({ type }: { type: UsableResource }) => {
  const nav = useNavigate();
  const Components = ResourceComponents[type];
  return (
    <div className="flex flex-col gap-4">
      <div className="flex items-center justify-between mb-4">
        <Button
          className="gap-2"
          variant="secondary"
          onClick={() => nav("/" + usableResourcePath(type))}
        >
          <ChevronLeft className="w-4" /> Back
        </Button>
      </div>
      <div className="grid lg:grid-cols-2 gap-4">
        <div className="flex items-center gap-4">
          <div className="mt-1">
            <Components.BigIcon />
          </div>
          <h1 className="text-3xl">{type} not found</h1>
        </div>
      </div>
    </div>
  );
};
