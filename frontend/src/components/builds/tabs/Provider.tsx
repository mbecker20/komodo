import { Build, Update } from "@monitor/types";
import { Component, createContext, createEffect, createResource, onCleanup, Resource, useContext } from "solid-js";
import { createStore, DeepReadonly, SetStoreFunction } from "solid-js/store";
import { ADD_UPDATE, UPDATE_BUILD } from "../../../state/actions";
import { useAppState } from "../../../state/StateProvider";
import { getBuild, getDockerAccounts } from "../../../util/query";

type ConfigBuild = Build & { loaded: boolean; updated: boolean; saving: boolean };

type State = {
  build: DeepReadonly<ConfigBuild>;
  setBuild: SetStoreFunction<ConfigBuild>;
  reset: () => void;
  save: () => void;
};

const context = createContext<State>();

export const ConfigProvider: Component<{ build: Build }> = (p) => {
  const { ws } = useAppState();
  const [build, set] = createStore({
    ...p.build,
    loaded: false,
    updated: false,
    saving: false,
  });
  const setBuild: SetStoreFunction<ConfigBuild> = (...args: any) => {
    // @ts-ignore
    set(...args);
    set("updated", true);
  };

  const load = () => {
    console.log("load server");
    getBuild(p.build._id!).then((build) => {
      set({
        ...build,
        repo: build.repo,
        branch: build.branch,
        onClone: build.onClone,
        dockerBuildArgs: build.dockerBuildArgs,
        cliBuild: build.cliBuild,
        dockerAccount: build.dockerAccount,
        loaded: true,
        updated: false,
        saving: false
      });
    });
  };
  createEffect(load);

  const save = () => {
    setBuild("saving", true);
    ws.send(UPDATE_BUILD, { build });
  };

  const unsub = ws.subscribe(
    [ADD_UPDATE],
    ({ update }: { update: Update }) => {
			if (update.buildID === p.build._id) {
				if ([UPDATE_BUILD].includes(update.operation)) {
					load();
				}
			}
		}
  );
	onCleanup(unsub);

  const state = {
    build,
    setBuild,
    reset: load,
    save,
  };
  return <context.Provider value={state}>{p.children}</context.Provider>;
};

export function useConfig() {
  return useContext(context) as State;
}
