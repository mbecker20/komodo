import { ParentComponent, createContext, useContext } from "solid-js";
import { useAppState } from "../../../state/StateProvider";
import { useLocalStorage } from "../../../util/hooks";

export const TREE_SORTS = ["name", "created"] as const;
export type TreeSortType = typeof TREE_SORTS[number];

const value = () => {
	const { servers, groups } = useAppState();
	const [sort, setSort] = useLocalStorage<TreeSortType>(
    TREE_SORTS[0],
    "home-sort-v1"
  );
	const server_sorter = () => {
    if (!servers.loaded()) return () => 0;
    if (sort() === "name") {
      return (a: string, b: string) => {
        const sa = servers.get(a)!;
        const sb = servers.get(b)!;
        if (sa.server.name < sb.server.name) {
          return -1;
        } else if (sa.server.name > sb.server.name) {
          return 1;
        }
        return 0;
      };
    } else {
      return () => 0;
    }
  };
	const group_sorter = () => {
    if (!groups.loaded) return () => 0;
    if (sort() === "name") {
      return (a: string, b: string) => {
        const ga = groups.get(a)!;
        const gb = groups.get(b)!;
        if (ga.name < gb.name) {
          return -1;
        } else if (ga.name > gb.name) {
          return 1;
        }
        return 0;
      };
    } else {
      return () => 0;
    }
  };
	return {
    sort,
		setSort,
		server_sorter,
		group_sorter,
	};
}

export type Value = ReturnType<typeof value>;

const context = createContext<Value>();

export const TreeProvider: ParentComponent<{}> = (p) => {
	return (
		<context.Provider value={value()}>
			{p.children}
		</context.Provider>
	);
}

export function useTreeState() {
	return useContext(context) as Value;
}