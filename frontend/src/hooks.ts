import { Types } from "@monitor/client";
import { client } from "./main";
import {
  useQuery,
  useMutation,
  UseMutationOptions,
} from "@tanstack/react-query";
import { useAtomValue, useSetAtom } from "jotai";
import { atomWithStorage } from "jotai/utils";
import { useNavigate } from "react-router-dom";
import {
  ExecuteResponses,
  WriteResponses,
} from "@monitor/client/dist/responses";

export const useRead = <T extends Types.ReadRequest>(req: T) =>
  useQuery([req], () => client.read(req));

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
    async (params: P) =>
      (await client.write({ type, params } as any)) as WriteResponses[T],
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
    async (params: P) =>
      (await client.execute({ type, params } as any)) as ExecuteResponses[T],
    config
  );

export const useUser = () => useRead({ type: "GetUser", params: {} });

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
