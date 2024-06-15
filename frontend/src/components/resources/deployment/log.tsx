import { Section } from "@components/layouts";
import { useRead } from "@lib/hooks";
import { Types } from "@monitor/client";
import { Button } from "@ui/button";
import { RefreshCw, ChevronDown, X, AlertOctagon } from "lucide-react";
import { ReactNode, useEffect, useRef, useState } from "react";
import { useDeployment } from ".";
import {
  Select,
  SelectContent,
  SelectGroup,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@ui/select";
import { Input } from "@ui/input";
import { useToast } from "@ui/use-toast";
import { logToHtml } from "@lib/utils";
import { ToggleGroup, ToggleGroupItem } from "@ui/toggle-group";
import { Switch } from "@ui/switch";

export const DeploymentLogs = ({
  id,
  titleOther,
}: {
  id: string;
  titleOther: ReactNode;
}) => {
  const state = useDeployment(id)?.info.state;
  if (
    state === undefined ||
    state === Types.DeploymentState.Unknown ||
    state === Types.DeploymentState.NotDeployed
  ) {
    return null;
  }
  return <DeploymentLogsInner id={id} titleOther={titleOther} />;
};

const DeploymentLogsInner = ({
  id,
  titleOther,
}: {
  id: string;
  titleOther: ReactNode;
}) => {
  const { toast } = useToast();
  const [stream, setStream] = useState("stdout");
  const [tail, set] = useState("100");
  const [terms, setTerms] = useState<string[]>([]);
  const [invert, setInvert] = useState(false);
  const [search, setSearch] = useState("");

  const addTerm = () => {
    if (!search.length) return;
    if (terms.includes(search)) {
      toast({ title: "Search term is already present" });
      setSearch("");
      return;
    }
    setTerms([...terms, search]);
    setSearch("");
  };

  const clearSearch = () => {
    setSearch("");
    setTerms([]);
  };

  const { Log, refetch, stderr } = terms.length
    ? SearchLogs(id, terms, invert)
    : NoSearchLogs(id, tail, stream);

  return (
    <Section
      titleOther={titleOther}
      actions={
        <div className="flex items-center gap-4">
          <div className="flex items-center gap-2">
            <div className="text-muted-foreground">Invert Search </div>
            <Switch checked={invert} onCheckedChange={setInvert} />
          </div>
          {terms.map((term, index) => (
            <Button
              key={term}
              variant="destructive"
              onClick={() => setTerms(terms.filter((_, i) => i !== index))}
              className="flex gap-2 items-center py-0 px-2"
            >
              {term}
              <X className="w-4 h-h" />
            </Button>
          ))}
          <div className="relative">
            <Input
              placeholder="Search Logs"
              value={search}
              onChange={(e) => setSearch(e.target.value)}
              onBlur={addTerm}
              onKeyDown={(e) => {
                if (e.key === "Enter") addTerm();
              }}
              className="w-[240px]"
            />
            <Button
              variant="ghost"
              size="icon"
              onClick={clearSearch}
              className="absolute right-0 top-1/2 -translate-y-1/2"
            >
              <X className="w-4 h-4" />
            </Button>
          </div>
          <ToggleGroup type="single" value={stream} onValueChange={setStream}>
            <ToggleGroupItem value="stdout">stdout</ToggleGroupItem>
            <ToggleGroupItem value="stderr">
              stderr
              {stderr && (
                <AlertOctagon className="w-4 h-4 ml-2 stroke-red-500" />
              )}
            </ToggleGroupItem>
          </ToggleGroup>
          <Button variant="secondary" size="icon" onClick={() => refetch()}>
            <RefreshCw className="w-4 h-4" />
          </Button>
          <TailLengthSelector
            selected={tail}
            onSelect={set}
            disabled={search.length > 0}
          />
        </div>
      }
    >
      {Log}
    </Section>
  );
};

const NoSearchLogs = (id: string, tail: string, stream: string) => {
  const { data: log, refetch } = useRead(
    "GetLog",
    { deployment: id, tail: Number(tail) },
    { refetchInterval: 30000 }
  );
  return {
    Log: (
      <div className="relative">
        <Log log={log} stream={stream as "stdout" | "stderr"} />
      </div>
    ),
    refetch,
    stderr: !!log?.stderr,
  };
};

const SearchLogs = (id: string, terms: string[], invert: boolean) => {
  const { data: log, refetch } = useRead("SearchLog", {
    deployment: id,
    terms,
    combinator: Types.SearchCombinator.And,
    invert,
  });
  return {
    Log: (
      <div className="h-full relative">
        <Log log={log} stream="stdout" />
      </div>
    ),
    refetch,
    stderr: !!log?.stderr,
  };
};

const Log = ({
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

const TailLengthSelector = ({
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
