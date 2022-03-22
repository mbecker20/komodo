import { Accessor, Component, createContext, useContext } from "solid-js";
import { useInnerWidth } from "../util/hooks";

type WidthState = {
  width: Accessor<number>;
  isMobile: () => boolean;
};

const WidthContext = createContext<WidthState>();

export const WidthProvider: Component = (p) => {
  const width = useInnerWidth();
  const context = {
    width,
    isMobile: () => width() < 700,
  };
  return (
    <WidthContext.Provider value={context}>{p.children}</WidthContext.Provider>
  );
};

export function useAppWidth() {
  return useContext(WidthContext) as WidthState;
}
