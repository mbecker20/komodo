import { useParams } from "@solidjs/router";
import {
  createContext,
  createEffect,
  onCleanup,
  ParentComponent,
  useContext,
} from "solid-js";
import { createStore, SetStoreFunction } from "solid-js/store";
import { client, pushNotification } from "../../..";
import { useAppState } from "../../../state/StateProvider";
import { useUser } from "../../../state/UserProvider";
import { Build, Operation, PermissionLevel, ServerWithStatus } from "../../../types";
import { getId } from "../../../util/helpers";

type ConfigBuild = Build & {
  loaded: boolean;
  updated: boolean;
  saving: boolean;
};

type State = {
  build: ConfigBuild;
  setBuild: SetStoreFunction<ConfigBuild>;
  server: () => ServerWithStatus | undefined
  reset: () => void;
  save: () => void;
  userCanUpdate: () => boolean;
};

const context = createContext<State>();

export const ConfigProvider: ParentComponent<{}> = (p) => {
  const { ws, builds, servers } = useAppState();
  const params = useParams();
  const { user } = useUser();
  const [build, set] = createStore({
    ...builds.get(params.id)!,
    loaded: false,
    updated: false,
    saving: false,
  });
  const setBuild: SetStoreFunction<ConfigBuild> = (...args: any) => {
    // @ts-ignore
    set(...args);
    set("updated", true);
  };
  const server = () =>
    builds.get(params.id)?.server_id
      ? servers.get(builds.get(params.id)!.server_id!)
      : undefined;

  const load = () => {
    // console.log("load build");
    client.get_build(params.id).then((build) => {
      set({
        ...build,
        _id: { $oid: params.id } as any,
        repo: build.repo,
        branch: build.branch,
        pre_build: build.pre_build,
        docker_build_args: {
          build_path: build.docker_build_args?.build_path!,
          dockerfile_path: build.docker_build_args?.dockerfile_path,
          build_args: build.docker_build_args?.build_args,
          extra_args: build.docker_build_args?.extra_args,
          use_buildx: build.docker_build_args?.use_buildx,
        },
        docker_account: build.docker_account,
        github_account: build.github_account,
        aws_config: build.aws_config,
        loaded: true,
        updated: false,
        saving: false,
      });
    });
  };
  createEffect(load);

  const save = () => {
    setBuild("saving", true);
    client
      .update_build(build)
      .catch((e) => {
        console.error(e);
        pushNotification("bad", "update build failed");
        setBuild("saving", false);
      });
  };

  let update_unsub = () => {};

  createEffect(() => {
    update_unsub();
    update_unsub = ws.subscribe([Operation.UpdateBuild], (update) => {
      if (update.target.id === params.id) {
        load();
      }
    });
  });

  onCleanup(() => update_unsub());

  let modify_unsub = () => {};

  createEffect(() => {
    modify_unsub();
    modify_unsub = ws.subscribe(
      [Operation.ModifyUserPermissions],
      async (update) => {
        if (update.target.id === params.id) {
          const build = await client.get_build(params.id);
          set("permissions", build.permissions);
        }
      }
    );
  });

  onCleanup(() => modify_unsub());

  const userCanUpdate = () => user().admin || build.permissions![getId(user())] === PermissionLevel.Update;

  const state = {
    build,
    setBuild,
    server,
    reset: load,
    save,
    userCanUpdate,
  };
  return <context.Provider value={state}>{p.children}</context.Provider>;
};

export function useConfig() {
  return useContext(context) as State;
}
