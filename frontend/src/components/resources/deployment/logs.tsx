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

const to_bottom = (id: string) => () =>
  document
    .getElementById(id)
    ?.scrollIntoView({ behavior: "smooth", block: "end", inline: "nearest" });

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

export const DeploymentLogs = ({ id }: { id: string }) => {
  const [tail, set] = useState("50");

  const { data: logs, refetch } = useRead(
    "GetLog",
    { deployment: id, tail: Number(tail) },
    { refetchInterval: 30000 }
  );

  const deployment = useDeployment(id);

  if (
    deployment?.info.state === Types.DockerContainerState.Unknown ||
    deployment?.info.state === Types.DockerContainerState.NotDeployed
  )
    return null;

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
        {["stdout", "stderr"].map((t) => (
          <TabsContent key={t} className="h-full relative" value={t}>
            <div className="h-[70vh] overflow-y-auto">
              <pre id={t} className="-scroll-mt-24">
                {logs?.[t as keyof typeof logs] || `no ${t} logs`}
              </pre>
            </div>
            <Button
              className="absolute bottom-4 right-4"
              onClick={to_bottom(t)}
            >
              <ChevronDown className="h-4 w-4" />
            </Button>
          </TabsContent>
        ))}
      </Section>
    </Tabs>
  );
};
