import { Collection } from "@monitor/types";
import { filterOutFromObj } from "@monitor/util";
import { createResource } from "solid-js";
import { client, WS_URL } from "..";
import {
  getBuilds,
  getDeployments,
  getServers,
  getUpdates,
} from "../util/query";

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
  const [collection, { refetch }] = createResource(() => getUpdates(query));
  return {
    collection,
    refetch,
  };
}

export function useCollection<T>(query: () => Promise<Collection<T>>) {
  const [collection, { mutate }] = createResource(query);
  
  const add = (items: Collection<T>) => {
    mutate((collection: any) => ({ ...collection, ...items }));
  };
  const del = (id: string) => {
    mutate((collection: any) => filterOutFromObj(collection, [id]));
  };
  const update = (item: T & { _id?: string }) => {
    mutate((collection: any) => ({ ...collection, [item._id!]: item }));
  };
  const get = (id: string) => {
    return collection() && collection()![id];
  };

  return {
    collection,
    add,
    delete: del,
    update,
    get,
  };
}
