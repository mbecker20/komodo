import { Accessor, createSignal } from "solid-js";

export function useToggle(initial = false): [Accessor<boolean>, () => void] {
  const [s, set] = createSignal(initial);
  const toggle = () => set((s) => !s);
  return [s, toggle];
}
