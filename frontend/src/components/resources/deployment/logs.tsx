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
  const [tail, set] = useState("50");
  const { data: logs, refetch } = useRead(
    "GetLog",
    { deployment: id, tail: Number(tail) },
    { refetchInterval: 30000 }
  );
  return (
    <Tabs defaultValue="stdout">
      <Section
        title="Logs"
        icon={<TerminalSquare className="w-4 h-4" />}
        actions={
          <div className="flex gap-2">
            <TabsList className="w-fit place-self-end">
              <TabsTrigger value="stdout" onClick={to_bottom("stdout")}>
                stdout
              </TabsTrigger>
              <TabsTrigger value="stderr" onClick={to_bottom("stderr")}>
                stderr
                {logs?.stderr && (
                  <AlertOctagon className="w-4 h-4 ml-2 stroke-red-500" />
                )}
              </TabsTrigger>
            </TabsList>
            <Button variant="secondary" onClick={() => refetch()}>
              <RefreshCw className="w-4 h-4" />
            </Button>
            <TailLengthSelector selected={tail} onSelect={set} />
          </div>
        }
      >
        {["stdout", "stderr"].map((t) => {
          const log = logs?.[t as keyof typeof logs] as string | undefined;
          return (
            <TabsContent key={t} className="h-full relative" value={t}>
              <div className="h-[70vh] overflow-y-auto">
                <pre
                  id={t}
                  dangerouslySetInnerHTML={{
                    __html: log ? logToHtml(log) : `no ${t} logs`,
                  }}
                  className="-scroll-mt-24"
                />
              </div>
              <Button
                className="absolute bottom-4 right-4"
                onClick={to_bottom(t)}
              >
                <ChevronDown className="h-4 w-4" />
              </Button>
            </TabsContent>
          );
        })}
      </Section>
    </Tabs>
  );
};

const TailLengthSelector = ({
  selected,
  onSelect,
}: {
  selected: string;
  onSelect: (value: string) => void;
}) => (
  <Select value={selected} onValueChange={onSelect}>
    <SelectTrigger>
      <SelectValue />
    </SelectTrigger>
    <SelectContent>
      <SelectGroup>
        {["50", "100", "500", "1000"].map((length) => (
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
