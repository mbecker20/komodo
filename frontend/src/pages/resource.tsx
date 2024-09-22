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
import { Types } from "@komodo/client";
import { UsableResource } from "@types";
import { Button } from "@ui/button";
import { AlertTriangle, ChevronLeft, Clapperboard, Link } from "lucide-react";
import { useNavigate, useParams } from "react-router-dom";

export const useEditPermissions = ({ type, id }: Types.ResourceTarget) => {
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

  if (!type || !id) return null;

  return <ResourceInner type={type} id={id} />;
};

const ResourceInner = ({ type, id }: { type: UsableResource; id: string }) => {
  const resources = useRead(`List${type}s`, {}).data;
  const resource = resources?.find((resource) => resource.id === id);
  const full_resource = useRead(`Get${type}`, { id } as any).data;

  usePushRecentlyViewed({ type, id });

  const { canWrite, canExecute } = useEditPermissions({ type, id });

  if (!type || !id) return null;

  if (!resource) {
    if (resources) return <NotFound type={type} />;
    else return null;
  }

  const Components = ResourceComponents[type];
  const links = full_resource ? Components.resource_links(full_resource) : [];

  return (
    <div className="flex flex-col gap-16">
      {/* Header */}
      <ResourceHeader type={type} id={id} links={links} />

      {/* Actions */}
      {canExecute &&
        (Object.keys(Components?.Actions ?? {})?.length ?? 0) > 0 && (
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

const ResourceHeader = ({
  type,
  id,
  links,
}: {
  type: UsableResource;
  id: string;
  links: string[] | undefined;
}) => {
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
            <h1 className="text-3xl text-nowrap">{name}</h1>
            <div className="flex items-center gap-4 flex-wrap">
              {Object.entries(Components.Status).map(([key, Status]) => (
                <Status key={key} id={id} />
              ))}
            </div>
          </div>

          <div className="flex items-center gap-2">
            {/* <p className="text-sm text-muted-foreground">Description: </p> */}
            <ResourceDescription type={type} id={id} disabled={!canWrite} />
          </div>
        </div>

        <div className="flex gap-4 justify-between flex-wrap">
          <div className="flex items-center gap-4 flex-wrap">
            {infoEntries.map(([key, Info]) => (
              <div
                key={key}
                className="pr-4 text-sm border-r last:pr-0 last:border-none"
              >
                <Info id={id} />
              </div>
            ))}
            {links?.map((link) => (
              <a
                key={link}
                target="__blank"
                href={link}
                className="flex gap-2 items-center pr-4 text-sm border-r cursor-pointer hover:underline last:pr-0 last:border-none"
              >
                <Link className="w-4 h-4" />
                <div className="max-w-[150px] lg:max-w-[250px] overflow-hidden overflow-ellipsis">
                  {link}
                </div>
              </a>
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

const NotFound = ({ type }: { type: UsableResource | undefined }) => {
  const nav = useNavigate();
  const Components = type && ResourceComponents[type];
  return (
    <div className="flex flex-col gap-4">
      {type && (
        <div className="flex items-center justify-between mb-4">
          <Button
            className="gap-2"
            variant="secondary"
            onClick={() => nav("/" + usableResourcePath(type))}
          >
            <ChevronLeft className="w-4" /> Back
          </Button>
        </div>
      )}
      <div className="grid lg:grid-cols-2 gap-4">
        <div className="flex items-center gap-4">
          <div className="mt-1">{Components && <Components.BigIcon />}</div>
          <h1 className="text-3xl">{type ?? "??"} not found</h1>
        </div>
      </div>
    </div>
  );
};
