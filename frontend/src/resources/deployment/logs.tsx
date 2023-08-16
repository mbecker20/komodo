import { Button } from "@ui/button";
import { Tabs, TabsList, TabsTrigger, TabsContent } from "@ui/tabs";
import { AlertOctagon, ChevronDown, TerminalSquare } from "lucide-react";
import { useRead } from "@hooks";
import { Section } from "@layouts/page";
import { DockerContainerState } from "@monitor/client/dist/types";
import {
  Select,
  SelectContent,
  SelectGroup,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@ui/select";
import { useState } from "react";

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

export const DeploymentLogs = ({
  deployment_id,
}: {
  deployment_id: string;
}) => {
  const [tail, set] = useState("50");

  const logs = useRead(
    "GetLog",
    { deployment_id, tail: Number(tail) },
    { refetchInterval: 30000 }
  ).data;
  const deployments = useRead("ListDeployments", {}).data;
  const deployment = deployments?.find((d) => d.id === deployment_id);

  if (deployment?.info.state === DockerContainerState.NotDeployed) return null;

  return (
    <Tabs defaultValue="stdout">
      <Section
        title="Logs"
        icon={<TerminalSquare className="w-4 h-4" />}
        actions={
          <div className="flex gap-4">
            <TailLengthSelector selected={tail} onSelect={set} />
            <TabsList className="w-fit place-self-end">
              <TabsTrigger value="stdout" onClick={to_bottom("stdout")}>
                Out
              </TabsTrigger>
              <TabsTrigger value="stderr" onClick={to_bottom("stderr")}>
                Err
                {logs?.stderr && (
                  <AlertOctagon className="w-4 h-4 ml-2 stroke-red-500" />
                )}
              </TabsTrigger>
            </TabsList>
          </div>
        }
      >
        {["stdout", "stderr"].map((t) => (
          <TabsContent key={t} className="h-full relative" value={t}>
            <div className="h-[60vh] overflow-y-scroll">
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
