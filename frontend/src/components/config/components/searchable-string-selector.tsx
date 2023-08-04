import { Card, CardDescription, CardHeader, CardTitle } from "@ui/card";
import { ConfigSetter } from "../Config";
import { SelectorItem, SearchableSelector } from "../Selector";

export function SearchableStringSelectorConfig({
  field,
  val,
  set,
  items,
  label,
  description,
  disabled,
}: {
  field: string;
  val: string;
  set: ConfigSetter<string>;
  items: SelectorItem[];
  label?: (val: string) => string;
  description?: string;
  disabled?: boolean;
}) {
  return (
    <Card>
      <CardHeader className="flex flex-row justify-between items-center">
        <div className="flex gap-2">
          <CardTitle>{field.replaceAll("_", " ")}</CardTitle>
          {description && <CardDescription>{description}</CardDescription>}
        </div>
        <SearchableSelector
          width="180px"
          value={{
            value: val,
            label: label ? label(val) : val,
          }}
          items={items}
          onSelect={(value) => set(() => value)}
          disabled={disabled}
        />
      </CardHeader>
    </Card>
  );
}
