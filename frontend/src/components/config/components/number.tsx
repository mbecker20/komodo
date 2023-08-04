import {
  Card,
  CardHeader,
  CardTitle,
  CardDescription,
  CardContent,
} from "@ui/card";
import { Input } from "@ui/input";
import { ConfigSetter } from "../Config";

export function NumberConfig({
  field,
  val,
  set,
  disabled,
  description,
}: {
  field: string;
  val: number;
  set: ConfigSetter<number>;
  description?: string;
  disabled?: boolean;
}) {
  const readable_field = field.replaceAll("_", " ");
  return (
    <Card>
      <CardHeader>
        <CardTitle>{readable_field}</CardTitle>
        {description && <CardDescription>{description}</CardDescription>}
      </CardHeader>
      <CardContent>
        <Input
          className="w-[250px]"
          type="number"
          defaultValue={val}
          onChange={(e) => set(() => e.target.valueAsNumber)}
          disabled={disabled}
        />
      </CardContent>
    </Card>
  );
}
