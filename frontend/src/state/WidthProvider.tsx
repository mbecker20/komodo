import { Accessor, Component, createContext, useContext } from "solid-js";
import { useInnerWidth } from "../util/hooks";

type Width = Accessor<number>;

const WidthContext = createContext<Width>();

export const WidthProvider: Component = (p) => {
  const width = useInnerWidth();
  return (
    <WidthContext.Provider value={width}>{p.children}</WidthContext.Provider>
  );
};

export function useAppWidth() {
  return useContext(WidthContext) as Width;
}