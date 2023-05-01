import {
  Component,
  Match,
  Setter,
  Show,
  Signal,
  Switch,
  createSignal,
} from "solid-js";
import { client } from "../..";
import { useAppState } from "../../state/StateProvider";
import { useUser } from "../../state/UserProvider";
import ConfirmButton from "../shared/ConfirmButton";
import Icon from "../shared/Icon";
import Flex from "../shared/layout/Flex";
import Grid from "../shared/layout/Grid";
import Loading from "../shared/loading/Loading";
import HoverMenu from "../shared/menu/HoverMenu";
import { useActionStates } from "./ActionStateProvider";
import { combineClasses } from "../../util/helpers";
import { A, useParams } from "@solidjs/router";
import {
  DockerContainerState,
  Operation,
  PermissionLevel,
  ServerStatus,
  TerminationSignal,
  TerminationSignalLabel,
  UpdateStatus,
} from "../../types";
import ConfirmMenuButton from "../shared/ConfirmMenuButton";
import Selector from "../shared/menu/Selector";

const Actions: Component<{}> = (p) => {
  const { deployments, builds, servers, getPermissionOnDeployment } =
    useAppState();
  const params = useParams();
  const { user, user_id } = useUser();
  const deployment = () => deployments.get(params.id)!;
  const server = () =>
    deployment() && servers.get(deployment()!.deployment.server_id);
  const show = () => {
    const permissions = getPermissionOnDeployment(params.id);
    return (
      server()?.status === ServerStatus.Ok &&
      deployment() &&
      (user().admin ||
        permissions === PermissionLevel.Execute ||
        permissions === PermissionLevel.Update)
    );
  };
  const showBuild = () => {
    const build = deployment().deployment.build_id
      ? builds.get(deployment().deployment.build_id!)
      : undefined;
    if (build !== undefined) {
      const permissions = build.permissions![user_id()];
      return (
        user().admin ||
        permissions === PermissionLevel.Execute ||
        permissions === PermissionLevel.Update
      );
    } else {
      return false;
    }
  };
  return (
    <Show when={show()}>
      <Grid class={combineClasses("card shadow")} gridTemplateRows="auto 1fr">
        <h1>actions</h1>
        <Grid style={{ height: "fit-content" }}>
          <Show when={showBuild()}>
            <Build />
          </Show>
          <Switch>
            <Match when={deployment().state === DockerContainerState.Running}>
              <Flex class={combineClasses("action shadow")}>
                deploy{" "}
                <Flex>
                  <Deploy redeploy />
                  <Stop />
                  <RemoveContainer />
                </Flex>
              </Flex>
            </Match>

            <Match
              when={
                deployment().state === DockerContainerState.Exited ||
                deployment().state === DockerContainerState.Created
              }
            >
              <Flex class={combineClasses("action shadow")}>
                deploy{" "}
                <Flex>
                  <Deploy redeploy />
                  <Start />
                  <RemoveContainer />
                </Flex>
              </Flex>
            </Match>
            <Match
              when={deployment().state === DockerContainerState.Restarting}
            >
              <Flex class={combineClasses("action shadow")}>
                deploy{" "}
                <Flex>
                  <Deploy redeploy />
                  <Stop />
                  <RemoveContainer />
                </Flex>
              </Flex>
              {/* <Flex class="action shadow">
              container <Start />
            </Flex> */}
            </Match>

            <Match
              when={deployment().state === DockerContainerState.NotDeployed}
            >
              <Flex class={combineClasses("action shadow")}>
                deploy <Deploy />
              </Flex>
            </Match>
          </Switch>
          <Show when={deployment().deployment.repo}>
            <Flex class={combineClasses("action shadow")}>
              frontend
              <Flex>
                <Reclone />
                <Pull />
              </Flex>
            </Flex>
          </Show>
        </Grid>
      </Grid>
    </Show>
  );
};

