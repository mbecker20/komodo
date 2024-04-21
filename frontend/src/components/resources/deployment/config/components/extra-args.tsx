import { ConfigItem } from "@components/config/util";
import { Types } from "@monitor/client";
import { Button } from "@ui/button";
import { Input } from "@ui/input";
import { MinusCircle, PlusCircle } from "lucide-react";

export const ExtraArgs = ({
  args,
  set,
  disabled,
}: {
  args: string[];
  set: (update: Partial<Types.DeploymentConfig>) => void;
  disabled: boolean;
}) => {
  return (
    <ConfigItem label="Extra Args" className="items-start">
      <div className="flex flex-col gap-4 w-full max-w-[400px]">
        {args.map((arg, i) => (
          <div className="w-full flex gap-4" key={i}>
            <Input
              value={arg}
              placeholder="--extra-arg=value"
              onChange={(e) => {
                args[i] = e.target.value;
                set({ extra_args: [...args] });
              }}
              disabled={disabled}
            />
            {!disabled && (
              <Button
                variant="secondary"
                onClick={() =>
                  set({ extra_args: [...args.filter((_, idx) => idx !== i)] })
                }
              >
                <MinusCircle className="w-4 h-4" />
              </Button>
            )}
          </div>
        ))}

        {!disabled && (
          <Button
            variant="secondary"
            className="flex items-center gap-2 w-[200px] place-self-end"
            onClick={() => set({ extra_args: [...args, ""] })}
          >
            <PlusCircle className="w-4 h-4" /> Add Extra Arg
          </Button>
        )}
      </div>
    </ConfigItem>
  );
};
