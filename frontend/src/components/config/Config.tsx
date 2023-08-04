import { ReactNode, useState } from "react";
import { Card, CardContent, CardHeader, CardTitle } from "@ui/card";
import { StringConfig } from "./components/string";
import { ArrayConfig } from "./components/array";
import { BooleanConfig } from "./components/boolean";
import { NumberConfig } from "./components/number";
import { Button } from "@ui/button";
import { Tabs, TabsList } from "@ui/tabs";
import { TabsContent, TabsTrigger } from "@radix-ui/react-tabs";

export type ConfigSetter<T> = (
  update: (curr: Partial<T>) => Partial<T>
) => void;

export type OverideFn<T extends Record<string, unknown>, P extends keyof T> = (
  value: T[P],
  set: ConfigSetter<T[P]>
) => ReactNode;

export type Overides<T extends Record<string, unknown>> = {
  [P in keyof T]?:
    | OverideFn<T, P>
    | Overides<T[P] extends Record<string, unknown> ? T[P] : never>;
};

type DeepDisabled<T extends Record<string, unknown>> = {
  [P in keyof T]?:
    | boolean
    | DeepDisabled<T[P] extends Record<string, unknown> ? T[P] : never>;
};

type DeepDescription<T extends Record<string, unknown>> = {
  [P in keyof T]?: T[P] extends Record<string, unknown>
    ? DeepDescription<T[P]>
    : string;
};

type ArrayDefaults<T extends Record<string, unknown>> = {
  [P in keyof T]?: T[P] extends unknown[] ? T[P][number] : never;
};

function findDeepDisabled<T extends Record<string, unknown>>(
  field: string,
  disabled?: boolean | DeepDisabled<T>
) {
  if (disabled === undefined) {
    return false;
  }
  if (typeof disabled === "boolean") {
    return disabled;
  } else {
    return disabled[field];
  }
}

export function Config<T extends Record<string, unknown>>({
  config,
  update,
  set,
  overrides,
  descriptions,
  arrayDefaults,
  disabled,
}: {
  config: T;
  update: Partial<T>;
  set: ConfigSetter<T>;
  overrides?: Overides<T>;
  descriptions?: DeepDescription<T>;
  arrayDefaults?: ArrayDefaults<T>;
  disabled?: boolean | DeepDisabled<T>;
}) {
  const config_keys = Object.keys(config);
  const [show, setShow] = useState(config_keys[0]);

  return (
    <div className="flex gap-4">
      <div className="flex flex-col gap-4 w-[300px]">
        {Object.keys(config).map((config) => (
          <Button
            key={config}
            onClick={() => setShow(config)}
            variant={config === show ? "secondary" : "outline"}
            // disabled={config === show}
            className="capitalize justify-start"
          >
            {config.split("_").join(" ")}
          </Button>
        ))}
      </div>
      <div className="w-full h-fit">
        {Object.entries(config).map(([field, value]) => {
          if (show !== field) return null;
          const val = update[field] ?? value;
          const overide = overrides?.[field];
          if (overide && typeof overide === "function") {
            return overide(update[field] ?? (value as T[string]), (updt) =>
              set((curr) => ({
                ...curr,
                [field]: updt(
                  update[field] ?? (config[field] as Partial<T[string]>)
                ),
              }))
            ) as ReactNode;
          }
          if (typeof val === "string") {
            return (
              <StringConfig
                key={field}
                field={field}
                val={val}
                set={(u) =>
                  set((curr) => ({
                    ...curr,
                    [field]: u(val),
                  }))
                }
                description={descriptions?.[field] as string | undefined}
                disabled={findDeepDisabled(field, disabled) as boolean}
              />
            );
          }
          if (typeof val === "boolean") {
            return (
              <BooleanConfig
                key={field}
                field={field}
                val={val}
                set={(u) => set((curr) => ({ ...curr, [field]: u(val) }))}
                description={descriptions?.[field] as string | undefined}
                disabled={findDeepDisabled(field, disabled) as boolean}
              />
            );
          }
          if (typeof val === "number") {
            return (
              <NumberConfig
                key={field}
                field={field}
                val={val}
                set={(u) => set((curr) => ({ ...curr, [field]: u(val) }))}
                description={descriptions?.[field] as string | undefined}
                disabled={findDeepDisabled(field, disabled) as boolean}
              />
            );
          }
          if (Array.isArray(value)) {
            const val = (update[field] ? update[field] : value) as unknown[];
            return (
              <ArrayConfig
                key={field}
                field={field}
                val={val}
                set={set}
                defaultNew={arrayDefaults?.[field] ?? ""}
                disabled={findDeepDisabled(field, disabled) as boolean}
              />
            );
          }
          return (
            <Card key={field}>
              <CardHeader>
                <CardTitle>{field.replaceAll("_", " ")}</CardTitle>
              </CardHeader>
              <CardContent>
                <Config
                  config={config[field] as Record<string, unknown>}
                  update={update[field] ?? {}}
                  set={(update) => {
                    set((curr) => ({
                      ...curr,
                      [field]: update(
                        curr[field] ?? (config[field] as Partial<T[string]>)
                      ),
                    }));
                  }}
                  overrides={
                    overrides?.[field] as Overides<Record<string, unknown>>
                  }
                  disabled={findDeepDisabled(field, disabled)}
                />
              </CardContent>
            </Card>
          );
        })}
      </div>
    </div>
  );
}
