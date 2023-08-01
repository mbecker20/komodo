import { ReactNode } from "react";
import { Button } from "@ui/button";
import { Input } from "@ui/input";
import { Switch } from "@ui/switch";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@ui/card";
import { Selector, SearchableSelector, SelectorItem } from "./Selector";

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
  return (
    <div className="grid lg:grid-cols-2 gap-4">
      {Object.entries(config).map(([field, value]) => {
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
  );
}

export function StringConfig({
  field,
  val,
  set,
  description,
  disabled,
}: {
  field: string;
  val: string;
  set: ConfigSetter<string>;
  description?: string;
  disabled?: boolean;
}) {
  const readable_field = field.replaceAll("_", " ");
  return (
    <Card>
      <CardHeader>
        <CardTitle>{readable_field}</CardTitle>
        {description && <CardDescription>{description}</CardDescription>}
      </CardHeader>
      <CardContent>
        <Input
          id={field}
          className="w-full"
          value={val}
          onChange={(e) => set(() => e.target.value)}
          disabled={disabled}
        />
      </CardContent>
    </Card>
  );
}

export function StringSelectorConfig({
  field,
  val,
  set,
  items,
  label,
  description,
  disabled,
}: {
  field: string;
  val: string;
  set: ConfigSetter<string>;
  items: SelectorItem[];
  label?: (val: string) => string;
  description?: string;
  disabled?: boolean;
}) {
  return (
    <Card>
      <CardHeader className="flex flex-row justify-between items-center">
        <div className="flex gap-2">
          <CardTitle>{field.replaceAll("_", " ")}</CardTitle>
          {description && <CardDescription>{description}</CardDescription>}
        </div>
        <Selector
          width="180px"
          value={{
            value: val,
            label: label ? label(val) : val,
          }}
          items={items}
          onSelect={({ value }) => set(() => value)}
          disabled={disabled}
        />
      </CardHeader>
    </Card>
  );
}

export function SearchableStringSelectorConfig({
  field,
  val,
  set,
  items,
  label,
  description,
  disabled,
}: {
  field: string;
  val: string;
  set: ConfigSetter<string>;
  items: SelectorItem[];
  label?: (val: string) => string;
  description?: string;
  disabled?: boolean;
}) {
  return (
    <Card>
      <CardHeader className="flex flex-row justify-between items-center">
        <div className="flex gap-2">
          <CardTitle>{field.replaceAll("_", " ")}</CardTitle>
          {description && <CardDescription>{description}</CardDescription>}
        </div>
        <SearchableSelector
          width="180px"
          value={{
            value: val,
            label: label ? label(val) : val,
          }}
          items={items}
          onSelect={(value) => set(() => value)}
          disabled={disabled}
        />
      </CardHeader>
    </Card>
  );
}

export function BooleanConfig({
  field,
  val,
  set,
  description,
  disabled,
}: {
  field: string;
  val: boolean;
  set: ConfigSetter<boolean>;
  description?: string;
  disabled?: boolean;
}) {
  const readable_field = field.replaceAll("_", " ");
  const onChange = (checked: boolean) => set(() => checked);
  return (
    <Card
      className="flex flex-row justify-between items-center cursor-pointer"
      onClick={() => onChange(!val)}
    >
      <CardHeader>
        <CardTitle>{readable_field}</CardTitle>
        {description && <CardDescription>{description}</CardDescription>}
      </CardHeader>
      <Switch
        id={field}
        className="m-6"
        checked={val}
        onCheckedChange={onChange}
        disabled={disabled}
      />
    </Card>
  );
}

export function NumberConfig({
  field,
  val,
  set,
  disabled,
  description,
}: {
  field: string;
  val: number;
  set: ConfigSetter<number>;
  description?: string;
  disabled?: boolean;
}) {
  const readable_field = field.replaceAll("_", " ");
  return (
    <Card>
      <CardHeader>
        <CardTitle>{readable_field}</CardTitle>
        {description && <CardDescription>{description}</CardDescription>}
      </CardHeader>
      <CardContent>
        <Input
          className="w-[250px]"
          type="number"
          defaultValue={val}
          onChange={(e) => set(() => e.target.valueAsNumber)}
          disabled={disabled}
        />
      </CardContent>
    </Card>
  );
}

export function ArrayConfig<T, D>({
  field,
  val,
  set,
  defaultNew,
  description,
  disabled,
}: {
  field: string;
  val: unknown[];
  set: ConfigSetter<T>;
  defaultNew: D;
  description?: string;
  disabled?: boolean;
}) {
  return (
    <Card>
      <CardHeader className="flex flex-row justify-between items-center">
        <div className="flex flex-col items-center">
          <CardTitle>{field.replaceAll("_", " ")}</CardTitle>
          {description && <CardDescription>{description}</CardDescription>}
        </div>
        <Button
          onClick={() =>
            set((curr) => ({ ...curr, [field]: [...val, defaultNew] }))
          }
          disabled={disabled}
        >
          +
        </Button>
      </CardHeader>
      {val.length > 0 && (
        <CardContent className="flex flex-col gap-4">
          {val.map((item, i) => {
            return (
              <div
                key={i.toString()}
                className="flex flex-row gap-4 justify-between items-center"
              >
                {typeof item === "string" ? (
                  <Input
                    value={val[i] as string}
                    onChange={(e) =>
                      set((curr) => {
                        return {
                          ...curr,
                          [field]: [
                            ...val.slice(0, i),
                            e.target.value,
                            ...val.slice(i + 1),
                          ],
                        };
                      })
                    }
                    disabled={disabled}
                  />
                ) : typeof item === "object" ? (
                  <Config
                    config={item as Record<string, unknown>}
                    update={item as Record<string, unknown>}
                    set={(upd) =>
                      set((curr) => ({
                        ...curr,
                        [field]: [
                          ...val.slice(0, i),
                          upd(item as Record<string, unknown>),
                          ...val.slice(i + 1),
                        ],
                      }))
                    }
                  />
                ) : null}
                <Button
                  onClick={() => {
                    set((curr) => ({
                      ...curr,
                      [field]: val.filter((_, index) => i !== index),
                    }));
                  }}
                  disabled={disabled}
                >
                  -
                </Button>
              </div>
            );
          })}
        </CardContent>
      )}
    </Card>
  );
}

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
