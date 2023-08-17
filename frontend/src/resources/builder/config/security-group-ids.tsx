import { ConfigItem } from "@components/config/util";
import { Types } from "@monitor/client";
import { Button } from "@ui/button";
import { Input } from "@ui/input";
import { MinusCircle } from "lucide-react";

export const SecurityGroupIds = ({
  ids,
  set,
}: {
  ids: string[];
  set: (update: Partial<Types.AwsBuilderConfig>) => void;
}) => {
  return (
    <ConfigItem label="Security Group Ids" className="items-start">
      <div className="flex flex-col gap-4 w-full max-w-[400px]">
        {ids.map((arg, i) => (
          <div className="w-full flex gap-4" key={i}>
            <Input
              // placeholder="--extra-arg=value"
              value={arg}
              onChange={(e) => {
                ids[i] = e.target.value;
                set({ security_group_ids: [...ids] });
              }}
            />
            <Button
              variant="outline"
              intent="warning"
              onClick={() =>
                set({
                  security_group_ids: [...ids.filter((_, idx) => idx !== i)],
                })
              }
            >
              <MinusCircle className="w-4 h-4" />
            </Button>
          </div>
        ))}

        <Button
          variant="outline"
          intent="success"
          onClick={() => set({ security_group_ids: [...ids, ""] })}
        >
          Add Security Group
        </Button>
      </div>
    </ConfigItem>
  );
};
