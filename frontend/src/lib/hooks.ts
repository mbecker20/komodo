import { AUTH_TOKEN_STORAGE_KEY, KOMODO_BASE_URL } from "@main";
import { KomodoClient as Client, Types } from "komodo_client";
import {
  AuthResponses,
  ExecuteResponses,
  ReadResponses,
  UserResponses,
  WriteResponses,
} from "komodo_client/dist/responses";
import {
  UseMutationOptions,
  UseQueryOptions,
  useMutation,
  useQuery,
  useQueryClient,
} from "@tanstack/react-query";
import { UsableResource } from "@types";
import { useToast } from "@ui/use-toast";
import { atom, useAtom } from "jotai";
import { useEffect, useState } from "react";
import { useParams } from "react-router-dom";

// ============== RESOLVER ==============

const token = () => ({
  jwt: localStorage.getItem(AUTH_TOKEN_STORAGE_KEY) ?? "",
});
const client = () => Client(KOMODO_BASE_URL, { type: "jwt", params: token() });

export const useLoginOptions = () =>
  useQuery({
    queryKey: ["GetLoginOptions"],
    queryFn: () => client().auth({ type: "GetLoginOptions", params: {} }),
  });

export const useUser = () => {
  const userInvalidate = useUserInvalidate();
  const query = useQuery({
    queryKey: ["GetUser"],
    queryFn: () => client().auth({ type: "GetUser", params: {} }),
    refetchInterval: 30_000,
  });
  useEffect(() => {
    if (query.data && query.error) {
      userInvalidate();
    }
  }, [query.data, query.error]);
  return query;
};

export const useUserInvalidate = () => {
  const qc = useQueryClient();
  return () => {
    qc.invalidateQueries({ queryKey: ["GetUser"] });
  };
};

export const useRead = <
  T extends Types.ReadRequest["type"],
  R extends Extract<Types.ReadRequest, { type: T }>,
  P extends R["params"],
  C extends Omit<
    UseQueryOptions<
      ReadResponses[R["type"]],
      unknown,
      ReadResponses[R["type"]],
      (T | P)[]
    >,
    "queryFn" | "queryKey"
  >
>(
  type: T,
  params: P,
  config?: C
) =>
  useQuery({
    queryKey: [type, params],
    queryFn: () => client().read({ type, params } as R),
    ...config,
  });

export const useInvalidate = () => {
  const qc = useQueryClient();
  return <
    Type extends Types.ReadRequest["type"],
    Params extends Extract<Types.ReadRequest, { type: Type }>["params"]
  >(
    ...keys: Array<[Type] | [Type, Params]>
  ) => keys.forEach((key) => qc.invalidateQueries({ queryKey: key }));
};

export const useManageUser = <
  T extends Types.UserRequest["type"],
  R extends Extract<Types.UserRequest, { type: T }>,
  P extends R["params"],
  C extends Omit<
    UseMutationOptions<UserResponses[T], unknown, P, unknown>,
    "mutationKey" | "mutationFn"
  >
>(
  type: T,
  config?: C
) => {
  const { toast } = useToast();
  return useMutation({
    mutationKey: [type],
    mutationFn: (params: P) => client().user({ type, params } as R),
    onError: (e: { response: { data: any } }, v, c) => {
      console.log("Auth error:", e.response.data);
      const msg = e.response.data?.error as string;
      let msg_log = msg ? msg + " | " : "";
      if (msg_log) {
        msg_log = msg_log[0].toUpperCase() + msg_log.slice(1);
      }
      toast({
        title: `Request ${type} Failed`,
        description: `${msg_log}See console for details`,
        variant: "destructive",
      });
      config?.onError && config.onError(e, v, c);
    },
    ...config,
  });
};

export const useWrite = <
  T extends Types.WriteRequest["type"],
  R extends Extract<Types.WriteRequest, { type: T }>,
  P extends R["params"],
  C extends Omit<
    UseMutationOptions<WriteResponses[T], unknown, P, unknown>,
    "mutationKey" | "mutationFn"
  >
>(
  type: T,
  config?: C
) => {
  const { toast } = useToast();
  return useMutation({
    mutationKey: [type],
    mutationFn: (params: P) => client().write({ type, params } as R),
    onError: (e: { response: { data: any } }, v, c) => {
      console.log("Write error:", e.response.data);
      const msg = e.response.data?.error;
      let msg_log = msg ? msg + " - " : "";
      if (msg_log) {
        msg_log = msg_log[0].toUpperCase() + msg_log.slice(1);
      }
      toast({
        title: `Write request ${type} failed`,
        description: `${msg_log}See console for details`,
        variant: "destructive",
      });
      config?.onError && config.onError(e, v, c);
    },
    ...config,
  });
};

