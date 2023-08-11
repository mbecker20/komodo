import { useRead } from "@hooks";
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
import { MinusCircle, PlusCircle } from "lucide-react";

export const ConfigInput = ({
  label,
  value,
  onChange,
}: {
  label: string;
  value: string | number | undefined;
  onChange: (value: string) => void;
}) => (
  <div className="flex justify-between items-center border-b pb-4 min-h-[60px]">
    <div className="capitalize "> {label} </div>
    <Input
      className="max-w-[400px]"
      type={typeof value === "number" ? "number" : undefined}
      value={value}
      onChange={(e) => onChange(e.target.value)}
      // disabled={loading}
    />
  </div>
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
  <div className="flex justify-between items-center border-b pb-4 min-h-[60px]">
    <div className="capitalize "> {label} </div>
    <Switch checked={value} onCheckedChange={onChange} />
  </div>
);

export const DoubleInput = <
  T extends {},
  K extends keyof T,
  L extends T[K] extends string | number | undefined ? K : never,
  R extends T[K] extends string | number | undefined ? K : never
>({
  values,
  leftval,
  rightval,
  onLeftChange,
  onRightChange,
  onAdd,
  onRemove,
}: {
  values: T[] | undefined;
  leftval: L;
  rightval: R;
  onLeftChange: (value: T[L], i: number) => void;
  onRightChange: (value: T[R], i: number) => void;
  onAdd: () => void;
  onRemove: (i: number) => void;
}) => {
  return (
    <div className="flex flex-col gap-4 border-b pb-4">
      {values?.map((value, i) => (
        <div className="flex items-center justify-between gap-4" key={i}>
          <Input
            value={value[leftval]}
            placeholder="Container"
            onChange={(e) => onLeftChange(e.target.value as T[L], i)}
          />
          =
          <Input
            value={value[rightval]}
            placeholder="Container"
            onChange={(e) => onRightChange(e.target.value as T[R], i)}
          />
          <Button
            variant="outline"
            intent="warning"
            onClick={() => onRemove(i)}
          >
            <MinusCircle className="w-4 h-4" />
          </Button>
        </div>
      ))}
      <Button
        variant="outline"
        intent="success"
        className="flex items-center gap-2"
        onClick={onAdd}
      >
        <PlusCircle className="w-4 h-4" />
        Add Port
      </Button>
    </div>
  );
};

type UsableResources = Exclude<Types.ResourceTarget["type"], "System">;

export const ResourceSelector = <T extends UsableResources>({
  type,
  selected,
  onSelect,
}: {
  type: T;
  selected: string | undefined;
  onSelect: (id: string) => void;
}) => {
  const builds = useRead(`List${type}s`, {}).data;
  return (
    <Select value={selected || undefined} onValueChange={onSelect}>
      <SelectTrigger className="w-full lg:w-[300px]">
        <SelectValue placeholder="Select Build" />
      </SelectTrigger>
      <SelectContent>
        {builds?.map((b) => (
          <SelectItem key={b.id} value={b.id}>
            {b.name}
          </SelectItem>
        ))}
      </SelectContent>
    </Select>
  );
};
