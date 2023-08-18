import { ConfigItem } from "@components/config/util";
import { Types } from "@monitor/client";
import { Button } from "@ui/button";
import { Input } from "@ui/input";
import { MinusCircle } from "lucide-react";

export const DockerAccounts = ({
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
                set({ docker_accounts: [...ids] });
              }}
            />
            <Button
              variant="outline"
              intent="warning"
              onClick={() =>
                set({
                  docker_accounts: [...ids.filter((_, idx) => idx !== i)],
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
          onClick={() => set({ docker_accounts: [...ids, ""] })}
        >
          Add Docker Account
        </Button>
      </div>
    </ConfigItem>
  );
};
