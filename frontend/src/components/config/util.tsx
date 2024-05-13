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
import { CheckCircle, MinusCircle, Save } from "lucide-react";
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
import { ConfirmButton, TextUpdateMenu } from "@components/util";

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

export const AccountSelector = ({
  disabled,
  id,
  type,
  account_type,
  selected,
  onSelect,
  placeholder,
}: {
  disabled: boolean;
  id: string | undefined;
  type: "Server" | "Builder";
  account_type: keyof Types.GetBuilderAvailableAccountsResponse;
  selected: string | undefined;
  onSelect: (id: string) => void;
  placeholder: string;
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
        onValueChange={(value) => {
          onSelect(value === "Empty" ? "" : value);
        }}
        disabled={disabled}
      >
        <SelectTrigger
          className="w-full lg:w-[300px] max-w-[50%]"
          disabled={disabled || !id}
        >
          <SelectValue placeholder={placeholder} />
        </SelectTrigger>
        <SelectContent>
          <SelectItem value={"Empty"}>{placeholder}</SelectItem>
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
        <Button onClick={() => set(true)} disabled={disabled}>
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
