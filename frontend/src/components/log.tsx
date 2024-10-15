import { logToHtml } from "@lib/utils";
import { Types } from "komodo_client";
import { Button } from "@ui/button";
import { Select, SelectContent, SelectGroup, SelectItem, SelectTrigger, SelectValue } from "@ui/select";
import { ChevronDown } from "lucide-react";
import { useEffect, useRef } from "react";

export const Log = ({
  log,
  stream,
}: {
  log: Types.Log | undefined;
  stream: "stdout" | "stderr";
}) => {
  const _log = log?.[stream as keyof typeof log] as string | undefined;
  const ref = useRef<HTMLDivElement>(null);
  const scroll = () =>
    ref.current?.scroll({
      top: ref.current.scrollHeight,
      behavior: "smooth",
    });
  useEffect(scroll, [_log]);
  return (
    <>
      <div ref={ref} className="h-[75vh] overflow-y-auto">
        <pre
          dangerouslySetInnerHTML={{
            __html: _log ? logToHtml(_log) : `no ${stream} logs`,
          }}
          className="-scroll-mt-24 pb-[20vh]"
        />
      </div>
      <Button
        variant="secondary"
        className="absolute top-4 right-4"
        onClick={scroll}
      >
        <ChevronDown className="h-4 w-4" />
      </Button>
    </>
  );
};

export const TailLengthSelector = ({
  selected,
  onSelect,
  disabled,
}: {
  selected: string;
  onSelect: (value: string) => void;
  disabled?: boolean;
}) => (
  <Select value={selected} onValueChange={onSelect} disabled={disabled}>
    <SelectTrigger className="w-[120px]">
      <SelectValue />
    </SelectTrigger>
    <SelectContent>
      <SelectGroup>
        {["100", "500", "1000", "5000"].map((length) => (
          <SelectItem key={length} value={length}>
            {length} lines
          </SelectItem>
        ))}
      </SelectGroup>
    </SelectContent>
  </Select>
);
