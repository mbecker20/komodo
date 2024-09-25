/* eslint-disable @typescript-eslint/no-explicit-any */
import { useRead } from "@lib/hooks";
import { Types } from "@komodo/client";
import {
  Select,
  SelectTrigger,
  SelectValue,
  SelectContent,
  SelectItem,
} from "@ui/select";
import { Button } from "@ui/button";
import { Input } from "@ui/input";
import { Switch } from "@ui/switch";
import {
  CheckCircle,
  Info,
  MinusCircle,
  PlusCircle,
  Save,
  SearchX,
} from "lucide-react";
import { ReactNode, useState } from "react";
import { cn, env_to_text, filterBySplit } from "@lib/utils";
import {
  Dialog,
  DialogContent,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from "@ui/dialog";
import { snake_case_to_upper_space_case } from "@lib/formatting";
import {
  ConfirmButton,
  ShowHideButton,
  TextUpdateMenu,
} from "@components/util";
import { Popover, PopoverContent, PopoverTrigger } from "@ui/popover";
import {
  Command,
  CommandEmpty,
  CommandGroup,
  CommandInput,
  CommandItem,
  CommandList,
} from "@ui/command";
import { HoverCard, HoverCardContent, HoverCardTrigger } from "@ui/hover-card";
import { Card, CardContent, CardHeader } from "@ui/card";
import { text_color_class_by_intention } from "@lib/color";
import { MonacoDiffEditor } from "@components/monaco";

export const ConfigItem = ({
  label,
  boldLabel,
  description,
  children,
  className,
}: {
  label?: string;
  boldLabel?: boolean;
  description?: ReactNode;
  children: ReactNode;
  className?: string;
}) => (
  <>
    <div
      className={cn(
        "flex justify-between items-center min-h-[60px]",
        className
      )}
    >
      <div className="flex items-center gap-4">
        {label && (
          <div className={cn("text-nowrap", boldLabel && "font-semibold")}>
            {snake_case_to_upper_space_case(label)}
          </div>
        )}
        {description && (
          <HoverCard openDelay={200}>
            <HoverCardTrigger asChild>
              <Card className="px-3 py-2 hover:bg-accent/50 transition-colors cursor-pointer">
                <Info className="w-4 h-4" />
              </Card>
            </HoverCardTrigger>
            <HoverCardContent align="start" side="right">
              {description}
            </HoverCardContent>
          </HoverCard>
        )}
      </div>
      {children}
    </div>
    <div className="w-full h-0 border-b last:hidden" />
  </>
);

export const ConfigInput = ({
  label,
  boldLabel,
  value,
  description,
  disabled,
  placeholder,
  onChange,
  onBlur,
  className,
  inputLeft,
  inputRight,
}: {
  label: string;
  boldLabel?: boolean;
  value: string | number | undefined;
  description?: string;
  disabled?: boolean;
  placeholder?: string;
  onChange?: (value: string) => void;
  onBlur?: (value: string) => void;
  className?: string;
  inputLeft?: ReactNode;
  inputRight?: ReactNode;
}) => (
  <ConfigItem label={label} boldLabel={boldLabel} description={description}>
    {inputLeft || inputRight ? (
      <div className="flex gap-2 items-center">
        {inputLeft}
        <Input
          className={cn("max-w-[75%] lg:max-w-[400px]", className)}
          type={typeof value === "number" ? "number" : undefined}
          value={value}
          onChange={(e) => onChange && onChange(e.target.value)}
          onBlur={(e) => onBlur && onBlur(e.target.value)}
          placeholder={placeholder}
          disabled={disabled}
        />
        {inputRight}
      </div>
    ) : (
      <Input
        className={cn("max-w-[75%] lg:max-w-[400px]", className)}
        type={typeof value === "number" ? "number" : undefined}
        value={value}
        onChange={(e) => onChange && onChange(e.target.value)}
        onBlur={(e) => onBlur && onBlur(e.target.value)}
        placeholder={placeholder}
        disabled={disabled}
      />
    )}
  </ConfigItem>
);

export const ConfigSwitch = ({
  label,
  boldLabel,
  value,
  description,
  disabled,
  onChange,
}: {
  label: string;
  boldLabel?: boolean;
  value: boolean | undefined;
  description?: string;
  disabled: boolean;
  onChange: (value: boolean) => void;
}) => (
  <ConfigItem label={label} description={description} boldLabel={boldLabel}>
    <Switch checked={value} onCheckedChange={onChange} disabled={disabled} />
  </ConfigItem>
);

export const DoubleInput = <
  T extends object,
  K extends keyof T,
  L extends T[K] extends string | number | undefined ? K : never,
  R extends T[K] extends string | number | undefined ? K : never
>({
  disabled,
  values,
  leftval,
  leftpl,
  rightval,
  rightpl,
  // addName,
  onLeftChange,
  onRightChange,
  // onAdd,
  onRemove,
  containerClassName,
  inputClassName,
}: {
  disabled: boolean;
  values: T[] | undefined;
  leftval: L;
  leftpl: string;
  rightval: R;
  rightpl: string;
  // addName: string;
  onLeftChange: (value: T[L], i: number) => void;
  onRightChange: (value: T[R], i: number) => void;
  // onAdd: () => void;
  onRemove: (i: number) => void;
  containerClassName?: string;
  inputClassName?: string;
}) => {
  return (
    <div className={cn("flex flex-col gap-4", containerClassName)}>
      {values?.map((value, i) => (
        <div
          className="flex items-center justify-between gap-4 flex-wrap"
          key={i}
        >
          <Input
            className={inputClassName}
            value={value[leftval] as any}
            placeholder={leftpl}
            onChange={(e) => onLeftChange(e.target.value as T[L], i)}
            disabled={disabled}
          />
          :
          <Input
            className={inputClassName}
            value={value[rightval] as any}
            placeholder={rightpl}
            onChange={(e) => onRightChange(e.target.value as T[R], i)}
            disabled={disabled}
          />
          {!disabled && (
            <Button variant="secondary" onClick={() => onRemove(i)}>
              <MinusCircle className="w-4 h-4" />
            </Button>
          )}
        </div>
      ))}
      {/* {!disabled && (
        <Button
          variant="secondary"
          className="flex items-center gap-2 w-[200px] place-self-end"
          onClick={onAdd}
        >
          <PlusCircle className="w-4 h-4" />
          Add {addName}
        </Button>
      )} */}
    </div>
  );
};

export const ProviderSelector = ({
  disabled,
  account_type,
  selected,
  onSelect,
  showCustom = true,
}: {
  disabled: boolean;
  account_type: "git" | "docker";
  selected: string | undefined;
  onSelect: (provider: string) => void;
  showCustom?: boolean;
}) => {
  const [db_request, config_request]:
    | ["ListGitProviderAccounts", "ListGitProvidersFromConfig"]
    | ["ListDockerRegistryAccounts", "ListDockerRegistriesFromConfig"] =
    account_type === "git"
      ? ["ListGitProviderAccounts", "ListGitProvidersFromConfig"]
      : ["ListDockerRegistryAccounts", "ListDockerRegistriesFromConfig"];
  const db_providers = useRead(db_request, {}).data;
  const config_providers = useRead(config_request, {}).data;
  const [customMode, setCustomMode] = useState(false);

  if (customMode) {
    return (
      <Input
        placeholder="Input custom provider domain"
        value={selected}
        onChange={(e) => onSelect(e.target.value)}
        className="max-w-[75%] lg:max-w-[400px]"
        onBlur={() => setCustomMode(false)}
        onKeyDown={(e) => {
          if (e.key === "Enter") {
            setCustomMode(false);
          }
        }}
        autoFocus
      />
    );
  }

  const domains = new Set<string>();
  for (const provider of db_providers ?? []) {
    domains.add(provider.domain);
  }
  for (const provider of config_providers ?? []) {
    domains.add(provider.domain);
  }
  const providers = [...domains];
  providers.sort();

  return (
    <Select
      value={selected}
      onValueChange={(value) => {
        if (value === "Custom") {
          onSelect("");
          setCustomMode(true);
        } else {
          onSelect(value);
        }
      }}
      disabled={disabled}
    >
      <SelectTrigger
        className="w-full lg:w-[200px] max-w-[50%]"
        disabled={disabled}
      >
        <SelectValue placeholder="Select Provider" />
      </SelectTrigger>
      <SelectContent>
        {providers
          ?.filter((provider) => provider)
          .map((provider) => (
            <SelectItem key={provider} value={provider}>
              {provider}
            </SelectItem>
          ))}
        {providers !== undefined &&
          selected &&
          !providers.includes(selected) && (
            <SelectItem value={selected}>{selected}</SelectItem>
          )}
        {showCustom && <SelectItem value={"Custom"}>Custom</SelectItem>}
      </SelectContent>
    </Select>
  );
};

export const ProviderSelectorConfig = (params: {
  disabled: boolean;
  account_type: "git" | "docker";
  selected: string | undefined;
  onSelect: (id: string) => void;
  https?: boolean;
  onHttpsSwitch?: () => void;
}) => {
  const select =
    params.account_type === "git" ? "git provider" : "docker registry";
  return (
    <ConfigItem
      label={`${params.account_type} Provider`}
      description={`Select ${select} domain`}
    >
      {params.account_type === "git" ? (
        <div className="flex items-center justify-end gap-2 w-[75%]">
          <Button
            variant="ghost"
            onClick={params.onHttpsSwitch}
            className="py-0 px-2"
          >
            {`http${params.https ? "s" : ""}://`}
          </Button>
          <ProviderSelector {...params} />
        </div>
      ) : (
        <ProviderSelector {...params} />
      )}
    </ConfigItem>
  );
};

export const AccountSelector = ({
  disabled,
  id,
  type,
  account_type,
  provider,
  selected,
  onSelect,
}: {
  disabled: boolean;
  type: "Server" | "Builder" | "None";
  id?: string;
  account_type: "git" | "docker";
  provider: string;
  selected: string | undefined;
  onSelect: (id: string) => void;
}) => {
  const [db_request, config_request]:
    | ["ListGitProviderAccounts", "ListGitProvidersFromConfig"]
    | ["ListDockerRegistryAccounts", "ListDockerRegistriesFromConfig"] =
    account_type === "git"
      ? ["ListGitProviderAccounts", "ListGitProvidersFromConfig"]
      : ["ListDockerRegistryAccounts", "ListDockerRegistriesFromConfig"];
  const config_params =
    type === "None" ? {} : { target: id ? { type, id } : undefined };
  const db_accounts = useRead(db_request, {}).data?.filter(
    (account) => account.domain === provider
  );
  const config_providers = useRead(config_request, config_params).data?.filter(
    (_provider) => _provider.domain === provider
  );

  const _accounts = new Set<string>();
  for (const account of db_accounts ?? []) {
    if (account.username) {
      _accounts.add(account.username);
    }
  }
  for (const provider of config_providers ?? []) {
    for (const account of provider.accounts ?? []) {
      _accounts.add(account.username);
    }
  }
  const accounts = [..._accounts];
  accounts.sort();
  return (
    <Select
      value={selected}
      onValueChange={(value) => {
        onSelect(value === "Empty" ? "" : value);
      }}
      disabled={disabled}
    >
      <SelectTrigger
        className="w-full lg:w-[200px] max-w-[50%]"
        disabled={disabled}
      >
        <SelectValue placeholder="Select Account" />
      </SelectTrigger>
      <SelectContent>
        <SelectItem value={"Empty"}>None</SelectItem>
        {accounts
          ?.filter((account) => account)
          .map((account) => (
            <SelectItem key={account} value={account}>
              {account}
            </SelectItem>
          ))}
      </SelectContent>
    </Select>
  );
};

export const AccountSelectorConfig = (params: {
  disabled: boolean;
  id?: string;
  type: "Server" | "Builder" | "None";
  account_type: "git" | "docker";
  provider: string;
  selected: string | undefined;
  onSelect: (id: string) => void;
  placeholder: string;
}) => {
  return (
    <ConfigItem
      label="Account"
      description="Select the account used to log in to the provider"
    >
      <AccountSelector {...params} />
    </ConfigItem>
  );
};

export const InputList = <T extends { [key: string]: unknown }>({
  field,
  values,
  disabled,
  set,
  placeholder,
  className,
}: {
  field: keyof T;
  values: string[];
  disabled: boolean;
  set: (update: Partial<T>) => void;
  placeholder?: string;
  className?: string;
}) => (
  <div className="flex justify-end w-full">
    <div className="flex flex-col gap-4 w-fit">
      {values.map((arg, i) => (
        <div className="w-full flex gap-4" key={i}>
          <Input
            placeholder={placeholder}
            value={arg}
            onChange={(e) => {
              values[i] = e.target.value;
              set({ [field]: [...values] } as Partial<T>);
            }}
            disabled={disabled}
            className={cn("w-[400px] max-w-full", className)}
          />
          {!disabled && (
            <Button
              variant="secondary"
              onClick={() =>
                set({
                  [field]: [...values.filter((_, idx) => idx !== i)],
                } as Partial<T>)
              }
            >
              <MinusCircle className="w-4 h-4" />
            </Button>
          )}
        </div>
      ))}
    </div>
  </div>
);

interface ConfirmUpdate2Props<T> {
  previous: T;
  content: Partial<T>;
  onConfirm: () => void;
  disabled: boolean;
}

export function ConfirmUpdate2<T>({
  previous,
  content,
  onConfirm,
  disabled,
}: ConfirmUpdate2Props<T>) {
  const [open, set] = useState(false);
  return (
    <Dialog open={open} onOpenChange={set}>
      <DialogTrigger asChild>
        <Button
          onClick={() => set(true)}
          disabled={disabled}
          className="flex items-center gap-2"
        >
          <Save className="w-4 h-4" />
          Save
        </Button>
      </DialogTrigger>
      <DialogContent className="max-w-[800px]">
        <DialogHeader>
          <DialogTitle>Confirm Update</DialogTitle>
        </DialogHeader>
        <div className="flex flex-col gap-4 py-4 my-4 max-h-[70vh] overflow-auto">
          {Object.entries(content).map(([key, val], i) => (
            <ConfirmUpdateItem
              key={i}
              _key={key as any}
              val={val as any}
              previous={previous}
            />
          ))}
          {/* <pre className="h-[300px] overflow-auto">{content}</pre> */}
        </div>
        <DialogFooter>
          <ConfirmButton
            title="Update"
            icon={<CheckCircle className="w-4 h-4" />}
            onClick={() => {
              onConfirm();
              set(false);
            }}
          />
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
function ConfirmUpdateItem<T>({
  _key,
  val: _val,
  previous,
}: {
  _key: keyof T;
  val: T[keyof T];
  previous: T;
}) {
  const [show, setShow] = useState(true);
  const val =
    typeof _val === "string"
      ? _key === "environment" ||
        _key === "build_args" ||
        _key === "secret_args"
        ? _val
            .split("\n")
            .filter((line) => !line.startsWith("#"))
            .map((line) => line.split(" #")[0])
            .join("\n")
        : _val
      : JSON.stringify(_val, null, 2);
  const prev_val =
    typeof previous[_key] === "string"
      ? previous[_key]
      : _key === "environment" ||
        _key === "build_args" ||
        _key === "secret_args"
      ? env_to_text(previous[_key] as any) ?? ""
      : JSON.stringify(previous[_key], null, 2);
  const showDiff =
    val.includes("\n") ||
    prev_val.includes("\n") ||
    Math.max(val.length, prev_val.length) > 30;
  return (
    <div
      className={cn("mr-6 flex flex-col gap-2", val === prev_val && "hidden")}
    >
      <Card>
        <CardHeader className="p-4 flex flex-row justify-between items-center">
          <h1 className={text_color_class_by_intention("Neutral")}>
            {snake_case_to_upper_space_case(_key as string)}
          </h1>
          <ShowHideButton show={show} setShow={setShow} />
        </CardHeader>
        {show && (
          <CardContent>
            {showDiff ? (
              <MonacoDiffEditor
                original={prev_val}
                modified={val}
                language="yaml"
              />
            ) : (
              <pre style={{ minHeight: 0 }}>
                <span className={text_color_class_by_intention("Critical")}>
                  {prev_val}
                </span>{" "}
                <span className="text-muted-foreground">{"->"}</span>{" "}
                <span className={text_color_class_by_intention("Good")}>
                  {val}
                </span>
              </pre>
            )}
          </CardContent>
        )}
      </Card>
    </div>
  );
}

export const SystemCommand = ({
  value,
  disabled,
  set,
}: {
  value?: Types.SystemCommand;
  disabled: boolean;
  set: (value: Types.SystemCommand) => void;
}) => {
  return (
    <div className="w-full flex justify-end flex-wrap">
      <div className="flex items-center gap-4">
        <div className="grid gap-2">
          <div className="text-muted-foreground">Path:</div>
          <Input
            placeholder="Command working directory"
            value={value?.path}
            className="w-[200px] lg:w-[300px]"
            onChange={(e) => set({ ...(value || {}), path: e.target.value })}
            disabled={disabled}
          />
        </div>
        <div className="grid gap-2">
          <div className="text-muted-foreground">Command:</div>
          <TextUpdateMenu
            title="Update Command"
            placeholder="Set shell command"
            value={value?.command}
            onUpdate={(command) => set({ ...(value || {}), command })}
            triggerClassName="w-[200px] lg:w-[300px] xl:w-[400px]"
            disabled={disabled}
          />
        </div>
      </div>
    </div>
  );
};

export const AddExtraArgMenu = ({
  onSelect,
  type,
  disabled,
}: {
  onSelect: (suggestion: string) => void;
  type: "Deployment" | "Build" | "Stack" | "StackBuild";
  disabled?: boolean;
}) => {
  const [open, setOpen] = useState(false);
  const [search, setSearch] = useState("");
  const suggestions = useRead(`ListCommon${type}ExtraArgs`, {}).data ?? [];

  const filtered = filterBySplit(suggestions, search, (item) => item);

  return (
    <Popover open={open} onOpenChange={setOpen}>
      <PopoverTrigger asChild>
        <Button
          variant="secondary"
          className="flex items-center gap-2 w-[200px]"
          disabled={disabled}
        >
          <PlusCircle className="w-4 h-4" /> Add Extra Arg
        </Button>
      </PopoverTrigger>
      <PopoverContent className="w-[300px] max-h-[400px] p-0" align="end">
        <Command shouldFilter={false}>
          <CommandInput
            placeholder="Search suggestions"
            className="h-9"
            value={search}
            onValueChange={setSearch}
          />
          <CommandList>
            <CommandEmpty className="flex justify-evenly items-center">
              No Suggestions Found
              <SearchX className="w-3 h-3" />
            </CommandEmpty>

            <CommandGroup>
              <CommandItem
                onSelect={() => {
                  onSelect("");
                  setOpen(false);
                }}
                className="w-full cursor-pointer"
              >
                Empty Extra Arg
              </CommandItem>

              {filtered?.map((suggestion) => (
                <CommandItem
                  key={suggestion}
                  onSelect={() => {
                    onSelect(suggestion);
                    setOpen(false);
                  }}
                  className="w-full overflow-hidden overflow-ellipsis cursor-pointer"
                >
                  {suggestion}
                </CommandItem>
              ))}
            </CommandGroup>
          </CommandList>
        </Command>
      </PopoverContent>
    </Popover>
  );
};

const IMAGE_REGISTRY_DESCRIPTION = "Configure where the built image is pushed.";

export const ImageRegistryConfig = ({
  registry: _registry,
  setRegistry,
  disabled,
  resource_id,
  registry_types,
}: {
  registry: Types.ImageRegistry | undefined;
  setRegistry: (registry: Types.ImageRegistry) => void;
  disabled: boolean;
  // For builds, its builder id. For servers, its server id.
  resource_id?: string;
  registry_types?: Types.ImageRegistry["type"][];
}) => {
  const registry = _registry ?? default_registry_config("None");

  // This is the only way to get organizations for now
  const config_provider = useRead("ListDockerRegistriesFromConfig", {
    target: resource_id ? { type: "Builder", id: resource_id } : undefined,
  }).data?.find((provider) => {
    if (registry.type === "Standard") {
      return provider.domain === registry.params.domain;
    } else {
      return false;
    }
  });

  if (registry.type === "None") {
    return (
      <ConfigItem
        label="Image Registry"
        description={IMAGE_REGISTRY_DESCRIPTION}
      >
        <RegistryTypeSelector
          registry={registry}
          setRegistry={setRegistry}
          disabled={disabled}
          registry_types={registry_types}
        />
      </ConfigItem>
    );
  }

  const organizations = config_provider?.organizations ?? [];

  return (
    <>
      <ConfigItem
        label="Image Registry"
        description={IMAGE_REGISTRY_DESCRIPTION}
      >
        <div className="flex items-center justify-stretch gap-4">
          <ProviderSelector
            disabled={disabled}
            account_type="docker"
            selected={registry.params?.domain}
            onSelect={(domain) =>
              setRegistry({
                ...registry,
                params: { ...registry.params, domain },
              })
            }
            showCustom={false}
          />
          <RegistryTypeSelector
            registry={registry}
            setRegistry={setRegistry}
            disabled={disabled}
            registry_types={registry_types}
          />
        </div>
      </ConfigItem>
      {organizations.length > 0 && (
        <ConfigItem
          label="Organization"
          description="Push the build under an organization namespace, rather than the account namespace."
        >
          <OrganizationSelector
            organizations={organizations}
            selected={registry.params?.organization!}
            set={(organization) =>
              setRegistry({
                ...registry,
                params: { ...registry.params, organization },
              })
            }
            disabled={disabled}
          />
        </ConfigItem>
      )}
      <ConfigItem
        label="Account"
        description="Select the account used to authenticate against the registry."
      >
        <AccountSelector
          id={resource_id}
          type="Builder"
          account_type="docker"
          provider={registry.params.domain!}
          selected={registry.params.account}
          onSelect={(account) =>
            setRegistry({
              ...registry,
              params: { ...registry.params, account },
            })
          }
          disabled={disabled}
        />
      </ConfigItem>
    </>
  );
};

const REGISTRY_TYPES: Types.ImageRegistry["type"][] = ["None", "Standard"];

const RegistryTypeSelector = ({
  registry,
  setRegistry,
  registry_types = REGISTRY_TYPES,
  disabled,
}: {
  registry: Types.ImageRegistry;
  setRegistry: (registry: Types.ImageRegistry) => void;
  registry_types?: Types.ImageRegistry["type"][];
  disabled: boolean;
}) => {
  return (
    <Select
      value={registry.type}
      onValueChange={(type: Types.ImageRegistry["type"]) => {
        setRegistry(default_registry_config(type));
      }}
      disabled={disabled}
    >
      <SelectTrigger
        className="w-full lg:w-[200px] max-w-[50%]"
        disabled={disabled}
      >
        <SelectValue placeholder="Select Registry" />
      </SelectTrigger>
      <SelectContent align="end">
        {registry_types.map((type) => {
          return (
            <SelectItem key={type} value={type}>
              {type}
            </SelectItem>
          );
        })}
      </SelectContent>
    </Select>
  );
};

const OrganizationSelector = ({
  organizations,
  selected,
  set,
  disabled,
}: {
  organizations: string[];
  selected: string;
  set: (org: string) => void;
  disabled: boolean;
}) => {
  const [customMode, setCustomMode] = useState(false);
  if (customMode || organizations.length === 0) {
    return (
      <Input
        placeholder="Input custom organization name"
        value={selected}
        onChange={(e) => set(e.target.value)}
        className="max-w-[75%] lg:max-w-[400px]"
        onBlur={() => setCustomMode(false)}
        onKeyDown={(e) => {
          if (e.key === "Enter") {
            setCustomMode(false);
          }
        }}
        autoFocus
      />
    );
  }

  const orgs =
    selected === "" || organizations.includes(selected)
      ? organizations
      : [...organizations, selected];
  orgs.sort();

  return (
    <Select
      value={selected}
      onValueChange={(organization) => {
        if (organization === "Custom") {
          set("");
          setCustomMode(true);
        } else if (organization === "Empty") {
          set("");
        } else {
          set(organization);
        }
      }}
      disabled={disabled}
    >
      <SelectTrigger
        className="w-full lg:w-[200px] max-w-[50%]"
        disabled={disabled}
      >
        <SelectValue placeholder="Select Organization" />
      </SelectTrigger>
      <SelectContent>
        <SelectItem value={"Empty"}>None</SelectItem>
        {orgs
          ?.filter((org) => org)
          .map((org) => (
            <SelectItem key={org} value={org}>
              {org}
            </SelectItem>
          ))}
        <SelectItem value={"Custom"}>Custom</SelectItem>
      </SelectContent>
    </Select>
  );
};

const default_registry_config = (
  type: Types.ImageRegistry["type"]
): Types.ImageRegistry => {
  switch (type) {
    case "None":
      return { type, params: {} };
    case "Standard":
      return {
        type,
        params: { domain: "docker.io", account: "", organization: "" },
      };
  }
};

export const SecretSelector = ({
  keys,
  onSelect,
  type,
  disabled,
}: {
  keys: string[];
  onSelect: (key: string) => void;
  type: "Variable" | "Secret";
  disabled: boolean;
}) => {
  const [open, setOpen] = useState(false);
  const [search, setSearch] = useState("");
  const filtered = filterBySplit(keys, search, (item) => item).sort((a, b) => {
    if (a > b) {
      return 1;
    } else if (a < b) {
      return -1;
    } else {
      return 0;
    }
  });
  return (
    <Popover open={open} onOpenChange={setOpen}>
      <PopoverTrigger asChild>
        <Button variant="secondary" className="flex gap-2" disabled={disabled}>
          <PlusCircle className="w-4 h-4" />
          <div>Add {type}</div>
        </Button>
      </PopoverTrigger>
      <PopoverContent className="w-[300px] max-h-[300px] p-0" align="start">
        <Command shouldFilter={false}>
          <CommandInput
            placeholder={`Search ${type}s`}
            className="h-9"
            value={search}
            onValueChange={setSearch}
          />
          <CommandList>
            <CommandEmpty className="flex justify-evenly items-center pt-2">
              {`No ${type}s Found`}
              <SearchX className="w-3 h-3" />
            </CommandEmpty>

            <CommandGroup>
              {filtered.map((key) => (
                <CommandItem
                  key={key}
                  onSelect={() => {
                    onSelect(key);
                    setOpen(false);
                  }}
                  className="flex items-center justify-between cursor-pointer"
                >
                  <div className="p-1">{key}</div>
                </CommandItem>
              ))}
            </CommandGroup>
          </CommandList>
        </Command>
      </PopoverContent>
    </Popover>
  );
};

export const PermissionLevelSelector = ({
  level,
  onSelect,
}: {
  level: Types.PermissionLevel;
  onSelect: (level: Types.PermissionLevel) => void;
}) => {
  return (
    <Select
      value={level}
      onValueChange={(value) => onSelect(value as Types.PermissionLevel)}
    >
      <SelectTrigger className="w-32 capitalize">
        <SelectValue />
      </SelectTrigger>
      <SelectContent className="w-32">
        {Object.keys(Types.PermissionLevel).map((permission) => (
          <SelectItem
            value={permission}
            key={permission}
            className="capitalize"
          >
            {permission}
          </SelectItem>
        ))}
      </SelectContent>
    </Select>
  );
};
