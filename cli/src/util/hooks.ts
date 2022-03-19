import { useInput, Key } from "ink";
import { useCallback, useEffect, useState } from "react";

export function useBlinker(interval = 750) {
  const [on, setOn] = useState(false);
  useEffect(() => {
    const int = setInterval(() => {
      setOn((on) => !on);
    }, interval);
    return () => clearInterval(int);
  }, []);
  return on;
}

export function useKey(key: keyof Key, callback: () => void) {
  useInput((_, k) => {
    if (k[key]) callback();
  });
}

export function useEnter(onEnter: () => void) {
  useKey("return", onEnter);
}

export function useEsc(onEsc: () => void) {
  useKey("escape", onEsc);
}