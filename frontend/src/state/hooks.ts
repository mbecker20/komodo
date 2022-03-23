import { Collection, Update } from "@monitor/types";
import { createEffect, createResource } from "solid-js";
import { filterOutFromObj } from "../util/helpers";
import { useLocalStorage } from "../util/hooks";
import {
  getBuilds,
  getDeployments,
  getServers,
  getUpdates,
} from "../util/query";

export function useSelected() {
  const [selected, setSelected] = useLocalStorage<{
    id: string;
    type: "server" | "deployment" | "build";
  }>({ id: "", type: "deployment" }, "selected-item");
  const set = (id: string, type: "server" | "deployment" | "build") => {
    setSelected({ id, type });
  };
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

  createEffect(() => console.log(collection()));

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
