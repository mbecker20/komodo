import { ExportButton } from "@components/export";
import { Section } from "@components/layouts";
import { ResourceComponents } from "@components/resources";
import {
  CopyResource,
  ResourceDescription,
} from "@components/resources/common";
import { AddTags, ResourceTags } from "@components/tags";
import {
  usePushRecentlyViewed,
  useRead,
  useResourceParamType,
  useSetTitle,
  useUser,
} from "@lib/hooks";
import { has_minimum_permissions, usableResourcePath } from "@lib/utils";
import { Types } from "@komodo/client";
import { UsableResource } from "@types";
import { Button } from "@ui/button";
import {
  AlertTriangle,
  ChevronLeft,
  Clapperboard,
  LinkIcon,
} from "lucide-react";
import { Link, useNavigate, useParams } from "react-router-dom";
import { ResourceNoficiations } from "./resource-notifications";

export const useEditPermissions = ({ type, id }: Types.ResourceTarget) => {
  const user = useUser().data;
  const perms = useRead("GetPermissionLevel", { target: { type, id } }).data;
  const info = useRead("GetCoreInfo", {}).data;
  const ui_write_disabled = info?.ui_write_disabled ?? false;
  const disable_non_admin_create = info?.disable_non_admin_create ?? false;

  const canWrite = !ui_write_disabled && perms === Types.PermissionLevel.Write;
  const canExecute = has_minimum_permissions(
    perms,
    Types.PermissionLevel.Execute
  );

  if (type === "Server") {
    return {
      canWrite,
      canExecute,
      canCreate:
        user?.admin ||
        (!disable_non_admin_create && user?.create_server_permissions),
    };
  }
  if (type === "Build") {
    return {
      canWrite,
      canExecute,
      canCreate:
        user?.admin ||
        (!disable_non_admin_create && user?.create_build_permissions),
    };
  }
  if (type === "Alerter" || type === "Builder") {
    return {
      canWrite,
      canExecute,
      canCreate: user?.admin,
    };
  }

  return {
    canWrite,
    canExecute,
    canCreate: user?.admin || !disable_non_admin_create,
  };
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

  const { canCreate, canExecute, canWrite } = useEditPermissions({ type, id });

  if (!type || !id) return null;

  if (!resource) {
    if (resources) return <NotFound type={type} />;
    else return null;
  }

  let showExport = true;
  if (type === "ResourceSync") {
    const info = resource?.info as Types.ResourceSyncListItemInfo;
    showExport = !info?.file_contents && (info.file_contents || !info.managed);
  }

  const Components = ResourceComponents[type];
  const links = full_resource ? Components.resource_links(full_resource) : [];

  return (
    <div>
      <div className="w-full flex items-center justify-between mb-12">
        <Link to={"/" + usableResourcePath(type)}>
          <Button className="gap-2" variant="secondary">
            <ChevronLeft className="w-4" />
            Back
          </Button>
        </Link>
        <div className="flex items-center gap-4">
          {type !== "Server" && canCreate && (
            <CopyResource type={type} id={id} />
          )}
          {showExport && <ExportButton targets={[{ type, id }]} />}
        </div>
      </div>
      <div className="flex flex-col xl:flex-row gap-4">
        <ResourceHeader type={type} id={id} links={links} />
        <ResourceNoficiations type={type} id={id} />
      </div>
      <div className="mt-8 flex flex-col gap-12">
        {canExecute && Object.keys(Components.Actions).length > 0 && (
          <Section title="Actions" icon={<Clapperboard className="w-4 h-4" />}>
            <div className="flex gap-4 items-center flex-wrap">
              {Object.entries(Components.Actions).map(([key, Action]) => (
                <Action key={key} id={id} />
              ))}
            </div>
          </Section>
        )}
        {Object.entries(Components.Page).map(([key, Component]) => (
          <Component key={key} id={id} />
        ))}
        <Components.Config id={id} />
        {canWrite && (
          <Section
            title="Danger Zone"
            icon={<AlertTriangle className="w-6 h-6" />}
            // actions={
            // type !== "Server" &&
            // canCreate && <CopyResource type={type} id={id} />
            // }
          >
            <Components.DangerZone id={id} />
          </Section>
        )}
      </div>
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
  const resource = useRead(`List${type}s`, {}).data?.find((r) => r.id === id);
  useSetTitle(resource?.name);

  const Components = ResourceComponents[type];
  const infoEntries = Object.entries(Components.Info);

  const { canWrite } = useEditPermissions({ type, id });

  return (
    <div className="w-full flex flex-col gap-4">
      <div className="flex flex-col gap-4 border rounded-md">
        <Components.ResourcePageHeader id={id} />
        <div className="flex items-center gap-4 flex-wrap p-4 pt-0">
          {infoEntries.map(([key, Info]) => (
            <div key={key} className="pr-4 text-sm border-r">
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
              <LinkIcon className="w-4" />
              <div className="max-w-[150px] lg:max-w-[250px] overflow-hidden overflow-ellipsis">
                {link}
              </div>
            </a>
          ))}
          <div className="flex items-center gap-3 h-7 lg:justify-self-end">
            <p className="text-sm text-muted-foreground">Tags:</p>
            <div className="flex items-center gap-2">
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
      <ResourceDescription type={type} id={id} disabled={!canWrite} />
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
