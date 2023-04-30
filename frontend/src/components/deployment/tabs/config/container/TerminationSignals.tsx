import { Component, For, Show, createSignal } from "solid-js";
import { useConfig } from "../Provider";
import Grid from "../../../../shared/layout/Grid";
import Flex from "../../../../shared/layout/Flex";
import Icon from "../../../../shared/Icon";
import { TERM_SIGNALS } from "../../../Deployment";
import { TerminationSignal } from "../../../../../types";
import Input from "../../../../shared/Input";
import Menu from "../../../../shared/menu/Menu";

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
        <Show when={userCanUpdate()}>
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
