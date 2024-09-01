import { Section } from "@components/layouts";
import { Log, TailLengthSelector } from "@components/log";
import { useRead } from "@lib/hooks";
import { Types } from "@komodo/client";
import { Button } from "@ui/button";
import { Input } from "@ui/input";
import { Switch } from "@ui/switch";
import { ToggleGroup, ToggleGroupItem } from "@ui/toggle-group";
import { useToast } from "@ui/use-toast";
import { AlertOctagon, RefreshCw, ScrollText, X } from "lucide-react";
import { useState } from "react";

export const ContainerLogs = ({
  id,
  container_name,
}: {
  /// Server id
  id: string;
  container_name: string;
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
    ? SearchLogs(id, container_name, terms, invert)
    : NoSearchLogs(id, container_name, tail, stream);

  return (
    <Section
      title="Log"
      icon={<ScrollText className="w-4 h-4" />}
      itemsCenterTitleRow
      actions={
        <div className="flex items-center gap-4 flex-wrap">
          <div className="flex items-center gap-2">
            <div className="text-muted-foreground flex gap-1">
              <div>Invert</div>
              <div className="hidden xl:block">Search</div>
            </div>
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
              className="w-[180px] xl:w-[240px]"
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

const NoSearchLogs = (
  id: string,
  container: string,
  tail: string,
  stream: string
) => {
  const { data: log, refetch } = useRead(
    "GetContainerLog",
    { server: id, container, tail: Number(tail) },
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

const SearchLogs = (
  id: string,
  container: string,
  terms: string[],
  invert: boolean
) => {
  const { data: log, refetch } = useRead("SearchContainerLog", {
    server: id,
    container,
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