const Build: Component = () => {
  const { deployments } = useAppState();
  const params = useParams();
  const actions = useActionStates();
  const buildID = () => deployments.get(params.id)!.deployment.build_id!;
  return (
    <Flex class={combineClasses("action shadow")}>
      <A
        href={`/build/${buildID()}`}
        class="pointer"
        style={{ padding: 0, "font-size": "16px" }}
      >
        build
      </A>
      <Show
        when={!actions.building}
        fallback={
          <button class="green">
            <Loading type="spinner" />
          </button>
        }
      >
        <ConfirmButton
          class="green"
          onConfirm={() => {
            client.build(buildID());
          }}
        >
          <Icon type="build" />
        </ConfirmButton>
      </Show>
    </Flex>
  );
};

const Deploy: Component<{ redeploy?: boolean }> = (p) => {
  const params = useParams();
  const actions = useActionStates();
  const { deployments } = useAppState();
  const deployment = () => deployments.get(params.id);
  const name = () => deployment()?.deployment.name;
  const [termSignalLabel, setTermSignalLabel] = useTermSignalLabel();
  return (
    <Show
      when={!actions.deploying}
      fallback={
        <button class="green">
          <Loading type="spinner" />
        </button>
      }
    >
      <HoverMenu
        target={
          <Show
            when={p.redeploy}
            fallback={
              <ConfirmButton
                class="green"
                onConfirm={() => {
                  client.deploy_container(params.id);
                }}
              >
                <Icon type={"play"} />
              </ConfirmButton>
            }
          >
            <ConfirmMenuButton
              class="green"
              onConfirm={() => {
                client.deploy_container(params.id, {
                  stop_signal: ((termSignalLabel().signal as any) === "default"
                    ? undefined
                    : termSignalLabel().signal) as TerminationSignal,
                });
              }}
              title="redeploy container"
              configs={
                <TermSignalSelector
                  termSignalLabel={termSignalLabel()}
                  setTermSignalLabel={setTermSignalLabel}
                />
              }
              match={name()!}
            >
              <Icon type={"reset"} />
            </ConfirmMenuButton>
          </Show>
        }
        content={p.redeploy ? "redeploy container" : "deploy container"}
        position="bottom center"
        padding="0.5rem"
      />
    </Show>
  );
};

const RemoveContainer = () => {
  const params = useParams();
  const actions = useActionStates();
  const { deployments } = useAppState();
  const name = () => deployments.get(params.id)?.deployment.name;
  const [termSignalLabel, setTermSignalLabel] = useTermSignalLabel();
  return (
    <Show
      when={!actions.removing}
      fallback={
        <button class="red">
          <Loading type="spinner" />
        </button>
      }
    >
      <HoverMenu
        target={
          <ConfirmMenuButton
            class="red"
            onConfirm={() => {
              client.remove_container(params.id, {
                stop_signal: ((termSignalLabel().signal as any) === "default"
                  ? undefined
                  : termSignalLabel().signal) as TerminationSignal,
              });
            }}
            title="destroy container"
            configs={
              <TermSignalSelector
                termSignalLabel={termSignalLabel()}
                setTermSignalLabel={setTermSignalLabel}
              />
            }
            match={name()!}
          >
            <Icon type="trash" />
          </ConfirmMenuButton>
        }
        content="delete container"
        position="bottom center"
        padding="0.5rem"
      />
    </Show>
  );
};

const Start = () => {
  const params = useParams();
  const actions = useActionStates();
  return (
    <Show
      when={!actions.starting}
      fallback={
        <button class="green">
          <Loading type="spinner" />
        </button>
      }
    >
      <HoverMenu
        target={
          <ConfirmButton
            class="green"
            onConfirm={() => {
              client.start_container(params.id);
            }}
          >
            <Icon type="play" />
          </ConfirmButton>
        }
        content="start container"
        position="bottom center"
        padding="0.5rem"
      />
    </Show>
  );
};

