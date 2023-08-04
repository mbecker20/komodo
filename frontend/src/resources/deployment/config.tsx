import { useRead, useWrite } from "@hooks";
import { Section } from "@layouts/page";
import { Types } from "@monitor/client";
import { Button } from "@ui/button";
import { Card, CardHeader, CardTitle, CardContent } from "@ui/card";
import { Input } from "@ui/input";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@ui/select";
import { Switch } from "@ui/switch";
import { Settings, Save, History, PlusCircle } from "lucide-react";
import { ReactNode, useState } from "react";
import { useParams } from "react-router-dom";

const ServersSelector = ({
  selected,
  onSelect,
}: {
  selected: string | undefined;
  onSelect: (serverId: string) => void;
}) => {
  const servers = useRead("ListServers", {}).data;
  return (
    <Select value={selected} onValueChange={onSelect}>
      <SelectTrigger className="w-[400px]">
        <SelectValue placeholder="Select A Server" />
      </SelectTrigger>
      <SelectContent>
        {servers?.map((s) => (
          <SelectItem key={s.id} value={s.id}>
            {s.name}
          </SelectItem>
        ))}
      </SelectContent>
    </Select>
  );
};

export const DeploymentConfig = () => {
  const id = useParams().deploymentId;
  const deployment = useRead("GetDeployment", { id }).data;
  const [update, set] = useState<Partial<Types.DeploymentConfig>>({});
  const { mutate, isLoading } = useWrite("UpdateDeployment");

  console.log(deployment?.config);

  if (!id || !deployment?.config) return null;

  return (
    <Section
      title="Config"
      icon={<Settings className="w-4 h-4" />}
      actions={
        <div className="flex gap-4">
          <Button variant="outline" intent="warning" onClick={() => set({})}>
            <History className="w-4 h-4" />
          </Button>
          <Button
            variant="outline"
            intent="success"
            onClick={() => mutate({ config: update, id })}
          >
            <Save className="w-4 h-4" />
          </Button>
        </div>
      }
    >
      <Configuration
        config={deployment.config}
        loading={isLoading}
        update={update}
        set={(input) => set((update) => ({ ...update, ...input }))}
        layout={{
          general: ["server_id", "image", "restart"],
          networking: ["network", "ports"],
          environment: ["environment", "skip_secret_interp"],
        }}
        overrides={{
          server_id: (value, set) => (
            <div className="flex items-center justify-between border-b pb-4">
              Server
              <ServersSelector
                selected={value}
                onSelect={(server_id) => set({ server_id })}
              />
            </div>
          ),
          image: () => <div>Image </div>,
          environment: (vars, set) => (
            <div className="flex flex-col gap-4 border-b pb-4">
              {vars?.map((variable, i) => (
                <div className="flex justify-between gap-4">
                  <Input
                    value={variable.variable}
                    placeholder="Variable Name"
                    onChange={(e) => {
                      vars[i].variable = e.target.value;
                      set({ environment: [...vars] });
                    }}
                  />
                  =
                  <Input
                    value={variable.value}
                    placeholder="Variable Value"
                    onChange={(e) => {
                      vars[i].value = e.target.value;
                      set({ environment: [...vars] });
                    }}
                  />
                </div>
              ))}
              <Button
                variant="outline"
                intent="success"
                className="flex items-center gap-2"
                onClick={() =>
                  set({
                    environment: [...(vars ?? []), { variable: "", value: "" }],
                  })
                }
              >
                <PlusCircle className="w-4 h-4" />
                Add
              </Button>
            </div>
          ),
        }}
      />
    </Section>
  );
};

const fmt_field = (s: string) => s.split("_").join(" ");
const Configuration = <T extends Partial<Record<keyof T, unknown>>>({
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
            if (!!override) return override(val, set);
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
                  className="flex justify-between items-center border-b pb-4"
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
              <Configuration
                config={config[field] as T}
                update={update[field] as Partial<T>}
                loading={loading}
                set={set}
              />
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
                    <div className="capitalize text-md">
                      {" "}
                      {fmt_field(field)}{" "}
                    </div>
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
                    <div className="capitalize text-md">
                      {" "}
                      {fmt_field(field)}{" "}
                    </div>
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
                  <div key={field} className="flex flex-col gap-2">
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
                <Configuration
                  config={config[field as keyof T] as T}
                  update={update[field as keyof T] as Partial<T>}
                  loading={loading}
                  set={set}
                />
              );
            })}
        </CardContent>
      </Card>
    </div>
  );
};
