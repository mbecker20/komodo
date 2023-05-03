import { useParams } from "@solidjs/router";
import {
  Accessor,
  createContext,
  createEffect,
  createResource,
  createSignal,
  onCleanup,
  ParentComponent,
  Resource,
  useContext,
} from "solid-js";
import { createStore, SetStoreFunction } from "solid-js/store";
import { client, pushNotification } from "../../../..";
import { useAppState } from "../../../../state/StateProvider";
import { useUser } from "../../../../state/UserProvider";
import {
  Deployment,
  Operation,
  PermissionLevel,
  ServerStatus,
  ServerWithStatus,
} from "../../../../types";
import { getId } from "../../../../util/helpers";

type ConfigDeployment = Deployment & {
  loaded: boolean;
  updated: boolean;
  updating: boolean;
};

type State = {
  editing: Accessor<boolean>;
  deployment: ConfigDeployment;
  setDeployment: SetStoreFunction<ConfigDeployment>;
  server: () => ServerWithStatus | undefined;
  reset: () => void;
  save: () => void;
  networks: Resource<any[]>;
  userCanUpdate: () => boolean;
};

const context = createContext<State>();

export const ConfigProvider: ParentComponent<{}> = (p) => {
  const { ws, deployments, servers } = useAppState();
  const params = useParams();
  const { user } = useUser();
  const [editing] = createSignal(false);
  const [deployment, set] = createStore({
    ...deployments.get(params.id)!.deployment,
    loaded: false,
    updated: false,
    updating: false,
  });
  const setDeployment = (...args: any) => {
    // @ts-ignore
    set(...args);
    set("updated", true);
  };
  const load = () => {
    // console.log("loading deployment");
    client.get_deployment(params.id).then((deployment) =>
      set({
        name: deployment.deployment.name,
        server_id: deployment.deployment.server_id,
        permissions: deployment.deployment.permissions,
        docker_run_args: {
          image: deployment.deployment.docker_run_args.image,
          ports: deployment.deployment.docker_run_args.ports,
          volumes: deployment.deployment.docker_run_args.volumes,
          environment: deployment.deployment.docker_run_args.environment,
          network: deployment.deployment.docker_run_args.network,
          restart: deployment.deployment.docker_run_args.restart,
          post_image: deployment.deployment.docker_run_args.post_image,
          container_user: deployment.deployment.docker_run_args.container_user,
          extra_args: deployment.deployment.docker_run_args.extra_args,
          docker_account: deployment.deployment.docker_run_args.docker_account,
        },
        build_id: deployment.deployment.build_id,
        build_version: deployment.deployment.build_version,
        repo: deployment.deployment.repo,
        branch: deployment.deployment.branch,
        github_account: deployment.deployment.github_account,
        on_clone: deployment.deployment.on_clone,
        on_pull: deployment.deployment.on_pull,
        repo_mount: deployment.deployment.repo_mount,
        created_at: deployment.deployment.created_at,
        updated_at: deployment.deployment.updated_at,
        redeploy_on_build: deployment.deployment.redeploy_on_build,
        term_signal_labels: deployment.deployment.term_signal_labels,
        termination_signal: deployment.deployment.termination_signal,
        termination_timeout: deployment.deployment.termination_timeout,
        loaded: true,
        updated: false,
        updating: false,
      })
    );
  };
  createEffect(load);

  const server = () =>
    servers.get(deployments.get(params.id)!.deployment.server_id);

  const [networks] = createResource(() => {
    if (server()?.status === ServerStatus.Ok) {
      return client.get_docker_networks(
        deployments.get(params.id)!.deployment.server_id
      );
    } else return [];
  });

  const save = () => {
    setDeployment("updating", true);
    client.update_deployment(deployment).catch((e) => {
      console.error(e);
      pushNotification("bad", "update deployment failed");
      setDeployment("updating", false);
    });
  };

  let update_unsub = () => {};

  createEffect(() => {
    update_unsub();
    update_unsub = ws.subscribe([Operation.UpdateDeployment], (update) => {
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
          const dep = await client.get_deployment(params.id);
          set("permissions", dep.deployment.permissions);
        }
      }
    );
  });

  onCleanup(() => modify_unsub());

  const userCanUpdate = () =>
    user().admin ||
    deployment.permissions![getId(user())] === PermissionLevel.Update;

  const state = {
    editing,
    deployment,
    setDeployment,
    server,
    reset: load,
    save,
    networks,
    userCanUpdate,
  };

  return <context.Provider value={state}>{p.children}</context.Provider>;
};

export function useConfig() {
  return useContext(context) as State;
}
