import {
  Component,
  createContext,
  createMemo,
  createSignal,
  useContext,
} from "solid-js";
import { useAppState } from "../../../state/StateProvider";
import { useLocalStorage, useWindowKeyDown } from "../../../util/hooks";

const value = () => {
  const { deployments, builds, servers, selected } = useAppState();
  const [search, setSearch] = createSignal("");
  const [open, setOpen] = createSignal(false);
  const close = (inputRef: HTMLInputElement | undefined) => {
    inputRef?.blur();
    setSearch("");
    setOpen(false);
  };
  const [highlighted, setHighlighted] = createSignal(0);

  const filteredDeployments = createMemo(
    () =>
      deployments.filterArray(
        (deployment) =>
          deployment.name.toLowerCase().includes(search().toLowerCase()) ||
          servers
            .get(deployment.serverID!)!
            .name.toLowerCase()
            .includes(search().toLowerCase())
      )!
  );
  const filteredBuilds = createMemo(
    () =>
      builds.filterArray((build) =>
        build.name.toLowerCase().includes(search().toLowerCase())
      )!
  );
  const filteredServers = createMemo(
    () =>
      servers.filterArray((server) =>
        server.name.toLowerCase().includes(search().toLowerCase())
      )!
  );

  const inputOnKeyDown =
    (inputRef: HTMLInputElement | undefined) => (e: any) => {
      if (e.key === "ArrowDown") {
        e.preventDefault();
        setHighlighted((h) => {
          const greaterThan =
            h + 1 >
            (selectedTab() === "deployments"
              ? filteredDeployments()?.length
              : selectedTab() === "builds"
              ? filteredBuilds()?.length
              : selectedTab() === "servers"
              ? filteredServers()?.length
              : 1) -
              1;
          return greaterThan ? 0 : h + 1;
        });
      } else if (e.key === "ArrowUp") {
        e.preventDefault();
        setHighlighted((h) => {
          if (h - 1 < 0) {
            return (
              (selectedTab() === "deployments"
                ? filteredDeployments()?.length
                : selectedTab() === "builds"
                ? filteredBuilds()?.length
                : selectedTab() === "servers"
                ? filteredServers()?.length
                : 1) - 1
            );
          } else {
            return h - 1;
          }
        });
      } else if (e.key === "ArrowRight") {
        e.preventDefault();
        switch (selectedTab()) {
          case "deployments":
            setSelectedTab("builds");
            break;

          case "builds":
            setSelectedTab("servers");
            break;

          case "servers":
            setSelectedTab("deployments");
            break;
        }
        setHighlighted(0);
      } else if (e.key === "ArrowLeft") {
        e.preventDefault();
        switch (selectedTab()) {
          case "deployments":
            setSelectedTab("servers");
            break;

          case "builds":
            setSelectedTab("deployments");
            break;

          case "servers":
            setSelectedTab("builds");
            break;
        }
        setHighlighted(0);
      } else if (e.key === "Enter") {
        switch (selectedTab()) {
          case "deployments":
            selected.set(
              filteredDeployments()![highlighted()]._id!,
              "deployment"
            );
            break;
          case "builds":
            selected.set(filteredBuilds()![highlighted()]._id!, "build");
            break;
          case "servers":
            selected.set(filteredServers()![highlighted()]._id!, "server");
            break;
        }
        close(inputRef);
      } else if (e.key === "Escape") {
        close(inputRef);
      }
    };

  const [selectedTab, setSelectedTab] = useLocalStorage(
    "deployments",
    "search-tab"
  );

  useWindowKeyDown((e) => {
    if (open()) {
      if (e.key === "ArrowRight") {
        switch (selectedTab()) {
          case "deployments":
            setSelectedTab("builds");
            break;

          case "builds":
            setSelectedTab("servers");
            break;

          case "servers":
            setSelectedTab("deployments");
            break;
        }
        setHighlighted(0);
      } else if (e.key === "ArrowLeft") {
        switch (selectedTab()) {
          case "deployments":
            setSelectedTab("servers");
            break;

          case "builds":
            setSelectedTab("deployments");
            break;

          case "servers":
            setSelectedTab("builds");
            break;
        }
        setHighlighted(0);
      } else if (e.key === "ArrowDown") {
        setHighlighted((h) => {
          const greaterThan =
            h + 1 >
            (selectedTab() === "deployments"
              ? filteredDeployments()?.length
              : selectedTab() === "builds"
              ? filteredBuilds()?.length
              : selectedTab() === "servers"
              ? filteredServers()?.length
              : 1) -
              1;
          return greaterThan ? 0 : h + 1;
        });
      } else if (e.key === "ArrowUp") {
        setHighlighted((h) => {
          if (h - 1 < 0) {
            return (
              (selectedTab() === "deployments"
                ? filteredDeployments()?.length
                : selectedTab() === "builds"
                ? filteredBuilds()?.length
                : selectedTab() === "servers"
                ? filteredServers()?.length
                : 1) - 1
            );
          } else {
            return h - 1;
          }
        });
      } else if (e.key === "Enter") {
        switch (selectedTab()) {
          case "deployments":
            selected.set(
              filteredDeployments()![highlighted()]._id!,
              "deployment"
            );
            break;
          case "builds":
            selected.set(filteredBuilds()![highlighted()]._id!, "build");
            break;
          case "servers":
            selected.set(filteredServers()![highlighted()]._id!, "server");
            break;
        }
        close(undefined);
      }
    }
  });

  const value = {
    search: {
      value: search,
      set: setSearch,
    },
    open: {
      value: open,
      set: setOpen,
      close,
    },
    highlighted: {
      value: highlighted,
      set: setHighlighted,
    },
    filteredDeployments,
    filteredBuilds,
    filteredServers,
    input: {
      onKeyDown: inputOnKeyDown,
      onEdit: (val: string) => {
        setSearch(val);
        setHighlighted(0);
      },
    },
    tab: {
      selected: selectedTab,
      set: setSelectedTab,
    },
  };
  return value;
};

export type Value = ReturnType<typeof value>;

const context = createContext<Value>();

export const SearchProvider: Component<{}> = (p) => {
  const val = value();
  return <context.Provider value={val}>{p.children}</context.Provider>;
};

export function useSearchState() {
  return useContext(context) as Value;
}
