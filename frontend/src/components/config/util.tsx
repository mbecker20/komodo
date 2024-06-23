/* eslint-disable @typescript-eslint/no-explicit-any */
import { useRead } from "@lib/hooks";
import { Types } from "@monitor/client";
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
  MinusCircle,
  PlusCircle,
  Save,
  SearchX,
} from "lucide-react";
import { ReactNode, useState } from "react";
import { cn, filterBySplit } from "@lib/utils";
import {
  Dialog,
  DialogContent,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from "@ui/dialog";
import { snake_case_to_upper_space_case } from "@lib/formatting";
import { ConfirmButton, TextUpdateMenu } from "@components/util";
import { Popover, PopoverContent, PopoverTrigger } from "@ui/popover";
import {
  Command,
  CommandEmpty,
  CommandGroup,
  CommandInput,
  CommandItem,
  CommandList,
} from "@ui/command";

export const ConfigItem = ({
  label,
  children,
  className,
}: {
  label?: string;
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
      {label && <div>{snake_case_to_upper_space_case(label)}</div>}
      {children}
    </div>
    <div className="w-full h-0 border-b last:hidden" />
  </>
);

export const ConfigInput = ({
  label,
  value,
  disabled,
  placeholder,
  onChange,
  onBlur,
}: {
  label: string;
  value: string | number | undefined;
  disabled?: boolean;
  placeholder?: string;
  onChange?: (value: string) => void;
  onBlur?: (value: string) => void;
}) => (
  <ConfigItem label={label}>
    <Input
      className="max-w-[75%] lg:max-w-[400px]"
      type={typeof value === "number" ? "number" : undefined}
      value={value}
      onChange={(e) => onChange && onChange(e.target.value)}
      onBlur={(e) => onBlur && onBlur(e.target.value)}
      placeholder={placeholder}
      disabled={disabled}
    />
  </ConfigItem>
);

export const ConfigSwitch = ({
  label,
  value,
  disabled,
  onChange,
}: {
  label: string;
  value: boolean | undefined;
  disabled: boolean;
  onChange: (value: boolean) => void;
}) => (
  <ConfigItem label={label}>
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

export const AccountSelectorConfig = (params: {
  disabled: boolean;
  id?: string;
  type: "Server" | "None" | "Builder";
  account_type: keyof Types.GetBuilderAvailableAccountsResponse;
  selected: string | undefined;
  onSelect: (id: string) => void;
  placeholder: string;
}) => {
  return (
    <ConfigItem label={`${params.account_type} Account`}>
      <AccountSelector {...params} />
    </ConfigItem>
  );
};

export const AccountSelector = ({
  disabled,
  id,
  type,
  account_type,
  selected,
  onSelect,
}: {
  disabled: boolean;
  id?: string;
  type: "Server" | "None" | "Builder";
  account_type: keyof Types.GetBuilderAvailableAccountsResponse;
  selected: string | undefined;
  onSelect: (id: string) => void;
}) => {
  const [request, params] =
    type === "Server" || type === "None"
      ? ["GetAvailableAccounts", { server: id }]
      : ["GetBuilderAvailableAccounts", { builder: id }];
  const accounts = useRead(request as any, params).data;
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
        {(accounts as any)?.[account_type]?.map((account: string) => (
          <SelectItem key={account} value={account}>
            {account}
          </SelectItem>
        ))}
      </SelectContent>
    </Select>
  );
};

export const AwsEcrLabelSelector = ({
  disabled,
  selected,
  onSelect,
}: {
  disabled: boolean;
  selected: string | undefined;
  onSelect: (id: string) => void;
}) => {
  const labels = useRead("GetAvailableAwsEcrLabels", {}).data;
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
        <SelectValue placeholder="Select Ecr Config" />
      </SelectTrigger>
      <SelectContent>
        <SelectItem value={"Empty"}>None</SelectItem>
        {labels?.map((label: string) => (
          <SelectItem key={label} value={label}>
            {label}
          </SelectItem>
        ))}
      </SelectContent>
    </Select>
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

interface ConfirmUpdateProps {
  content: string;
  onConfirm: () => void;
  disabled: boolean;
}

export const ConfirmUpdate = ({
  content,
  onConfirm,
  disabled,
}: ConfirmUpdateProps) => {
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
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Confirm Update</DialogTitle>
        </DialogHeader>
        <div className="flex flex-col gap-4 py-4 my-4">
          New configuration to be applied:
          <pre className="h-[300px] overflow-auto">{content}</pre>
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
};

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
  type: "Deployment" | "Build";
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

export const ImageRegistryConfig = ({
  registry,
  setRegistry,
  disabled,
  type,
  resource_id,
  registry_types,
}: {
  registry: Types.ImageRegistry | undefined;
  setRegistry: (registry: Types.ImageRegistry) => void;
  disabled: boolean;
  type: "Build" | "Deployment";
  // For builds, its builder id. For servers, its server id.
  resource_id?: string;
  registry_types?: Types.ImageRegistry["type"][];
}) => {
  const _registry = registry ?? default_registry_config("None");
  const cloud_params =
    _registry.type === "DockerHub" || _registry.type === "Ghcr"
      ? _registry.params
      : undefined;
  if (_registry.type === "None" || _registry.type === "Custom") {
    return (
      <ConfigItem label="Image Registry">
        <RegistryTypeSelector
          registry={_registry}
          setRegistry={setRegistry}
          disabled={disabled}
          deployment={type === "Deployment"}
          registry_types={registry_types}
        />
      </ConfigItem>
    );
  }
  if (_registry.type === "AwsEcr") {
    return (
      <ConfigItem label="Image Registry">
        <div className="flex items-center justify-stretch gap-4">
          <AwsEcrLabelSelector
            selected={_registry.params}
            onSelect={(label) =>
              setRegistry({
                type: "AwsEcr",
                params: label,
              })
            }
            disabled={disabled}
          />
          <RegistryTypeSelector
            registry={_registry}
            setRegistry={setRegistry}
            disabled={disabled}
            deployment={type === "Deployment"}
            registry_types={registry_types}
          />
        </div>
      </ConfigItem>
    );
  }
  return (
    <ConfigItem label="Image Registry">
      <div className="flex items-center justify-stretch gap-4">
        {type === "Build" && cloud_params?.account && (
          <OrganizationSelector
            value={cloud_params?.organization}
            set={(organization) =>
              setRegistry({
                ..._registry,
                params: { ..._registry.params, organization },
              })
            }
            disabled={disabled}
            type={_registry.type === "DockerHub" ? "Docker" : "Github"}
          />
        )}
        <AccountSelector
          id={resource_id}
          type={type === "Build" ? "Builder" : "Server"}
          account_type={_registry.type === "DockerHub" ? "docker" : "github"}
          selected={cloud_params?.account}
          onSelect={(account) =>
            setRegistry({
              ..._registry,
              params: { ..._registry.params, account },
            })
          }
          disabled={disabled}
        />
        <RegistryTypeSelector
          registry={_registry}
          setRegistry={setRegistry}
          disabled={disabled}
          deployment={type === "Deployment"}
          registry_types={registry_types}
        />
      </div>
    </ConfigItem>
  );
};

const REGISTRY_TYPES: Types.ImageRegistry["type"][] = [
  "None",
  "DockerHub",
  "Ghcr",
];

const RegistryTypeSelector = ({
  registry,
  setRegistry,
  registry_types = REGISTRY_TYPES,
  disabled,
  deployment,
}: {
  registry: Types.ImageRegistry;
  setRegistry: (registry: Types.ImageRegistry) => void;
  registry_types?: Types.ImageRegistry["type"][];
  disabled: boolean;
  deployment?: boolean;
}) => {
  return (
    <Select
      value={to_readable_registry_type(registry.type, deployment)}
      onValueChange={(type) => {
        setRegistry(
          default_registry_config(from_readable_registry_type(type, deployment))
        );
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
          const t = to_readable_registry_type(type, deployment);
          return (
            <SelectItem key={type} value={t}>
              {t}
            </SelectItem>
          );
        })}
      </SelectContent>
    </Select>
  );
};

const OrganizationSelector = ({
  value,
  set,
  disabled,
  type,
}: {
  value?: string;
  set: (org: string) => void;
  disabled: boolean;
  type: "Docker" | "Github";
}) => {
  const organizations = useRead(`List${type}Organizations`, {}).data;
  if (!organizations || organizations.length === 0) return null;
  return (
    <Select
      value={value}
      onValueChange={(v) => set(v === "Empty" ? "" : v)}
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
        {organizations?.map((org) => (
          <SelectItem key={org} value={org}>
            {org}
          </SelectItem>
        ))}
      </SelectContent>
    </Select>
  );
};

const to_readable_registry_type = (
  type: Types.ImageRegistry["type"],
  deployment?: boolean
) => {
  if (deployment && type === "None") return "Same as build";
  return type;
};

const from_readable_registry_type = (
  readable: string,
  deployment?: boolean
) => {
  if (deployment && readable === "Same as build") return "None";
  return readable as Types.ImageRegistry["type"];
};

const default_registry_config = (
  type: Types.ImageRegistry["type"]
): Types.ImageRegistry => {
  switch (type) {
    case "None":
      return { type, params: {} };
    case "DockerHub":
      return { type, params: { account: "", organization: "" } };
    case "Ghcr":
      return { type, params: { account: "", organization: "" } };
    case "AwsEcr":
      return { type, params: "" };
    case "Custom":
      return { type, params: "" };
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
