import {
  createEffect,
  createResource,
  createSignal,
  onCleanup,
} from "solid-js";
import { client } from "..";
import { Server, ServerStatus, SystemStats } from "../types";
import {
  filterOutFromObj,
  intoCollection,
  keepOnlyInObj,
} from "../util/helpers";

const pages: PageType[] = ["deployment", "server", "build", "users"];
type PageType = "deployment" | "server" | "build" | "users" | "home";
type Collection<T> = Record<string, T>;

export function useSelected() {
  const [_type, id] = location.pathname.split("/").filter((val) => val);
  const type = (
    pages.includes(_type as PageType) ? _type : undefined
  ) as PageType;
  const [selected, setSelected] = createSignal<{
    id: string;
    type: PageType;
  }>({ id: id || "", type: type || "home" });

  history.replaceState(
    { type: selected().type, id: selected().id },
    "",
    selected().type === "home"
      ? location.origin
      : `${location.origin}/${selected().type}/${selected().id}`
  );

  const [prevSelected, setPrevSelected] = createSignal<{
    id: string;
    type: PageType;
  }>();

  const set = (id: string, type: PageType) => {
    setPrevSelected({ id: selected().id, type: selected().type });
    setSelected({ id, type });
    if (type === "home") {
      history.pushState({ id, type }, "", location.origin);
    } else {
      history.pushState({ id, type }, "", `${location.origin}/${type}/${id}`);
    }
  };

  const popstate = (e: any) => {
    setSelected({ id: e.state.id, type: e.state.type });
  };

  window.addEventListener("popstate", popstate);

  onCleanup(() => window.removeEventListener("popstate", popstate));

  return {
    id: () => selected().id,
    type: () => selected().type,
    set,
    prevId: () => prevSelected()?.id,
    prevType: () => prevSelected()?.type,
  };
}

export function useServers() {
  return useCollection(() => client.list_servers().then(intoCollection));
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
        (serverStatus ? serverStatus === "ok" : true)
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
  return useCollection(() => client.list_builds().then(intoCollection));
}

export function useDeployments() {
  const deployments = useCollection(() =>
    client.list_deployments().then(intoCollection)
  );
  const state = (id: string) => {
    const deployment = deployments.get(id)!;
    return deployment.container?.state;
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

// export function useUpdates(query?: Parameters<typeof getUpdates>[0]) {
//   const updates = useArray(() => getUpdates(query));
//   const [noMore, setNoMore] = createSignal(false);
//   const loadMore = async () => {
//     const offset = updates.collection()?.length;
//     if (offset) {
//       const newUpdates = await getUpdates({ offset });
//       updates.addManyToEnd(newUpdates);
//       if (newUpdates.length !== 10) {
//         setNoMore(true);
//       }
//     }
//   };
//   return {
//     noMore,
//     loadMore,
//     ...updates,
//   };
// }

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
