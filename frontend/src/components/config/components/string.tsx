import {
  Card,
  CardHeader,
  CardTitle,
  CardDescription,
  CardContent,
} from "@ui/card";
import { Input } from "@ui/input";
import { ConfigSetter } from "../Config";

export const StringConfig = ({
  field,
  val,
  set,
  description,
  disabled,
}: {
  field: string;
  val: string;
  set: ConfigSetter<string>;
  description?: string;
  disabled?: boolean;
}) => {
  const readable_field = field.replaceAll("_", " ");
  return (
    <Card>
      <CardHeader>
        <CardTitle>{readable_field}</CardTitle>
        {description && <CardDescription>{description}</CardDescription>}
      </CardHeader>
      <CardContent>
        <Input
          id={field}
          className="w-full"
          value={val}
          onChange={(e) => set(() => e.target.value)}
          disabled={disabled}
        />
      </CardContent>
    </Card>
  );
};
