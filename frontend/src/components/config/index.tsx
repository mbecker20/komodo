import {
  ConfigInput,
  ConfigSwitch,
  ConfirmUpdate,
} from "@components/config/util";
import { Section } from "@components/layouts";
import { Types } from "@monitor/client";
import { Button } from "@ui/button";
import { Card, CardHeader, CardTitle, CardContent } from "@ui/card";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@ui/select";
import { History, Settings } from "lucide-react";
import { Fragment, ReactNode, SetStateAction, useState } from "react";

const keys = <T extends Record<string, unknown>>(obj: T) =>
  Object.keys(obj) as Array<keyof T>;

export const ConfigAgain = <
  T extends Types.Resource<unknown, unknown>["config"]
>({
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

const ConfigLayout = <T extends Types.Resource<unknown, unknown>["config"]>({
  content,
  children,
  onConfirm,
  onReset,
  selector,
}: {
  content: Partial<T>;
  children: ReactNode;
  onConfirm: () => void;
  onReset: () => void;
  selector?: ReactNode;
}) => (
  <Section
    title="Config"
    icon={<Settings className="w-4 h-4" />}
    actions={
      <div className="flex gap-2">
        {selector}
        <Button
          variant="outline"
          onClick={onReset}
          disabled={content ? !Object.keys(content).length : true}
        >
          <History className="w-4 h-4" />
        </Button>
        <ConfirmUpdate
          content={JSON.stringify(content, null, 2)}
          onConfirm={onConfirm}
        />
      </div>
    }
  >
    {children}
  </Section>
);

export const ConfigInner = <T,>({
  config,
  update,
  set,
  onSave,
  components,
}: {
  config: T;
  update: Partial<T>;
  set: React.Dispatch<SetStateAction<Partial<T>>>;
  onSave: () => void;
  components: Record<
    string,
    Record<
      string,
      {
        [K in keyof Partial<T>]:
          | true
          | ((value: T[K], set: (value: Partial<T>) => void) => ReactNode);
      }
    >
  >;
}) => {
  const [show, setShow] = useState(keys(components)[0]);

  return (
    <ConfigLayout
      content={update}
      onConfirm={onSave}
      onReset={() => set({})}
      selector={
        <Select value={show} onValueChange={setShow}>
          <SelectTrigger className="w-32 capitalize lg:hidden">
            <SelectValue />
          </SelectTrigger>
          <SelectContent className="w-32">
            {keys(components).map((key) => (
              <SelectItem value={key} key={key} className="capitalize">
                {key}
              </SelectItem>
            ))}
          </SelectContent>
        </Select>
      }
    >
      <div className="flex gap-4">
        <div className="hidden lg:flex flex-col gap-4 w-[300px]">
          {keys(components).map((tab) => (
            <Button
              key={tab}
              variant={show === tab ? "secondary" : "outline"}
              onClick={() => setShow(tab)}
              className="capitalize"
            >
              {tab}
            </Button>
          ))}
        </div>
        <div className="flex flex-col gap-6 min-h-[500px] w-full">
          {Object.entries(components[show]).map(([k, v]) => (
            <Card className="w-full" key={k}>
              <CardHeader className="border-b">
                <CardTitle className="capitalize">{k} Settings</CardTitle>
              </CardHeader>
              <CardContent className="flex flex-col gap-4 mt-4">
                <ConfigAgain
                  config={config}
                  update={update}
                  set={(u) => set((p) => ({ ...p, ...u }))}
                  components={v}
                />
              </CardContent>
            </Card>
          ))}
        </div>
      </div>
    </ConfigLayout>
  );
};
