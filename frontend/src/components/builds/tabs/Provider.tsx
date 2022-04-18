import { Build, Update } from "@monitor/types";
import {
  Component,
  createContext,
  createEffect,
  onCleanup,
  useContext,
} from "solid-js";
import { createStore, DeepReadonly, SetStoreFunction } from "solid-js/store";
import { ADD_UPDATE, BUILD_OWNER_UPDATE, UPDATE_BUILD } from "@monitor/util";
import { useAppState } from "../../../state/StateProvider";
import { useUser } from "../../../state/UserProvider";
import { getBuild } from "../../../util/query";

type ConfigBuild = Build & {
  loaded: boolean;
  updated: boolean;
  saving: boolean;
};

type State = {
  build: DeepReadonly<ConfigBuild>;
  setBuild: SetStoreFunction<ConfigBuild>;
  reset: () => void;
  save: () => void;
  userCanUpdate: () => boolean;
};

const context = createContext<State>();

export const ConfigProvider: Component<{}> = (p) => {
  const { ws, selected, builds } = useAppState();
  const { permissions, username } = useUser();
  const [build, set] = createStore({
    ...builds.get(selected.id())!,
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
    console.log("load build");
    getBuild(selected.id()).then((build) => {
      set({
        ...build,
        repo: build.repo,
        branch: build.branch,
        onClone: build.onClone,
        dockerBuildArgs: build.dockerBuildArgs,
        cliBuild: build.cliBuild,
        dockerAccount: build.dockerAccount,
        githubAccount: build.githubAccount,
        loaded: true,
        updated: false,
        saving: false,
      });
    });
  };
  createEffect(load);

  const save = () => {
    setBuild("saving", true);
    ws.send(UPDATE_BUILD, { build });
  };

  onCleanup(
    ws.subscribe([ADD_UPDATE], ({ update }: { update: Update }) => {
      if (update.buildID === selected.id()) {
        if ([UPDATE_BUILD].includes(update.operation)) {
          load();
        }
      }
    })
  );

  onCleanup(
    ws.subscribe(
      [BUILD_OWNER_UPDATE],
      async ({ buildID }: { buildID: string }) => {
        if (buildID === selected.id()) {
          const build = await getBuild(selected.id());
          set("owners", build.owners);
        }
      }
    )
  );

  const userCanUpdate = () => {
    if (permissions() > 1) {
      return true;
    } else if (permissions() > 0 && build.owners.includes(username()!)) {
      return true;
    } else {
      return false;
    }
  };

  const state = {
    build,
    setBuild,
    reset: load,
    save,
    userCanUpdate,
  };
  return <context.Provider value={state}>{p.children}</context.Provider>;
};

export function useConfig() {
  return useContext(context) as State;
}
