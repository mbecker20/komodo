import { Types } from "@monitor/client";
import { client } from "./main";
import { useQuery, useMutation } from "@tanstack/react-query";
import { useAtomValue, useSetAtom } from "jotai";
import { atomWithStorage } from "jotai/utils";

export const useRead = <T extends Types.ReadRequest>(req: T) =>
  useQuery([req], () => client.read(req));

export const useWrite = <T extends Types.WriteRequest>() =>
  useMutation((req: T) => client.write(req));

export const useExecute = <T extends Types.ExecuteRequest>() =>
  useMutation((req: T) => client.execute(req));

export const useUser = () => useRead({ type: "GetUser", params: {} });

export const useLogin = () =>
  useMutation(client.login, {
    onSuccess: (jwt) => localStorage.setItem("monitor-auth-token", jwt ?? ""),
  });

// const recents_atom = atomWithStorage<
//   { type: "Deployment" | "Build" | "Server"; id: string }[]
// >("recents", []);

const recents_atom = atomWithStorage<
  { type: "Deployment" | "Build" | "Server"; id: string }[]
>("recently-viewed", []);

export const useGetRecentlyViewed = () => useAtomValue(recents_atom);

export const useSetRecentlyViewed = () => {
  const set = useSetAtom(recents_atom);
  const push = (type: "Deployment" | "Build" | "Server", id: string) =>
    set((res) => [{ type, id }, ...res.filter((r) => r.id !== id)].slice(0, 5));
  return push;
};
