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
import { CheckCircle, MinusCircle, PlusCircle, Save } from "lucide-react";
import { ReactNode, useState } from "react";
import { cn } from "@lib/utils";
import {
  Dialog,
  DialogContent,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from "@ui/dialog";
import { snake_case_to_upper_space_case } from "@lib/formatting";
import { ConfirmButton } from "@components/util";

export const ConfigItem = ({
  label,
  children,
  className,
}: {
  label: string;
  children: ReactNode;
  className?: string;
}) => (
  <div
    className={cn(
      "flex justify-between items-center border-b pb-2 min-h-[60px] last:border-none last:pb-0",
      className
    )}
  >
    <div>{snake_case_to_upper_space_case(label)}</div>
    {children}
  </div>
);

export const ConfigInput = ({
  label,
  value,
  placeholder,
  disabled,
  onChange,
  onBlur,
}: {
  label: string;
  value: string | number | undefined;
  onChange?: (value: string) => void;
  onBlur?: (value: string) => void;
  placeholder?: string;
  disabled?: boolean;
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
  onChange,
}: {
  label: string;
  value: boolean | undefined;
  onChange: (value: boolean) => void;
}) => (
  <ConfigItem label={label}>
    <Switch checked={value} onCheckedChange={onChange} />
  </ConfigItem>
);

export const DoubleInput = <
  T extends object,
  K extends keyof T,
  L extends T[K] extends string | number | undefined ? K : never,
  R extends T[K] extends string | number | undefined ? K : never
>({
  values,
  leftval,
  leftpl,
  rightval,
  rightpl,
  addName,
  onLeftChange,
  onRightChange,
  onAdd,
  onRemove,
  inputClassName,
}: {
  values: T[] | undefined;
  leftval: L;
  leftpl: string;
  rightval: R;
  rightpl: string;
  addName: string;
  onLeftChange: (value: T[L], i: number) => void;
  onRightChange: (value: T[R], i: number) => void;
  onAdd: () => void;
  onRemove: (i: number) => void;
  inputClassName?: string;
}) => {
  return (
    <div className="flex flex-col gap-4">
      {values?.map((value, i) => (
        <div className="flex items-center justify-between gap-4" key={i}>
          <Input
            className={inputClassName}
            value={value[leftval] as any}
            placeholder={leftpl}
            onChange={(e) => onLeftChange(e.target.value as T[L], i)}
          />
          :
          <Input
            className={inputClassName}
            value={value[rightval] as any}
            placeholder={rightpl}
            onChange={(e) => onRightChange(e.target.value as T[R], i)}
          />
          <Button
            variant="outline"
            // intent="warning"
            onClick={() => onRemove(i)}
          >
            <MinusCircle className="w-4 h-4" />
          </Button>
        </div>
      ))}
      <Button
        variant="outline"
        // intent="success"
        className="flex items-center gap-2 w-[200px] place-self-end"
        onClick={onAdd}
      >
        <PlusCircle className="w-4 h-4" />
        Add {addName}
      </Button>
    </div>
  );
};

export const AccountSelector = ({
  id,
  type,
  account_type,
  selected,
  onSelect,
}: {
  id: string | undefined;
  type: "Server" | "Builder";
  account_type: keyof Types.GetBuilderAvailableAccountsResponse;
  selected: string | undefined;
  onSelect: (id: string) => void;
}) => {
  const [request, params] =
    type === "Server"
      ? ["GetAvailableAccounts", { server: id! }]
      : ["GetBuilderAvailableAccounts", { builder: id }];
  const accounts = useRead(request as any, params, { enabled: !!id }).data;
  return (
    <ConfigItem label={`${account_type} Account`}>
      <Select
        value={type === "Builder" ? selected || undefined : selected}
        onValueChange={onSelect}
      >
        <SelectTrigger
          className="w-full lg:w-[300px] max-w-[50%]"
          disabled={!id}
        >
          <SelectValue
            placeholder={type === "Server" ? "Same as build" : "Select Account"}
          />
        </SelectTrigger>
        <SelectContent>
          {type === "Server" && (
            <SelectItem value={" "}>Same as build</SelectItem>
          )}
          {(accounts as any)?.[account_type]?.map((account: string) => (
            <SelectItem key={account} value={account}>
              {account}
            </SelectItem>
          ))}
        </SelectContent>
      </Select>
    </ConfigItem>
  );
};

export const InputList = <T extends { [key: string]: unknown }>({
  field,
  values,
  set,
}: {
  field: keyof T;
  values: string[];
  set: (update: Partial<T>) => void;
}) => (
  <ConfigItem label={field as string} className="items-start">
    <div className="flex flex-col gap-4 w-full max-w-[400px]">
      {values.map((arg, i) => (
        <div className="w-full flex gap-4" key={i}>
          <Input
            // placeholder="--extra-arg=value"
            value={arg}
            onChange={(e) => {
              values[i] = e.target.value;
              set({ [field]: [...values] } as Partial<T>);
            }}
          />
          <Button
            variant="outline"
            // intent="warning"
            onClick={() =>
              set({
                [field]: [...values.filter((_, idx) => idx !== i)],
              } as Partial<T>)
            }
          >
            <MinusCircle className="w-4 h-4" />
          </Button>
        </div>
      ))}

      <Button
        variant="outline"
        // intent="success"
        onClick={() => set({ [field]: [...values, ""] } as Partial<T>)}
      >
        Add {snake_case_to_upper_space_case(field as string).slice(0, -1)}
      </Button>
    </div>
  </ConfigItem>
);

interface ConfirmUpdateProps {
  content: string;
  onConfirm: () => void;
}

export const ConfirmUpdate = ({ content, onConfirm }: ConfirmUpdateProps) => {
  const [open, set] = useState(false);
  return (
    <Dialog open={open} onOpenChange={set}>
      <DialogTrigger asChild>
        <Button onClick={() => set(true)}>
          <Save className="w-4 h-4" />
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
  label,
  value,
  set,
}: {
  label: string;
  value?: Types.SystemCommand;
  set: (value: Types.SystemCommand) => void;
}) => {
  return (
    <ConfigItem label={label} className="items-start">
      <div className="grid gap-2">
        <div className="flex gap-4 items-center justify-end">
          Path:
          <Input
            placeholder="command working directory"
            value={value?.path}
            className="w-[300px]"
            onChange={(e) => set({ ...(value || {}), path: e.target.value })}
          />
        </div>
        <div className="flex gap-4 items-center justify-end">
          Command:
          <Input
            placeholder="shell command"
            value={value?.command}
            className="w-[300px]"
            onChange={(e) => set({ ...(value || {}), command: e.target.value })}
          />
        </div>
      </div>
    </ConfigItem>
  );
};
