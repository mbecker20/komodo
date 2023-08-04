import { Switch } from "@ui/switch";
import { Card, CardHeader, CardTitle, CardDescription } from "@ui/card";
import { ConfigSetter } from "../Config";

export const BooleanConfig = ({
  field,
  val,
  set,
  description,
  disabled,
}: {
  field: string;
  val: boolean;
  set: ConfigSetter<boolean>;
  description?: string;
  disabled?: boolean;
}) => {
  const readable_field = field.replaceAll("_", " ");
  const onChange = (checked: boolean) => set(() => checked);
  return (
    <Card
      className="flex flex-row justify-between items-center cursor-pointer"
      onClick={() => onChange(!val)}
    >
      <CardHeader>
        <CardTitle>{readable_field}</CardTitle>
        {description && <CardDescription>{description}</CardDescription>}
      </CardHeader>
      <Switch
        id={field}
        className="m-6"
        checked={val}
        onCheckedChange={onChange}
        disabled={disabled}
      />
    </Card>
  );
};
