import { Collection, Update } from "@monitor/types";
import { createEffect, createResource, createSignal } from "solid-js";
import { URL } from "..";
import { filterOutFromObj } from "../util/helpers";
import {
  getBuilds,
  getDeployments,
  getServers,
  getUpdates,
} from "../util/query";
import { State } from "./StateProvider";

export function useSelected({ servers, builds, deployments }: State) {
  const [_type, id] = location.pathname.split("/").filter((val) => val);
  const type =
    _type === "deployment" || _type === "server" || _type === "build"
      ? _type
      : undefined;
  const [selected, setSelected] = createSignal<{
    id: string;
    type: "server" | "deployment" | "build";
  }>({ id: id || "", type: type || "deployment" });

  const set = (id: string, type: "server" | "deployment" | "build") => {
    setSelected({ id, type });
    history.pushState({}, "", `${location.origin}/${type}/${id}`);
  };

  createEffect(() => {
    if (selected().type === "deployment" && deployments.loaded()) {
      if (!deployments.get(selected().id)) {
        const id = deployments.ids()![0];
        set(id, "deployment");
      } else {
        const [type, id] = location.pathname.split("/").filter((val) => val);
        if (type !== selected().type || id !== selected().id) {
          history.replaceState({}, "", `${selected().type}/${selected().id}`);
        }
      }
    } else if (selected().type === "server" && servers.loaded()) {
      if (!servers.get(selected().id)) {
        const id = servers.ids()![0];
        set(id, "server");
      } else {
        const [type, id] = location.pathname.split("/").filter((val) => val);
        if (type !== selected().type || id !== selected().id) {
          history.replaceState({}, "", `${selected().type}/${selected().id}`);
        }
      }
    } else if (selected().type === "build" && builds.loaded()) {
      if (!builds.get(selected().id)) {
        const id = builds.ids()![0];
        set(id, "build");
      } else {
        const [type, id] = location.pathname.split("/").filter((val) => val);
        if (type !== selected().type || id !== selected().id) {
          history.replaceState({}, "", `${selected().type}/${selected().id}`);
        }
      }
    }
  });

  return {
    id: () => selected().id,
    type: () => selected().type,
    set,
  };
}

export function useServers() {
  return useCollection(getServers);
}

export function useBuilds() {
  return useCollection(getBuilds);
}

export function useDeployments(query?: Parameters<typeof getDeployments>[0]) {
  return useCollection(() => getDeployments(query));
}

export function useUpdates(query?: Parameters<typeof getUpdates>[0]) {
  return useArray(() => getUpdates(query));
}

export function useArray<T>(query: () => Promise<T[]>) {
  const [collection, { mutate }] = createResource(query);
  const push = (update: Update) => {
    mutate((updates: any) => [update, ...updates]);
  };
  const loaded = () => (collection() ? true : false);
  return {
    collection,
    push,
    loaded,
  };
}

export function useCollection<T>(query: () => Promise<Collection<T>>) {
  const [collection, { mutate }] = createResource(query);
  const add = (item: T & { _id?: string }) => {
    mutate((collection: any) => ({ ...collection, [item._id!]: item }));
  };
  const addMany = (items: Collection<T>) => {
    mutate((collection: any) => ({ ...collection, ...items }));
  };
  const del = (id: string) => {
    mutate((collection: any) => filterOutFromObj(collection, [id]));
  };
  const update = (item: T & { _id?: string }) => {
    mutate((collection: any) => ({
      ...collection,
      [item._id!]: { ...collection[item._id!], ...item },
    }));
  };
  const get = (id: string) => {
    return collection() && collection()![id];
  };
  const ids = () => collection() && Object.keys(collection()!);
  const loaded = () => (collection() ? true : false);

  return {
    collection,
    add,
    addMany,
    delete: del,
    update,
    get,
    ids,
    loaded,
  };
}
