import { Accessor, Component, createContext, useContext } from "solid-js";
import { useInnerHeight, useInnerWidth } from "../util/hooks";

type DimensionState = {
  width: Accessor<number>;
  height: Accessor<number>;
  isMobile: () => boolean;
};

const DimensionContext = createContext<DimensionState>();

export const DimensionProvider: Component = (p) => {
  const width = useInnerWidth();
  const height = useInnerHeight();
  const context = {
    width,
    height,
    isMobile: () => width() <= 700,
  };
  return (
    <DimensionContext.Provider value={context}>
      {p.children}
    </DimensionContext.Provider>
  );
};

export function useAppDimensions() {
  return useContext(DimensionContext) as DimensionState;
}
