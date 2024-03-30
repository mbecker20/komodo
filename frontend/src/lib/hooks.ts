import { MONITOR_BASE_URL } from "@main";
import { MonitorClient as Client, Types } from "@monitor/client";
import {
  AuthResponses,
  ExecuteResponses,
  ReadResponses,
  WriteResponses,
} from "@monitor/client/dist/responses";
import {
  UseMutationOptions,
  UseQueryOptions,
  useMutation,
  useQuery,
  useQueryClient,
} from "@tanstack/react-query";
import { UsableResource } from "@types";
import { useEffect } from "react";
import { useParams } from "react-router-dom";

// ============== RESOLVER ==============

const token = () => ({ jwt: localStorage.getItem("monitor-auth-token") ?? "" });
const client = () => Client(MONITOR_BASE_URL, { type: "jwt", params: token() });

export const useInvalidate = () => {
  const qc = useQueryClient();
  return <
    Type extends Types.ReadRequest["type"],
    Params extends Extract<Types.ReadRequest, { type: Type }>["params"]
  >(
    ...keys: Array<[Type] | [Type, Params]>
  ) => keys.forEach((key) => qc.invalidateQueries({ queryKey: key }));
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
) =>
  useMutation({
    mutationKey: [type],
    mutationFn: (params: P) => client().write({ type, params } as R),
    ...config,
  });

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
) =>
  useMutation({
    mutationKey: [type],
    mutationFn: (params: P) => client().execute({ type, params } as R),
    ...config,
  });

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
  if (!type) return undefined as unknown as UsableResource;
  return (type[0].toUpperCase() + type.slice(1, -1)) as UsableResource;
};

export const usePushRecentlyViewed = ({ type, id }: Types.ResourceTarget) => {
  const invalidate = useInvalidate();

  const push = useWrite("PushRecentlyViewed", {
    onSuccess: () => invalidate(["GetUser"]),
  }).mutate;

  useEffect(() => {
    !!type && !!id && push({ resource: { type, id } });
  }, [type, id, push]);

  return push;
};