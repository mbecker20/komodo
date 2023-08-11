import { useRead } from "@hooks";
import { BuilderConfig, Resource } from "@monitor/client/dist/types";
import { ReactNode, useState } from "react";

const keys = <T extends Record<string, unknown>>(obj: T) =>
  Object.keys(obj) as Array<keyof T>;

export const ConfigAgain = <T extends Resource<unknown, unknown>["config"]>({
  config,
  update,
  components,
}: {
  config: T;
  update: Partial<T>;
  components: {
    [K in keyof T]: (value: T[K]) => ReactNode;
  };
}) => {
  return (
    <>
      {keys(components).map((key) => {
        const value = update[key] ?? config[key];
        return <>{components[key]?.(value)}</>;
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

export const Builder = ({ id }: { id: string }) => {
  const builder = useRead("GetBuilder", { id }).data;
  if (!builder?.config) return null;
  const [update, set] = useState<{
    type: BuilderConfig["type"];
    params: Partial<BuilderConfig["params"]>;
  }>({ type: builder.config.type, params: {} });

  return (
    <VariantConfig
      config={builder.config}
      update={update}
      components={{
        Server: {
          id: (id) => <div>{id}</div>,
        },
        Aws: {
          ami_id:
        }
      }}
    />
  );
};
