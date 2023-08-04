import { Button } from "@ui/button";
import {
  Card,
  CardHeader,
  CardTitle,
  CardDescription,
  CardContent,
} from "@ui/card";
import { Input } from "@ui/input";
import { Config, ConfigSetter } from "../Config";

export function ArrayConfig<T, D>({
  field,
  val,
  set,
  defaultNew,
  description,
  disabled,
}: {
  field: string;
  val: unknown[];
  set: ConfigSetter<T>;
  defaultNew: D;
  description?: string;
  disabled?: boolean;
}) {
  return (
    <Card>
      <CardHeader className="flex flex-row justify-between items-center">
        <div className="flex flex-col items-center">
          <CardTitle>{field.replaceAll("_", " ")}</CardTitle>
          {description && <CardDescription>{description}</CardDescription>}
        </div>
        <Button
          onClick={() =>
            set((curr) => ({ ...curr, [field]: [...val, defaultNew] }))
          }
          disabled={disabled}
        >
          +
        </Button>
      </CardHeader>
      {val.length > 0 && (
        <CardContent className="flex flex-col gap-4">
          {val.map((item, i) => {
            return (
              <div
                key={i.toString()}
                className="flex flex-row gap-4 justify-between items-center"
              >
                {typeof item === "string" ? (
                  <Input
                    value={val[i] as string}
                    onChange={(e) =>
                      set((curr) => {
                        return {
                          ...curr,
                          [field]: [
                            ...val.slice(0, i),
                            e.target.value,
                            ...val.slice(i + 1),
                          ],
                        };
                      })
                    }
                    disabled={disabled}
                  />
                ) : typeof item === "object" ? (
                  <Config
                    config={item as Record<string, unknown>}
                    update={item as Record<string, unknown>}
                    set={(upd) =>
                      set((curr) => ({
                        ...curr,
                        [field]: [
                          ...val.slice(0, i),
                          upd(item as Record<string, unknown>),
                          ...val.slice(i + 1),
                        ],
                      }))
                    }
                  />
                ) : null}
                <Button
                  onClick={() => {
                    set((curr) => ({
                      ...curr,
                      [field]: val.filter((_, index) => i !== index),
                    }));
                  }}
                  disabled={disabled}
                >
                  -
                </Button>
              </div>
            );
          })}
        </CardContent>
      )}
    </Card>
  );
}
