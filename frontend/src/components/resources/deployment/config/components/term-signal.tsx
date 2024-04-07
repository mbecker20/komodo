import { ConfigItem } from "@components/config/util";
import { Types } from "@monitor/client";
import { Button } from "@ui/button";
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
import { MinusCircle, PlusCircle } from "lucide-react";
import { useEffect, useState } from "react";

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
        <SelectTrigger className="w-[200px]">
          <SelectValue placeholder="Select Type" />
        </SelectTrigger>
        <SelectContent>
          <SelectGroup>
            {Object.values(Types.TerminationSignal)
              .reverse()
              .map((term_signal) => (
                <SelectItem
                  key={term_signal}
                  value={term_signal}
                  className="cursor-pointer"
                >
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

export const TermSignalLabels = ({
  args,
  set,
}: {
  args: Types.TerminationSignalLabel[];
  set: (input: Partial<Types.DeploymentConfig>) => void;
}) => {
  const signals = Object.values(Types.TerminationSignal)
    .filter((signal) => args.every((arg) => arg.signal !== signal))
    .reverse();
  return (
    <ConfigItem label="Signal Labels" className="items-start">
      <div className="grid gap-2">
        {args.map((label, i) => (
          <div key={label.signal} className="flex gap-4 items-center w-full">
            <Input
              placeholder="Label this termination signal"
              value={label.label}
              onChange={(e) =>
                set({
                  term_signal_labels: args.map((item, index) =>
                    index === i ? { ...item, label: e.target.value } : item
                  ),
                })
              }
            />

            <Select
              value={label.signal}
              onValueChange={(value) =>
                set({
                  term_signal_labels: args.map((item, index) =>
                    index === i
                      ? { ...item, signal: value as Types.TerminationSignal }
                      : item
                  ),
                })
              }
            >
              <SelectTrigger className="w-[200px]">
                {label.signal}
              </SelectTrigger>
              <SelectContent>
                <SelectGroup>
                  {signals.map((term_signal) => (
                    <SelectItem
                      key={term_signal}
                      value={term_signal}
                      className="cursor-pointer"
                    >
                      {term_signal}
                    </SelectItem>
                  ))}
                </SelectGroup>
              </SelectContent>
            </Select>

            <Button
              variant="outline"
              size="icon"
              onClick={() =>
                set({
                  term_signal_labels: args.filter((_, index) => i !== index),
                })
              }
              className="p-2"
            >
              <MinusCircle className="w-4 h-4" />
            </Button>
          </div>
        ))}
        {signals.length > 0 && (
          <Button
            className="justify-self-end p-2"
            variant="outline"
            size="icon"
            onClick={() =>
              set({
                term_signal_labels: [
                  ...args,
                  { label: "", signal: signals[0] },
                ],
              })
            }
          >
            <PlusCircle className="w-4 h-4" />
          </Button>
        )}
      </div>
    </ConfigItem>
  );
};