export const useExecute = <
  T extends Types.ExecuteRequest["type"],
  R extends Extract<Types.ExecuteRequest, { type: T }>,
  P extends R["params"],
  C extends Omit<
    UseMutationOptions<ExecuteResponses[T], unknown, P, unknown>,
    "mutationKey" | "mutationFn"
  >
>(
  type: T,
  config?: C
) => {
  const { toast } = useToast();
  return useMutation({
    mutationKey: [type],
    mutationFn: (params: P) => client().execute({ type, params } as R),
    onError: (e: { response: { data: any } }, v, c) => {
      console.log("Execute error:", e.response.data);
      const msg = e.response.data?.error;
      let msg_log = msg ? msg + " | " : "";
      if (msg_log) {
        msg_log = msg_log[0].toUpperCase() + msg_log.slice(1);
      }
      toast({
        title: `Execute request ${type} failed`,
        description: `${msg_log}See console for details`,
        variant: "destructive",
      });
      config?.onError && config.onError(e, v, c);
    },
    ...config,
  });
};

export const useAuth = <
  T extends Types.AuthRequest["type"],
  R extends Extract<Types.AuthRequest, { type: T }>,
  P extends R["params"],
  C extends Omit<
    UseMutationOptions<AuthResponses[T], unknown, P, unknown>,
    "mutationKey" | "mutationFn"
  >
>(
  type: T,
  config?: C
) =>
  useMutation({
    mutationKey: [type],
    mutationFn: (params: P) => client().auth({ type, params } as R),
    ...config,
  });

// ============== UTILITY ==============

export const useResourceParamType = () => {
  const type = useParams().type;
  if (!type) return undefined;
  if (type === "server-templates") return "ServerTemplate";
  if (type === "resource-syncs") return "ResourceSync";
  return (type[0].toUpperCase() + type.slice(1, -1)) as UsableResource;
};

export const usePushRecentlyViewed = ({ type, id }: Types.ResourceTarget) => {
  const userInvalidate = useUserInvalidate();

  const push = useManageUser("PushRecentlyViewed", {
    onSuccess: userInvalidate,
  }).mutate;

  const exists = useRead(`List${type as UsableResource}s`, {}).data?.find(
    (r) => r.id === id
  )
    ? true
    : false;

  useEffect(() => {
    exists && push({ resource: { type, id } });
  }, [exists, push]);

  return () => push({ resource: { type, id } });
};

export const useSetTitle = (more?: string) => {
  const info = useRead("GetCoreInfo", {}).data;
  const title = more ? `${more} | ${info?.title}` : info?.title;
  useEffect(() => {
    if (title) {
      document.title = title;
    }
  }, [title]);
};

export const atomWithStorage = <T>(key: string, init: T) => {
  const stored = localStorage.getItem(key);
  const inner = atom(stored ? JSON.parse(stored) : init);

  return atom(
    (get) => get(inner),
    (_, set, newValue) => {
      set(inner, newValue);
      localStorage.setItem(key, JSON.stringify(newValue));
    }
  );
};

export const tagsAtom = atomWithStorage<string[]>("tags-v0", []);

export const useTagsFilter = () => {
  const [tags] = useAtom<string[]>(tagsAtom);
  return tags;
};

/** returns function that takes a resource target and checks if it exists */
export const useCheckResourceExists = () => {
  const servers = useRead("ListServers", {}).data;
  const deployments = useRead("ListDeployments", {}).data;
  const builds = useRead("ListBuilds", {}).data;
  const repos = useRead("ListRepos", {}).data;
  const procedures = useRead("ListProcedures", {}).data;
  const builders = useRead("ListBuilders", {}).data;
  const alerters = useRead("ListAlerters", {}).data;
  return (target: Types.ResourceTarget) => {
    switch (target.type) {
      case "Server":
        return servers?.some((resource) => resource.id === target.id) || false;
      case "Deployment":
        return (
          deployments?.some((resource) => resource.id === target.id) || false
        );
      case "Build":
        return builds?.some((resource) => resource.id === target.id) || false;
      case "Repo":
        return repos?.some((resource) => resource.id === target.id) || false;
      case "Procedure":
        return (
          procedures?.some((resource) => resource.id === target.id) || false
        );
      case "Builder":
        return builders?.some((resource) => resource.id === target.id) || false;
      case "Alerter":
        return alerters?.some((resource) => resource.id === target.id) || false;
      default:
        return false;
    }
  };
};

