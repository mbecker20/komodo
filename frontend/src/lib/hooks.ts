import { client } from "@main";
import { Types } from "@monitor/client";
import {
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

export const useResourceParamType = () => {
  const type = useParams().type;
  if (!type) return undefined as unknown as UsableResource;
  return (type[0].toUpperCase() + type.slice(1, -1)) as UsableResource;
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
) => useQuery([type, params], () => client.read({ type, params } as R), config);

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
  useMutation([type], (params: P) => client.write({ type, params } as R), {
    ...config,
    onError: (e, v, c) => {
      config?.onError && config.onError(e, v, c);
    },
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
  useMutation([type], (params: P) => client.execute({ type, params } as R), {
    ...config,
    onError: (e, v, c) => {
      // toast({ title: `Error - ${type} Failed`, intent: "danger" });
      config?.onError && config.onError(e, v, c);
    },
  });

export const useInvalidate = () => {
  const qc = useQueryClient();

  return <
    T extends Types.ReadRequest["type"],
    P = Extract<Types.ReadRequest, { type: T }>["params"]
  >(
    ...keys: Array<[T] | [T, P]>
  ) => keys.forEach((k) => qc.invalidateQueries([...k]));
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
