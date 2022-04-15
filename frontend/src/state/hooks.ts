import { Collection, ContainerStatus } from "@monitor/types";
import {
  createEffect,
  createResource,
  createSignal,
  onCleanup,
} from "solid-js";
import { filterOutFromObj, keepOnlyInObj } from "../util/helpers";
import {
  getBuilds,
  getDeployments,
  getServers,
  getUpdates,
} from "../util/query";
import { State } from "./StateProvider";

const pages: PageType[] = ["deployment", "server", "build", "users"];
type PageType = "deployment" | "server" | "build" | "users" | "home";

export function useSelected({ servers, builds, deployments }: State) {
  const [_type, id] = location.pathname.split("/").filter((val) => val);
  const [firstLoad, setFirstLoad] = createSignal(true);
  const type = (
    pages.includes(_type as PageType) ? _type : undefined
  ) as PageType;
  const [selected, setSelected] = createSignal<{
    id: string;
    type: PageType;
  }>({ id: id || "", type: type || "home" });

  const set = (id: string, type: PageType) => {
    setSelected({ id, type });
    if (type === "home") {
      history.pushState({ id, type }, "", `${location.origin}`);
    } else {
      history.pushState({ id, type }, "", `${location.origin}/${type}/${id}`);
    }
  };

  createEffect(() => {
    if (firstLoad()) {
      if (selected().type === "home") {
        history.replaceState({ id: "", type: "home" }, "", "/");
        setFirstLoad(false);
      } else if (selected().type === "deployment" && deployments.loaded()) {
        if (!deployments.get(selected().id)) {
          const id = deployments.ids()![0];
          set(id, "deployment");
        } else {
          const [type, id] = location.pathname.split("/").filter((val) => val);
          if (type !== selected().type || id !== selected().id) {
            history.replaceState(
              { type, id },
              "",
              `${selected().type}/${selected().id}`
            );
            setFirstLoad(false);
          }
        }
        setFirstLoad(false);
      } else if (
        selected().type === "server" &&
        servers.loaded() &&
        deployments.loaded()
      ) {
        if (!servers.get(selected().id)) {
          const id = servers.ids()![0];
          set(id, "server");
        } else {
          const [type, id] = location.pathname.split("/").filter((val) => val);
          if (type !== selected().type || id !== selected().id) {
            history.replaceState(
              { type, id },
              "",
              `${selected().type}/${selected().id}`
            );
          }
        }
        setFirstLoad(false);
      } else if (
        selected().type === "build" &&
        builds.loaded() &&
        deployments.loaded()
      ) {
        if (!builds.get(selected().id)) {
          const id = builds.ids()![0];
          if (!id) {
            set(deployments.ids()![0], "deployment");
          } else {
            set(id, "build");
          }
        } else {
          const [type, id] = location.pathname.split("/").filter((val) => val);
          if (type !== selected().type || id !== selected().id) {
            history.replaceState(
              { id, type },
              "",
              `${selected().type}/${selected().id}`
            );
          }
        }
        setFirstLoad(false);
      }
    }
  });

  const popstate = (e: any) => {
    setSelected({ id: e.state.id, type: e.state.type });
  };

  window.addEventListener("popstate", popstate);

  onCleanup(() => window.removeEventListener("popstate", popstate));

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

export function useDeployments() {
  const deployments = useCollection(getDeployments);
  const state = (id: string) =>
    deployments.get(id)!.status === "not deployed"
      ? "not deployed"
      : (deployments.get(id)!.status as ContainerStatus).State;
  const status = (id: string) =>
    deployments.get(id)!.status === "not deployed"
      ? undefined
      : (deployments.get(id)!.status as ContainerStatus).Status.toLowerCase();
  return {
    ...deployments,
    status,
    state,
  }
}

export function useUpdates(query?: Parameters<typeof getUpdates>[0]) {
  const updates = useArray(() => getUpdates(query));
  const [noMore, setNoMore] = createSignal(false);
  const loadMore = async () => {
    const offset = updates.collection()?.length;
    if (offset) {
      const newUpdates = await getUpdates({ offset });
      updates.addManyToEnd(newUpdates);
      if (newUpdates.length !== 10) {
        setNoMore(true);
      }
    }
  }
  return {
    noMore,
    loadMore,
    ...updates
  }
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
