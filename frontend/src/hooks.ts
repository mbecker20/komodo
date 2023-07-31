import { Types } from "@monitor/client";
import { client } from "./main";
import {
  useQuery,
  useMutation,
  UseQueryOptions,
  UseMutationOptions,
} from "@tanstack/react-query";
import { useAtomValue, useSetAtom } from "jotai";
import { atomWithStorage } from "jotai/utils";
import { useNavigate } from "react-router-dom";
import {
  ExecuteResponses,
  ReadResponses,
  WriteResponses,
} from "@monitor/client/dist/responses";
import { useEffect, useState } from "react";

export const useRead = <
  T extends Types.ReadRequest["type"],
  P = Extract<Types.ReadRequest, { type: T }>["params"]
>(
  type: T,
  params: P,
  config?: Omit<
    UseQueryOptions<ReadResponses[T], unknown, ReadResponses[T], (T | P)[]>,
    "initialData" | "queryFn" | "queryKey"
  >
) =>
  useQuery([type, params], () => client.read({ type, params } as any), config);

export const useWrite = <
  T extends Types.WriteRequest["type"],
  P = Extract<Types.WriteRequest, { type: T }>["params"]
>(
  type: T,
  config?: Omit<
    UseMutationOptions<WriteResponses[T], unknown, P, unknown>,
    "mutationKey" | "mutationFn"
  >
) =>
  useMutation(
    [type],
    (params: P) => client.write({ type, params } as any),
    config
  );

export const useExecute = <
  T extends Types.ExecuteRequest["type"],
  P = Extract<Types.ExecuteRequest, { type: T }>["params"]
>(
  type: T,
  config?: Omit<
    UseMutationOptions<ExecuteResponses[T], unknown, P, unknown>,
    "mutationKey" | "mutationFn"
  >
) =>
  useMutation(
    [type],
    (params: P) => client.execute({ type, params } as any),
    config
  );

export const useUser = () => useRead("GetUser", {});

export const useLogin = () => {
  const { refetch } = useUser();
  const nav = useNavigate();

  return useMutation(client.login, {
    onSuccess: async (jwt) => {
      await refetch();
      localStorage.setItem("monitor-auth-token", jwt ?? "");
      nav("/");
    },
  });
};

const recently_viewed = atomWithStorage<
  { type: "Deployment" | "Build" | "Server"; id: string }[]
>("recently-viewed", []);

export const useGetRecentlyViewed = () => useAtomValue(recently_viewed);

export const useSetRecentlyViewed = () => {
  const set = useSetAtom(recently_viewed);
  const push = (type: "Deployment" | "Build" | "Server", id: string) =>
    set((res) => [{ type, id }, ...res.filter((r) => r.id !== id)].slice(0, 5));
  return push;
};

export const useServerStats = (server_id: string) => {
  const [stats, set] = useState<Types.AllSystemStats>();

  const fetch = () =>
    client
      .read({
        type: "GetAllSystemStats",
        params: { server_id },
      })
      .then(set);

  useEffect(() => {
    fetch();
    const handle = setInterval(() => {
      fetch();
    }, 1000);
    return () => {
      clearInterval(handle);
    };
  }, []);

  return stats;
};
