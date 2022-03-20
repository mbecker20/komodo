import { atom, useAtom } from "jotai";
import { useCallback } from "react";

export function createUseConfig<T>(init: T) {
  const configAtom = atom<T>(init);

  return () => {
    const [config, setConfig] = useAtom(configAtom);
    const set = useCallback((field: keyof T, val: T[keyof T]) => {
      setConfig((config) => ({ ...config, [field]: val }));
    }, []);
    const setMany = useCallback(
      (...updates: Array<[field: keyof T, val: T[keyof T]]>) => {
        setConfig((config) =>
          Object.assign(
            {},
            config,
            ...updates.map(([field, val]) => ({ [field]: val }))
          )
        );
      },
      []
    );
    return {
      config: config as T,
      set,
      setMany
    };
  }
}

export function createUseSequence() {
  const currentAtom = atom(0);
  return () => {
    const [current, set] = useAtom(currentAtom);
    const next = useCallback(() => {
      set((current) => current + 1);
    }, []);
    const prev = useCallback(() => {
      set((current) => Math.max(current - 1, 0));
    }, []);
    return {
      current,
      next,
      prev,
    };
  };
}
