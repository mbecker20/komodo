import { ConfigItem } from "@components/config/util";
import { Types } from "@monitor/client";
import { Input } from "@ui/input";
import {
  Select,
  SelectContent,
  SelectGroup,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@ui/select";
import { useToast } from "@ui/use-toast";
import { useEffect, useState } from "react";

// export const TerminationSignals = ({
//   args,
//   set,
// }: {
//   args: Types.TerminationSignalLabel[];
//   set: (input: Partial<Types.DeploymentConfig>) => void;
// }) => {
//   return (
//     <ConfigItem label="Termination Signals">
//       <div>
//         {args.map((arg, i) => {
//           return <></>;
//         })}
//       </div>
//     </ConfigItem>
//   );
// };

export const DefaultTerminationSignal = ({
  arg,
  set,
}: {
  arg?: Types.TerminationSignal;
  set: (input: Partial<Types.DeploymentConfig>) => void;
}) => {
  return (
    <ConfigItem label="Default Termination Signal">
      <Select
        value={arg}
        onValueChange={(value) =>
          set({ termination_signal: value as Types.TerminationSignal })
        }
      >
        <SelectTrigger className="w-[150px]">
          <SelectValue placeholder="Select Type" />
        </SelectTrigger>
        <SelectContent>
          <SelectGroup>
            {Object.values(Types.TerminationSignal)
              .reverse()
              .map((term_signal) => (
                <SelectItem key={term_signal} value={term_signal} className="cursor-pointer">
                  {term_signal}
                </SelectItem>
              ))}
          </SelectGroup>
        </SelectContent>
      </Select>
    </ConfigItem>
  );
};

export const TerminationTimeout = ({
  arg,
  set,
}: {
  arg: number;
  set: (input: Partial<Types.DeploymentConfig>) => void;
}) => {
  const { toast } = useToast();
  const [input, setInput] = useState(arg.toString());
  useEffect(() => {
    setInput(arg.toString());
  }, [arg]);
  return (
    <ConfigItem label="Termination Timeout">
      <div className="flex items-center justify-between gap-4">
        <Input
          className="w-[100px]"
          placeholder="time in seconds"
          value={input}
          onChange={(e) => setInput(e.target.value)}
          onBlur={(e) => {
            const num = Number(e.target.value);
            if (num || num === 0) {
              set({ termination_timeout: num });
            } else {
              toast({ title: "Termination timeout must be a number" });
              setInput(arg.toString());
            }
          }}
        />
        seconds
      </div>
    </ConfigItem>
  );
};