const Stop = () => {
  const params = useParams();
  const actions = useActionStates();
  const { deployments } = useAppState();
  const name = () => deployments.get(params.id)?.deployment.name;
  const [termSignalLabel, setTermSignalLabel] = useTermSignalLabel();
  return (
    <Show
      when={!actions.stopping}
      fallback={
        <button class="orange">
          <Loading type="spinner" />
        </button>
      }
    >
      <HoverMenu
        target={
          <ConfirmMenuButton
            class="orange"
            onConfirm={() => {
              client.stop_container(params.id, {
                stop_signal: termSignalLabel().signal,
              });
            }}
            title="stop container"
            configs={
              <TermSignalSelector
                termSignalLabel={termSignalLabel()}
                setTermSignalLabel={setTermSignalLabel}
              />
            }
            match={name()!}
          >
            <Icon type="pause" />
          </ConfirmMenuButton>
        }
        content="stop container"
        position="bottom center"
        padding="0.5rem"
      />
    </Show>
  );
};

const Pull = () => {
  const params = useParams();
  const actions = useActionStates();
  return (
    <Show
      when={!actions.pulling}
      fallback={
        <button class="blue">
          <Loading type="spinner" />
        </button>
      }
    >
      <HoverMenu
        target={
          <ConfirmButton
            class="blue"
            onConfirm={() => {
              client.pull_deployment(params.id);
            }}
          >
            <Icon type="arrow-down" />
          </ConfirmButton>
        }
        content="pull"
        position="bottom center"
        padding="0.5rem"
      />
    </Show>
  );
};

const Reclone = () => {
  const params = useParams();
  const actions = useActionStates();
  return (
    <Show
      when={!actions.recloning}
      fallback={
        <button class="orange">
          <Loading type="spinner" />
        </button>
      }
    >
      <HoverMenu
        target={
          <ConfirmButton
            class="orange"
            onConfirm={() => {
              client.reclone_deployment(params.id);
            }}
          >
            <Icon type="reset" />
          </ConfirmButton>
        }
        content="reclone"
        position="bottom center"
        padding="0.5rem"
      />
    </Show>
  );
};

const TermSignalSelector: Component<{
  termSignalLabel: TerminationSignalLabel;
  setTermSignalLabel: Setter<TerminationSignalLabel>;
}> = (p) => {
  const params = useParams();
  const { deployments } = useAppState();
  const deployment = () => deployments.get(params.id);
  return (
    <Show
      when={
        deployment()?.state === DockerContainerState.Running &&
        (deployment()?.deployment.term_signal_labels?.length || 0) > 1
      }
    >
      <Flex
        class="full-width wrap"
        justifyContent="space-between"
        alignItems="center"
      >
        <div class="dimmed">termination signal: </div>
        <Selector
          targetClass="blue"
          selected={p.termSignalLabel}
          items={deployment()?.deployment.term_signal_labels || []}
          itemMap={({ signal, label }) => (
            <Flex gap="0.5rem" alignItems="center">
              <div>{signal}</div>
              <Show when={label.length > 0}>
                <div class="dimmed">{label}</div>
              </Show>
            </Flex>
          )}
          onSelect={(signal) => p.setTermSignalLabel(signal)}
          position="bottom right"
        />
      </Flex>
    </Show>
  );
};

function useTermSignalLabel(): Signal<TerminationSignalLabel> {
  const params = useParams();
  const { deployments, ws } = useAppState();
  const deployment = () => deployments.get(params.id)?.deployment;
  const term_signal = () =>
    deployment()?.termination_signal || TerminationSignal.SigTerm;
  const default_term_signal_label = () => ({
    signal: term_signal(),
    label:
      deployment()?.term_signal_labels?.find(
        ({ signal }) => signal === term_signal()
      )?.label || "",
  });
  const [label, setLabel] = createSignal<TerminationSignalLabel>(
    default_term_signal_label()
  );
  ws.subscribe([Operation.UpdateDeployment], (update) => {
    if (update.status === UpdateStatus.Complete) {
      setTimeout(() => setLabel(default_term_signal_label()), 100);
    }
  });
  return [label, setLabel];
}

export default Actions;
