import { useRead } from "@lib/hooks";
import { Types } from "@monitor/client";
import { RequiredResourceComponents } from "@types";
import {
  AlertOctagon,
  ChevronDown,
  RefreshCw,
  Rocket,
  Server,
  TerminalSquare,
} from "lucide-react";
import { cn } from "@lib/utils";
import { useState } from "react";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@ui/tabs";
import { Section } from "@components/layouts";
import { Button } from "@ui/button";
import {
  Select,
  SelectContent,
  SelectGroup,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@ui/select";
import { useServer } from "../server";
import { DeploymentConfig } from "./config";
import {
  RedeployContainer,
  StartOrStopContainer,
  RemoveContainer,
} from "./actions";

export const useDeployment = (id?: string) =>
  useRead("ListDeployments", {}).data?.find((d) => d.id === id);

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

export const Deployment: RequiredResourceComponents = {
  Name: ({ id }) => <>{useDeployment(id)?.name}</>,
  Description: ({ id }) => (
    <>{useDeployment(id)?.info.status ?? "Not Deployed"}</>
  ),
  Info: ({ id }) => (
    <div className="flex items-center gap-2">
      <Server className="w-4 h-4" />
      {useServer(useDeployment(id)?.info.server_id)?.name ?? "N/A"}
    </div>
  ),
  Icon: ({ id }) => {
    const s = useDeployment(id)?.info.state;

    const color = () => {
      if (s === Types.DockerContainerState.Running) return "fill-green-500";
      if (s === Types.DockerContainerState.Paused) return "fill-orange-500";
      if (s === Types.DockerContainerState.NotDeployed) return "fill-blue-500";
      return "fill-red-500";
    };

    return <Rocket className={cn("w-4 h-4", color())} />;
  },
  Actions: ({ id }) => (
    <div className="flex gap-4">
      <RedeployContainer id={id} />
      <StartOrStopContainer id={id} />
      <RemoveContainer id={id} />
    </div>
  ),
  Page: {
    Logs: ({ id }) => {
      const [tail, set] = useState("50");

      const { data: logs, refetch } = useRead(
        "GetLog",
        { deployment_id: id, tail: Number(tail) },
        { refetchInterval: 30000 }
      );

      const deployment = useDeployment(id);

      if (deployment?.info.state === Types.DockerContainerState.NotDeployed)
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
                <div className="h-[60vh] overflow-y-auto">
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
    },
    Config: ({ id }) => <DeploymentConfig id={id} />,
  },
};
