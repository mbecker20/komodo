import {
  createEffect,
  createResource,
  createSignal,
} from "solid-js";
import { client } from "..";
import { ServerStatus, SystemStats, UpdateTarget } from "../types";
import {
  filterOutFromObj,
  intoCollection,
  keepOnlyInObj,
} from "../util/helpers";

type Collection<T> = Record<string, T>;

export function useServers() {
  return useCollection(() => client.list_servers().then(intoCollection), ["_id", "$oid"]);
}

export function useServerStats() {
  const [stats, set] = createSignal<Record<string, SystemStats | undefined>>(
    {}
  );
  const load = async (serverID: string) => {
    const stats = await client.get_server_stats(serverID);
    set((s) => ({ ...s, [serverID]: stats }));
  };
  const loading: Record<string, boolean> = {};
  return {
    get: (serverID: string, serverStatus?: ServerStatus) => {
      const stat = stats()[serverID];
      if (
        stat === undefined &&
        !loading[serverID] &&
        (serverStatus ? serverStatus === ServerStatus.Ok : true)
      ) {
        loading[serverID] = true;
        load(serverID);
      }
      return stat;
    },
    load,
  };
}

export function useBuilds() {
  return useCollection(() => client.list_builds().then(intoCollection), ["_id", "$oid"]);
}

export function useDeployments() {
  const deployments = useCollection(() =>
    client.list_deployments().then(intoCollection),
    ["deployment", "_id", "$oid"]
  );
  const state = (id: string) => {
    const deployment = deployments.get(id)!;
    return deployment.state;
  };
  const status = (id: string) => {
    const deployment = deployments.get(id)!;
    const status = deployment.container?.status;
    return status;
  };
  return {
    ...deployments,
    status,
    state,
  };
}

export function useUpdates(target?: UpdateTarget) {
  const updates = useArray(() => client.list_updates(0, target));
  const [noMore, setNoMore] = createSignal(false);
  const loadMore = async () => {
    const offset = updates.collection()?.length;
    if (offset) {
      const newUpdates = await client.list_updates(offset, target);
      updates.addManyToEnd(newUpdates);
      if (newUpdates.length !== 10) {
        setNoMore(true);
      }
    }
  };
  return {
    noMore,
    loadMore,
    ...updates,
  };
}

export function useArray<T>(query: () => Promise<T[]>) {
  const [collection, set] = createSignal<T[]>();
  createEffect(() => {
    query().then(set);
  });
  const add = (item: T) => {
    set((items: any) => (items ? [item, ...items] : [item]));
  };
  const addManyToEnd = (items: T[]) => {
    set((curr: any) => (curr ? [...curr, ...items] : items));
  };
  const loaded = () => (collection() ? true : false);
  return {
    collection,
    add,
    addManyToEnd,
    loaded,
  };
}

export function useCollection<T>(query: () => Promise<Collection<T>>, idPath: string[]) {
  const [collection, { mutate }] = createResource(query);
  const add = (item: T) => {
    mutate((collection: any) => ({ ...collection, [getNestedEntry(item, idPath)]: item }));
  };
  const addMany = (items: Collection<T>) => {
    mutate((collection: any) => ({ ...collection, ...items }));
  };
  const del = (id: string) => {
    mutate((collection: any) => filterOutFromObj(collection, [id]));
  };
  const update = (item: T) => {
    mutate((collection: any) => ({
      ...collection,
      [getNestedEntry(item, idPath)]: {
        ...collection[getNestedEntry(item, idPath)],
        ...item,
      },
    }));
  };
  const get = (id: string) => {
    return collection() && collection()![id];
  };
  const ids = () => collection() && Object.keys(collection()!);
  const loaded = () => (collection() ? true : false);
  const filter = (condition: (item: T) => boolean) => {
    const coll = collection();
    const _ids = coll && ids()!.filter((id) => condition(coll[id]));
    return _ids && keepOnlyInObj(coll, _ids);
  };
  const filterArray = (condition: (item: T) => boolean) => {
    const coll = collection();
    const _ids = coll && ids()!.filter((id) => condition(coll[id]));
    return _ids && _ids.map((id) => coll[id]);
  };
  return {
    collection,
    add,
    addMany,
    delete: del,
    update,
    get,
    ids,
    loaded,
    filter,
    filterArray,
  };
}

function getNestedEntry(obj: any, path: string[]): any {
  if (path.length === 0) {
    return obj
  } else {
    return getNestedEntry(obj[path[0]], path.slice(1));
  }
}