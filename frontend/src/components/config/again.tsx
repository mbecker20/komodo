import { ConfigInput, ConfigSwitch } from "@components/config/util";
import { Resource } from "@monitor/client/dist/types";
import { Fragment, ReactNode } from "react";

const keys = <T extends Record<string, unknown>>(obj: T) =>
  Object.keys(obj) as Array<keyof T>;

export const ConfigAgain = <T extends Resource<unknown, unknown>["config"]>({
  config,
  update,
  components,
  set,
}: {
  config: T;
  update: Partial<T>;
  components: Partial<{
    [K in keyof T extends string ? keyof T : never]:
      | true
      | ((value: T[K], set: (value: Partial<T>) => void) => ReactNode);
  }>;
  set: (value: Partial<T>) => void;
}) => {
  return (
    <>
      {keys(components).map((key) => {
        const component = components[key];
        const value = update[key] ?? config[key];
        if (component === true) {
          switch (typeof value) {
            case "string":
              return (
                <ConfigInput
                  key={key.toString()}
                  label={key.toString()}
                  value={value}
                  onChange={(value) => set({ [key]: value } as Partial<T>)}
                />
              );
            case "number":
              return (
                <ConfigInput
                  key={key.toString()}
                  label={key.toString()}
                  value={Number(value)}
                  onChange={(value) =>
                    set({ [key]: Number(value) } as Partial<T>)
                  }
                />
              );
            case "boolean":
              return (
                <ConfigSwitch
                  key={key.toString()}
                  label={key.toString()}
                  value={value}
                  onChange={(value) => set({ [key]: value } as Partial<T>)}
                />
              );
            default:
              return <div>{key.toString()}</div>;
          }
        }
        return (
          <Fragment key={key.toString()}>{component?.(value, set)}</Fragment>
        );
      })}
    </>
  );
};
