import { Resource } from "@monitor/client/dist/types";
import { Fragment, ReactNode } from "react";

const keys = <T extends Record<string, unknown>>(obj: T) =>
  Object.keys(obj) as Array<keyof T>;

export const ConfigAgain = <T extends Resource<unknown, unknown>["config"]>({
  config,
  update,
  components,
}: {
  config: T;
  update: Partial<T>;
  components: Partial<{
    [K in keyof T]: (value: T[K]) => ReactNode;
  }>;
}) => {
  return (
    <>
      {keys(components).map((key) => {
        const value = update[key] ?? config[key];
        return (
          <Fragment key={key.toString()}>{components[key]?.(value)}</Fragment>
        );
      })}
    </>
  );
};

export const VariantConfig = <P, T extends { type: string; params: P }>({
  config,
  update,
  components,
}: {
  config: T;
  update: { type: T["type"]; params: Partial<P> };
  components: {
    [key in T["type"]]: { [K in keyof P]: (value: P[K]) => ReactNode };
  };
}) => {
  return <>{config}</>;
};
