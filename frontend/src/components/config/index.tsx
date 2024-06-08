import {
  ConfigInput,
  ConfigSwitch,
  ConfirmUpdate,
} from "@components/config/util";
import { Section } from "@components/layouts";
import { cn } from "@lib/utils";
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
import { AlertTriangle, History, Settings } from "lucide-react";
import { Fragment, ReactNode, SetStateAction, useState } from "react";

const keys = <T extends Record<string, unknown>>(obj: T) =>
  Object.keys(obj) as Array<keyof T>;

export const ConfigLayout = <
  T extends Types.Resource<unknown, unknown>["config"]
>({
  config,
  children,
  disabled,
  onConfirm,
  onReset,
  selector,
  titleOther,
}: {
  config: Partial<T>;
  children: ReactNode;
  disabled: boolean;
  onConfirm: () => void;
  onReset: () => void;
  selector?: ReactNode;
  titleOther?: ReactNode;
}) => {
  const titleProps = titleOther
    ? { titleOther }
    : { title: "Config", icon: <Settings className="w-4 h-4" /> };
  const changesMade = Object.keys(config).length ? true : false;
  return (
    <Section
      {...titleProps}
      actions={
        <div className="flex gap-2">
          {changesMade && (
            <div className="text-muted-foreground flex items-center gap-2">
              <AlertTriangle className="w-4 h-4" /> Unsaved changes
              <AlertTriangle className="w-4 h-4" />
            </div>
          )}
          {selector}
          {changesMade && (
            <Button
              variant="outline"
              onClick={onReset}
              disabled={disabled || !changesMade}
              className="flex items-center gap-2"
            >
              <History className="w-4 h-4" />
              Reset
            </Button>
          )}
          {changesMade && (
            <ConfirmUpdate
              content={JSON.stringify(config, null, 2)}
              onConfirm={onConfirm}
              disabled={disabled}
            />
          )}
        </div>
      }
    >
      {children}
    </Section>
  );
};

type PrimitiveConfigArgs = { placeholder?: string; label?: string };

type ConfigComponent<T> = {
  label: string;
  icon?: ReactNode;
  actions?: ReactNode;
  hidden?: boolean;
  labelHidden?: boolean;
  contentHidden?: boolean;
  components: {
    [K in keyof Partial<T>]:
      | boolean
      | PrimitiveConfigArgs
      | ((value: T[K], set: (value: Partial<T>) => void) => ReactNode);
  };
};

export const Config = <T,>({
  config,
  update,
  disabled,
  set,
  onSave,
  components,
  selector,
  titleOther,
}: {
  config: T;
  update: Partial<T>;
  disabled: boolean;
  set: React.Dispatch<SetStateAction<Partial<T>>>;
  onSave: () => Promise<void>;
  selector?: ReactNode;
  titleOther?: ReactNode;
  components: Record<
    string, // sidebar key
    ConfigComponent<T>[]
  >;
}) => {
  const [show, setShow] = useState(keys(components)[0]);

  return (
    <ConfigLayout
      titleOther={titleOther}
      config={update}
      disabled={disabled}
      onConfirm={async () => {
        await onSave();
        set({});
      }}
      onReset={() => set({})}
      selector={
        <div className="flex gap-4 items-center">
          {selector}

          {/* Add the config page selector when view is small / md (lg:hidden) */}
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
        </div>
      }
    >
      <div className="flex gap-4">
        {/** The sidebar when large */}
        <div className="hidden xl:flex flex-col gap-4 w-[300px]">
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
          {components[show].map(
            ({
              label,
              labelHidden,
              icon,
              actions,
              hidden,
              contentHidden,
              components,
            }) =>
              !hidden && (
                <Card className="w-full grid gap-2" key={label}>
                  {!labelHidden && (
                    <CardHeader
                      className={cn(
                        "flex-row items-center justify-between w-full py-0 h-[60px] space-y-0",
                        !contentHidden && "border-b"
                      )}
                    >
                      <CardTitle className="flex gap-4">
                        {icon}
                        {label}
                      </CardTitle>
                      {actions}
                    </CardHeader>
                  )}
                  {!contentHidden && (
                    <CardContent
                      className={cn(
                        "flex flex-col gap-2 pb-3",
                        labelHidden && "pt-3"
                      )}
                    >
                      <ConfigAgain
                        config={config}
                        update={update}
                        set={(u) => set((p) => ({ ...p, ...u }))}
                        components={components}
                        disabled={disabled}
                      />
                    </CardContent>
                  )}
                </Card>
              )
          )}
        </div>
      </div>
    </ConfigLayout>
  );
};

export const ConfigAgain = <
  T extends Types.Resource<unknown, unknown>["config"]
>({
  config,
  update,
  disabled,
  components,
  set,
}: {
  config: T;
  update: Partial<T>;
  disabled: boolean;
  components: Partial<{
    [K in keyof T extends string ? keyof T : never]:
      | boolean
      | PrimitiveConfigArgs
      | ((value: T[K], set: (value: Partial<T>) => void) => ReactNode);
  }>;
  set: (value: Partial<T>) => void;
}) => {
  return (
    <>
      {keys(components).map((key) => {
        const component = components[key];
        const value = update[key] ?? config[key];
        if (typeof component === "function") {
          return (
            <Fragment key={key.toString()}>{component(value, set)}</Fragment>
          );
        } else if (typeof component === "object" || component === true) {
          const args =
            typeof component === "object"
              ? (component as PrimitiveConfigArgs)
              : undefined;
          switch (typeof value) {
            case "string":
              return (
                <ConfigInput
                  key={args?.label ?? key.toString()}
                  label={key.toString()}
                  value={value}
                  onChange={(value) => set({ [key]: value } as Partial<T>)}
                  disabled={disabled}
                  placeholder={args?.placeholder}
                />
              );
            case "number":
              return (
                <ConfigInput
                  key={key.toString()}
                  label={args?.label ?? key.toString()}
                  value={Number(value)}
                  onChange={(value) =>
                    set({ [key]: Number(value) } as Partial<T>)
                  }
                  disabled={disabled}
                  placeholder={args?.placeholder}
                />
              );
            case "boolean":
              return (
                <ConfigSwitch
                  key={key.toString()}
                  label={args?.label ?? key.toString()}
                  value={value}
                  onChange={(value) => set({ [key]: value } as Partial<T>)}
                  disabled={disabled}
                />
              );
            default:
              return <div>{args?.label ?? key.toString()}</div>;
          }
        } else if (component === false) {
          return <Fragment key={key.toString()} />;
        }
      })}
    </>
  );
};
