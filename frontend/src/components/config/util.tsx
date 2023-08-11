import { Input } from "@ui/input";
import { Switch } from "@ui/switch";

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
