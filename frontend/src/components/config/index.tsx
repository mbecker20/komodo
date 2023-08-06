import { Switch } from "@ui/switch";
import { Button } from "@ui/button";
import { Card, CardHeader, CardTitle, CardContent } from "@ui/card";
import { Input } from "@ui/input";
import { Fragment, ReactNode, useState } from "react";

const fmt_field = (s: string) => s.split("_").join(" ");

export const Configuration = <T extends Partial<Record<keyof T, unknown>>>({
  config,
  loading,
  update,
  layout,
  overrides,
  set,
}: {
  config: T;
  loading: boolean;
  update: Partial<T>;
  layout?: { [key: string]: Array<keyof T> };
  overrides?: Partial<{
    [P in keyof T]: (
      value: T[P],
      set: (input: Partial<T>) => void
    ) => ReactNode;
  }>;
  set: (input: Partial<T>) => void;
}) => {
  const keys = Object.keys(layout ?? {});
  const [show, setShow] = useState(keys[0]);

  return (
    <div className="flex gap-4">
      {layout && (
        <div className="flex flex-col gap-4 w-[300px]">
          {Object.keys(layout).map((key) => (
            <Button
              key={key}
              onClick={() => setShow(key)}
              variant={key === show ? "secondary" : "outline"}
              // disabled={config === show}
              className="capitalize justify-start"
            >
              {fmt_field(key)}
            </Button>
          ))}
        </div>
      )}
      <Card className="w-full min-h-[50vh]">
        <CardHeader className="border-b">
          <CardTitle className="capitalize">{show}</CardTitle>
        </CardHeader>
        <CardContent className="flex flex-col gap-6 mt-6">
          {layout?.[show].map((field) => {
            if (typeof field !== "string") return null;
            const val = update[field] ?? config[field];
            const override = overrides?.[field];
            if (!!override)
              return <Fragment key={field}>{override(val, set)}</Fragment>;
            if (typeof val === "string") {
              return (
                <div
                  key={field}
                  className="flex justify-between items-center border-b pb-4"
                >
                  <div className="capitalize"> {fmt_field(field)} </div>
                  <Input
                    className="max-w-[400px]"
                    value={val}
                    onChange={(e) =>
                      set({ [field]: e.target.value } as Partial<T>)
                    }
                    disabled={loading}
                  />
                </div>
              );
            }
            if (typeof val === "number") {
              return (
                <div
                  key={field}
                  className="flex justify-between items-center border-b pb-4"
                >
                  <div className="capitalize"> {fmt_field(field)} </div>
                  <Input
                    className="max-w-[400px]"
                    type="number"
                    value={val}
                    onChange={(e) =>
                      set({ [field]: e.target.value } as Partial<T>)
                    }
                    disabled={loading}
                  />
                </div>
              );
            }
            if (typeof val === "boolean") {
              return (
                <div
                  key={field}
                  className="flex justify-between items-center border-b pb-4 min-h-[40px]"
                >
                  <div className="capitalize"> {fmt_field(field)} </div>
                  <Switch
                    checked={val}
                    onCheckedChange={(e) => set({ [field]: e } as Partial<T>)}
                    disabled={loading}
                  />
                </div>
              );
            }
            return (
              <>
                {field}
                <Configuration
                  key={field}
                  config={config[field] as T}
                  update={update[field] ?? {}}
                  loading={loading}
                  set={set}
                />
              </>
            );
          })}
          {!layout &&
            Object.entries(config).map(([field, value]) => {
              if (typeof field !== "string") return null;
              const val = update[field as keyof T] ?? value;
              const override = overrides?.[field as keyof T];
              if (!!override) return override(val as T[keyof T], set);
              if (typeof val === "string") {
                return (
                  <div
                    key={field}
                    className="flex justify-between items-center border-b pb-4"
                  >
                    <div className="capitalize text-md">{fmt_field(field)}</div>
                    <Input
                      className="max-w-[400px]"
                      value={val}
                      onChange={(e) =>
                        set({ [field]: e.target.value } as Partial<T>)
                      }
                      disabled={loading}
                    />
                  </div>
                );
              }
              if (typeof val === "number") {
                return (
                  <div
                    key={field}
                    className="flex justify-between items-center border-b pb-4"
                  >
                    <div className="capitalize text-md">{fmt_field(field)}</div>
                    <Input
                      className="max-w-[400px]"
                      type="number"
                      value={val}
                      onChange={(e) =>
                        set({ [field]: e.target.value } as Partial<T>)
                      }
                      disabled={loading}
                    />
                  </div>
                );
              }
              if (typeof val === "boolean") {
                return (
                  <div key={field} className="flex flex-col gap-2 min-h-[40px]">
                    <div className="capitalize"> {fmt_field(field)} </div>
                    <Switch
                      checked={val}
                      onCheckedChange={(e) => set({ [field]: e } as Partial<T>)}
                      disabled={loading}
                    />
                  </div>
                );
              }
              return (
                <>
                  {field}
                  <Configuration
                    key={field}
                    config={config[field as keyof T] as T}
                    update={update[field as keyof T] as Partial<T>}
                    loading={loading}
                    set={set}
                  />
                </>
              );
            })}
        </CardContent>
      </Card>
    </div>
  );
};
