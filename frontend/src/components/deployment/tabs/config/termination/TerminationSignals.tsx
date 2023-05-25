import { Component, For, Show, createSignal } from "solid-js";
import { useConfig } from "../Provider";
import Grid from "../../../../shared/layout/Grid";
import Flex from "../../../../shared/layout/Flex";
import Icon from "../../../../shared/Icon";
import { TERM_SIGNALS } from "../../../Deployment";
import { TerminationSignal } from "../../../../../types";
import Input from "../../../../shared/Input";
import Menu from "../../../../shared/menu/Menu";
import Selector from "../../../../shared/menu/Selector";
import { pushNotification } from "../../../../..";

const TerminationSignals: Component<{}> = (p) => {
  const { deployment, setDeployment, userCanUpdate } = useConfig();
  const signals_to_add = () =>
    TERM_SIGNALS.filter(
      (sig) =>
        !deployment.term_signal_labels
          ?.map(({ signal }) => signal)
          .includes(sig)
    );
  const onAdd = (signal: TerminationSignal) => {
    setDeployment("term_signal_labels", (term_signals: any) => [
      ...term_signals,
      { signal, label: "" },
    ]);
  };
  const onRemove = (index: number) => {
    setDeployment("term_signal_labels", (term_signals) =>
      term_signals?.filter((_, i) => i !== index)
    );
  };
  const [menuOpen, setMenuOpen] = createSignal(false);
  return (
    <Grid class="config-item shadow">
      <Flex alignItems="center" justifyContent="space-between">
        <h1>termination signals</h1>
        <Show when={userCanUpdate() && signals_to_add().length > 0}>
          <Menu
            show={menuOpen()}
            close={() => setMenuOpen(false)}
            target={
              <button class="green" onClick={() => setMenuOpen(true)}>
                <Icon type="plus" />
              </button>
            }
            content={
              <For each={signals_to_add()}>
                {(signal) => (
                  <button
                    class="grey"
                    onClick={() => {
                      onAdd(signal);
                      setMenuOpen(false);
                    }}
                  >
                    <h2>{signal}</h2>
                  </button>
                )}
              </For>
            }
          />
        </Show>
      </Flex>
      <Show when={(deployment.term_signal_labels?.length || 0) > 0}>
        <Grid gridTemplateColumns="auto 1fr auto" placeItems="center start">
          <For each={deployment.term_signal_labels}>
            {({ signal, label }, index) => (
              <>
                <h2>{signal}</h2>
                <Input
                  class="full-width"
                  placeholder="label this termination signal"
                  value={label}
                  onConfirm={(value) =>
                    setDeployment("term_signal_labels", index(), "label", value)
                  }
                  disabled={!userCanUpdate()}
                />
                <Show when={userCanUpdate()}>
                  <button class="red" onClick={() => onRemove(index())}>
                    <Icon type="minus" />
                  </button>
                </Show>
              </>
            )}
          </For>
        </Grid>
      </Show>
    </Grid>
  );
};

export default TerminationSignals;

export const DefaultTerminationSignal: Component<{}> = () => {
  const { deployment, setDeployment, userCanUpdate } = useConfig();
  const term_signal = () =>
    deployment.termination_signal || TerminationSignal.SigTerm;
  const selected = () => ({
    signal: term_signal(),
    label:
      deployment.term_signal_labels?.find(
        ({ signal }) => signal === term_signal()
      )?.label || "",
  });
  return (
    <Show when={deployment.term_signal_labels?.length || 0 > 0}>
      <Flex
        class="config-item shadow"
        alignItems="center"
        justifyContent="space-between"
      >
        <h1>default termination signal</h1>
        <Selector
          disabled={!userCanUpdate()}
          targetClass="blue"
          selected={selected()}
          items={deployment.term_signal_labels || []}
          onSelect={({ signal }) => setDeployment("termination_signal", signal)}
          itemMap={({ signal }) => signal}
        />
      </Flex>
    </Show>
  );
};

export const DefaultTerminationTimeout: Component<{}> = () => {
  const { deployment, setDeployment, userCanUpdate } = useConfig();
  return (
    <Flex
      class="config-item shadow"
      alignItems="center"
      justifyContent="space-between"
    >
      <h1>termination timeout</h1>
      <div style={{ position: "relative" }}>
        <Input
          disabled={!userCanUpdate()}
          style={{ width: "10rem" }}
          placeholder="10"
          value={deployment.termination_timeout}
          onConfirm={(value) => {
            const val = Number(value);
            if (!isNaN(val)) {
              setDeployment("termination_timeout", val);
            } else {
              pushNotification("bad", "timeout must be number");
            }
          }}
        />
        <div class="dimmed" style={{ position: "absolute", right: "1rem", top: "50%", transform: "translateY(-50%)" }}>seconds</div>
      </div>
    </Flex>
  );
};
