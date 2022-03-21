import { Accessor, createSignal, onCleanup, onMount } from "solid-js";

export function useToggle(initial = false): [Accessor<boolean>, () => void] {
  const [s, set] = createSignal(initial);
  const toggle = () => set((s) => !s);
  return [s, toggle];
}

export function useToggleTimeout(
  timeout = 5000
): [Accessor<boolean>, () => void] {
  const [s, set] = createSignal(false);
  let handle = -1;
  const toggle = () => {
    if (s()) {
      set(false);
      window.clearTimeout(handle);
    } else {
      set(true);
      handle = window.setTimeout(() => set(false), timeout);
    }
  };
  return [s, toggle];
}

export function useLocalStorageToggle(
  initial: boolean,
  key: string
): [Accessor<boolean>, () => void] {
  const [s, set] = useLocalStorage(initial, key);
  const toggle = () => set((s) => !s);
  return [s, toggle];
}

export function useLocalStorage<T>(
  defaultStore: T,
  key: string
): [Accessor<T>, (arg: T | ((s: T) => T)) => void] {
  const toStore = window.localStorage.getItem(key);
  const [stored, setStored] = createSignal<T>(
    toStore ? JSON.parse(toStore) : defaultStore
  );
  const set = (newStore: T | ((s: T) => T)) => {
    if (typeof newStore === "function") {
      const ns = (newStore as (s: T) => T)(stored());
      setStored(() => ns);
      window.localStorage.setItem(key, JSON.stringify(ns));
    } else {
      setStored(() => newStore);
      window.localStorage.setItem(key, JSON.stringify(newStore));
    }
  };
  return [stored, set];
}


export function useInnerWidth(): Accessor<number> {
  const [width, setWidth] = createSignal(window.innerWidth);
  onMount(() => {
    const listener = () => setWidth(window.innerWidth);
    window.addEventListener("resize", listener);
    onCleanup(() => window.removeEventListener("resize", listener));
  })
  return width;
}

export function useWidth(): [
  Accessor<number>,
  (el: HTMLDivElement) => void,
  () => void
] {
  const [width, setWidth] = createSignal(0);
  let ref: HTMLDivElement;
  const setRef = (el: HTMLDivElement) => (ref = el);
  const updateWidth = () => {
    setWidth(ref.clientWidth);
  };
  onMount(() => {
    updateWidth();
    window.addEventListener("resize", updateWidth);
    onCleanup(() => window.removeEventListener("resize", updateWidth));
  });
  return [width, setRef, updateWidth];
}

export function useKeyDown(key: string, action: () => void) {
  onMount(() => {
    const listener = (e: KeyboardEvent) => {
      if (e.key === key) action();
    };
    window.addEventListener("keydown", listener);
    onCleanup(() => window.removeEventListener("keydown", listener));
  });
}