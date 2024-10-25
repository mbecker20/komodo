import {
  ConfigInput,
  ConfigSwitch,
  ConfirmUpdate,
} from "@components/config/util";
import { Section } from "@components/layouts";
import { MonacoLanguage } from "@components/monaco";
import { Types } from "komodo_client";
import { cn } from "@lib/utils";
import { Button } from "@ui/button";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@ui/select";
import { AlertTriangle, History, Settings } from "lucide-react";
import { Fragment, ReactNode, SetStateAction } from "react";

const keys = <T extends Record<string, unknown>>(obj: T) =>
  Object.keys(obj) as Array<keyof T>;

export const ConfigLayout = <
  T extends Types.Resource<unknown, unknown>["config"],
>({
  original,
  config,
  children,
  disabled,
  onConfirm,
  onReset,
  selector,
  titleOther,
  file_contents_language,
}: {
  original: T;
  config: Partial<T>;
  children: ReactNode;
  disabled: boolean;
  onConfirm: () => void;
  onReset: () => void;
  selector?: ReactNode;
  titleOther?: ReactNode;
  file_contents_language?: MonacoLanguage;
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
              previous={original}
              content={config}
              onConfirm={async () => onConfirm()}
              disabled={disabled}
              file_contents_language={file_contents_language}
            />
          )}
        </div>
      }
    >
      {children}
    </Section>
  );
};

export type PrimitiveConfigArgs = {
  placeholder?: string;
  label?: string;
  boldLabel?: boolean;
  description?: string;
};

export type ConfigComponent<T> = {
  label: string;
  boldLabel?: boolean; // defaults to true
  icon?: ReactNode;
  actions?: ReactNode;
  labelExtra?: ReactNode;
  description?: ReactNode;
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
  disableSidebar,
  set,
  onSave,
  components,
  selector,
  titleOther,
  file_contents_language,
}: {
  config: T;
  update: Partial<T>;
  disabled: boolean;
  disableSidebar?: boolean;
  set: React.Dispatch<SetStateAction<Partial<T>>>;
  onSave: () => Promise<void>;
  selector?: ReactNode;
  titleOther?: ReactNode;
  components: Record<
    string, // sidebar key
    ConfigComponent<T>[] | false | undefined
  >;
  file_contents_language?: MonacoLanguage;
}) => {
  const sections = keys(components).filter((section) => !!components[section]);

  return (
    <ConfigLayout
      original={config}
      titleOther={titleOther}
      config={update}
      disabled={disabled}
      onConfirm={async () => {
        await onSave();
        set({});
      }}
      onReset={() => set({})}
      selector={selector}
      file_contents_language={file_contents_language}
    >
      <div className="flex gap-6">
        {!disableSidebar && (
          <div className="hidden xl:block relative pr-6 border-r">
            <div className="sticky top-24 hidden xl:flex flex-col gap-8 w-[140px] h-fit pb-24">
              {sections.map((section) => (
                <div key={section}>
                  {section && (
                    <p className="text-muted-foreground uppercase text-right mb-2">
                      {section}
                    </p>
                  )}
                  <div className="flex flex-col gap-2">
                    {components[section] &&
                      components[section]
                        .filter((item) => !item.hidden)
                        .map((item) => (
                          // uses a tags becasue react-router-dom Links don't reliably hash scroll
                          <a
                            href={"#" + section + item.label}
                            key={section + item.label}
                          >
                            <Button
                              variant="secondary"
                              className="justify-end w-full"
                              size="sm"
                            >
                              {item.label}
                            </Button>
                          </a>
                        ))}
                  </div>
                </div>
              ))}
            </div>
          </div>
        )}
        <div className="w-full flex flex-col gap-12">
          {sections.map(
            (section) =>
              components[section] && (
                <div
                  key={section}
                  className="relative pb-12 border-b last:pb-0 last:border-b-0 "
                >
                  <div className="xl:hidden sticky top-16 h-16 flex items-center justify-between bg-background z-10">
                    {section && <p className="uppercase text-2xl">{section}</p>}
                    <Select
                      onValueChange={(value) => (window.location.hash = value)}
                    >
                      <SelectTrigger className="w-32 capitalize xl:hidden">
                        <SelectValue placeholder="Go To" />
                      </SelectTrigger>
                      <SelectContent className="w-32">
                        {components[section]
                          .filter((item) => !item.hidden)
                          .map(({ label }) => (
                            <SelectItem
                              key={section + label}
                              value={section + label}
                              className="capitalize"
                            >
                              {label}
                            </SelectItem>
                          ))}
                      </SelectContent>
                    </Select>
                  </div>
                  {section && (
                    <p className="hidden xl:block bg-background text-2xl uppercase mb-6 h-fit">
                      {section}
                    </p>
                  )}
                  <div className="flex flex-col gap-6 w-full">
                    {components[section].map(
                      ({
                        label,
                        boldLabel = true,
                        labelHidden,
                        icon,
                        labelExtra,
                        actions,
                        description,
                        hidden,
                        contentHidden,
                        components,
                      }) => (
                        <div
                          key={section + label}
                          id={section + label}
                          className={cn(
                            "p-6 border rounded-md flex flex-col gap-6 scroll-mt-40 xl:scroll-mt-24",
                            hidden && "hidden"
                          )}
                        >
                          {!labelHidden && (
                            <div className="flex justify-between">
                              <div>
                                <div className="flex items-center gap-4">
                                  {icon}
                                  <div
                                    className={cn(
                                      "text-lg",
                                      boldLabel && "font-bold"
                                    )}
                                  >
                                    {label}
                                  </div>
                                  {labelExtra}
                                </div>
                                {description && (
                                  <div className="text-sm text-muted-foreground">
                                    {description}
                                  </div>
                                )}
                              </div>
                              {actions}
                            </div>
                          )}
                          {!contentHidden && (
                            <ConfigAgain
                              config={config}
                              update={update}
                              set={(u) => set((p) => ({ ...p, ...u }))}
                              components={components}
                              disabled={disabled}
                            />
                          )}
                        </div>
                      )
                    )}
                  </div>
                </div>
              )
          )}
        </div>
      </div>
    </ConfigLayout>
  );
};

export const ConfigAgain = <
  T extends Types.Resource<unknown, unknown>["config"],
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
                  key={key.toString()}
                  label={args?.label ?? key.toString()}
                  value={value}
                  onChange={(value) => set({ [key]: value } as Partial<T>)}
                  disabled={disabled}
                  placeholder={args?.placeholder}
                  description={args?.description}
                  boldLabel={args?.boldLabel}
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
                  description={args?.description}
                  boldLabel={args?.boldLabel}
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
                  description={args?.description}
                  boldLabel={args?.boldLabel}
                />
              );
            default:
              return (
                <div key={key.toString()}>{args?.label ?? key.toString()}</div>
              );
          }
        } else {
          return <Fragment key={key.toString()} />;
        }
      })}
    </>
  );
};
