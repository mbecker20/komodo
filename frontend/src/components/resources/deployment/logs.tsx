import { Section } from "@components/layouts";
import { useRead } from "@lib/hooks";
import { Types } from "@monitor/client";
import { Tabs, TabsList, TabsTrigger, TabsContent } from "@ui/tabs";
import { Button } from "@ui/button";
import {
  TerminalSquare,
  AlertOctagon,
  RefreshCw,
  ChevronDown,
  X,
} from "lucide-react";
import { useState } from "react";
import { useDeployment } from ".";
import {
  Select,
  SelectContent,
  SelectGroup,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@ui/select";
import sanitizeHtml from "sanitize-html";
import Convert from "ansi-to-html";
import { Input } from "@ui/input";
import { useToast } from "@ui/use-toast";

export const DeploymentLogs = ({ id }: { id: string }) => {
  const state = useDeployment(id)?.info.state;
  if (
    state === undefined ||
    state === Types.DockerContainerState.Unknown ||
    state === Types.DockerContainerState.NotDeployed
  ) {
    return null;
  }
  return <DeploymentLogsInner id={id} />;
};

const DeploymentLogsInner = ({ id }: { id: string }) => {
  const { toast } = useToast();
  const [tail, set] = useState("100");
  const [terms, setTerms] = useState<string[]>([]);
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
    ? SearchLogs(id, terms)
    : NoSearchLogs(id, tail);

  return (
    <Tabs defaultValue="stdout">
      <Section
        title="Logs"
        icon={<TerminalSquare className="w-4 h-4" />}
        actions={
          <div className="flex gap-2">
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
                className="w-[300px]"
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
            <TabsList>
              <TabsTrigger value="stdout" onClick={to_bottom("stdout")}>
                stdout
              </TabsTrigger>
              <TabsTrigger value="stderr" onClick={to_bottom("stderr")}>
                stderr
                {stderr && (
                  <AlertOctagon className="w-4 h-4 ml-2 stroke-red-500" />
                )}
              </TabsTrigger>
            </TabsList>
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
    </Tabs>
  );
};

const NoSearchLogs = (id: string, tail: string) => {
  const { data: log, refetch } = useRead(
    "GetLog",
    { deployment: id, tail: Number(tail) },
    { refetchInterval: 30000 }
  );
  return {
    Log: (
      <>
        {["stdout", "stderr"].map((stream) => (
          <TabsContent key={stream} className="h-full relative" value={stream}>
            <Log log={log} stream={stream as "stdout" | "stderr"} />
          </TabsContent>
        ))}
      </>
    ),
    refetch,
    stderr: !!log?.stderr,
  };
};

const SearchLogs = (id: string, terms: string[]) => {
  const { data: log, refetch } = useRead("SearchLog", {
    deployment: id,
    terms,
    combinator: Types.SearchCombinator.And,
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
  return (
    <>
      <div className="h-[70vh] overflow-y-auto">
        <pre
          id={stream}
          dangerouslySetInnerHTML={{
            __html: _log ? logToHtml(_log) : `no ${stream} logs`,
          }}
          className="-scroll-mt-24"
        />
      </div>
      <Button className="absolute bottom-4 right-4" onClick={to_bottom(stream)}>
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

const to_bottom = (id: string) => () =>
  document
    .getElementById(id)
    ?.scrollIntoView({ behavior: "smooth", block: "end", inline: "nearest" });

const convert = new Convert();
/**
 * Converts the ansi colors in log to html.
 * sanitizes incoming log first for any eg. script tags.
 * @param log incoming log string
 */
const logToHtml = (log: string) => {
  if (!log) return "No log.";
  const sanitized = sanitizeHtml(log, {
    allowedTags: sanitizeHtml.defaults.allowedTags.filter(
      (tag) => tag !== "script"
    ),
    allowedAttributes: sanitizeHtml.defaults.allowedAttributes,
  });
  return convert.toHtml(sanitized);
};