export const useFilterResources = <Info>(
  resources?: Types.ResourceListItem<Info>[],
  search?: string
) => {
  const tags = useTagsFilter();
  const searchSplit = search?.toLowerCase()?.split(" ") || [];
  return (
    resources?.filter(
      (resource) =>
        tags.every((tag: string) => resource.tags.includes(tag)) &&
        (searchSplit.length > 0
          ? searchSplit.every((search) =>
              resource.name.toLowerCase().includes(search)
            )
          : true)
    ) ?? []
  );
};

export type LocalStorageSetter<T> = (state: T) => T;

export const useLocalStorage = <T>(
  key: string,
  init: T
): [T, (state: T | LocalStorageSetter<T>) => void] => {
  const stored = localStorage.getItem(key);
  const parsed = stored ? (JSON.parse(stored) as T) : undefined;
  const [state, inner_set] = useState<T>(parsed ?? init);
  const set = (state: T | LocalStorageSetter<T>) => {
    inner_set((prev_state) => {
      const new_val =
        typeof state === "function"
          ? (state as LocalStorageSetter<T>)(prev_state)
          : state;
      localStorage.setItem(key, JSON.stringify(new_val));
      return new_val;
    });
  };
  return [state, set];
};

export const useKeyListener = (listenKey: string, onPress: () => void) => {
  useEffect(() => {
    const keydown = (e: KeyboardEvent) => {
      // This will ignore Shift + listenKey if it is sent from input / textarea
      const target = e.target as any;
      if (target.matches("input") || target.matches("textarea")) return;

      if (e.key === listenKey) {
        e.preventDefault();
        onPress();
      }
    };
    document.addEventListener("keydown", keydown);
    return () => document.removeEventListener("keydown", keydown);
  });
};

export const useShiftKeyListener = (listenKey: string, onPress: () => void) => {
  useEffect(() => {
    const keydown = (e: KeyboardEvent) => {
      // This will ignore Shift + listenKey if it is sent from input / textarea
      const target = e.target as any;
      if (target.matches("input") || target.matches("textarea")) return;

      if (e.shiftKey && e.key === listenKey) {
        e.preventDefault();
        onPress();
      }
    };
    document.addEventListener("keydown", keydown);
    return () => document.removeEventListener("keydown", keydown);
  });
};

/** Listens for ctrl (or CMD on mac) + the listenKey */
export const useCtrlKeyListener = (listenKey: string, onPress: () => void) => {
  useEffect(() => {
    const keydown = (e: KeyboardEvent) => {
      if ((e.ctrlKey || e.metaKey) && e.key === listenKey) {
        e.preventDefault();
        onPress();
      }
    };
    document.addEventListener("keydown", keydown);
    return () => document.removeEventListener("keydown", keydown);
  });
};

// Returns true if Komodo has no resources.
export const useNoResources = () => {
  const servers =
    useRead("ListServers", {}, { refetchInterval: 5000 }).data?.length ?? 0;
  const deployments =
    useRead("ListDeployments", {}, { refetchInterval: 5000 }).data?.length ?? 0;
  const stacks =
    useRead("ListStacks", {}, { refetchInterval: 5000 }).data?.length ?? 0;
  const builds =
    useRead("ListBuilds", {}, { refetchInterval: 5000 }).data?.length ?? 0;
  const repos =
    useRead("ListRepos", {}, { refetchInterval: 5000 }).data?.length ?? 0;
  const procedures =
    useRead("ListProcedures", {}, { refetchInterval: 5000 }).data?.length ?? 0;
  const builders =
    useRead("ListBuilders", {}, { refetchInterval: 5000 }).data?.length ?? 0;
  const alerters =
    useRead("ListAlerters", {}, { refetchInterval: 5000 }).data?.length ?? 0;
  const templates =
    useRead("ListServerTemplates", {}, { refetchInterval: 5000 }).data
      ?.length ?? 0;
  const syncs =
    useRead("ListResourceSyncs", {}, { refetchInterval: 5000 }).data?.length ??
    0;
  return (
    servers === 0 &&
    deployments === 0 &&
    stacks === 0 &&
    builds === 0 &&
    repos === 0 &&
    procedures === 0 &&
    builders === 0 &&
    alerters === 0 &&
    templates === 0 &&
    syncs === 0
  );
};
