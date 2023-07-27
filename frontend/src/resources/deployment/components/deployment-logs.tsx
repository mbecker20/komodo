import { Button } from "@ui/button";
import { Card, CardHeader, CardContent } from "@ui/card";
import { Tabs, TabsList, TabsTrigger, TabsContent } from "@ui/tabs"; // import { useDeploymentLog } from "@hooks/deployments";
import { AlertOctagon, ChevronDown, TerminalSquare } from "lucide-react";
import { useEffect } from "react";
import { useRead } from "@hooks";
import { useParams } from "react-router-dom";

const scroll_to_bottom = (id: string) => () =>
  document
    .getElementById(id)
    ?.scrollIntoView({ behavior: "smooth", block: "end", inline: "nearest" });

export const DeploymentLogs = () => {
  const deployment_id = useParams().deploymentId;
  const { data, refetch } = useRead(
    "GetLog",
    { deployment_id, tail: 200 },
    { enabled: !!deployment_id }
  );

  useEffect(() => {
    const handle = setInterval(() => refetch(), 30000);
    return () => clearInterval(handle);
  }, [refetch]);

  useEffect(() => {
    scroll_to_bottom("stdout")();
    scroll_to_bottom("stderr")();
  }, [data]);

  return (
    <Tabs defaultValue="stdout">
      <div className="flex justify-between">
        <div className="flex items-center gap-2 text-muted-foreground">
          <TerminalSquare className="w-4 h-4" />
          <h2 className="text-xl">Logs</h2>
        </div>
        <TabsList className="w-fit place-self-end">
          <TabsTrigger value="stdout" onClick={scroll_to_bottom("stdout")}>
            Out
          </TabsTrigger>
          <TabsTrigger value="stderr" onClick={scroll_to_bottom("stderr")}>
            Err
            {data?.stderr && (
              <AlertOctagon className="w-4 h-4 ml-2 stroke-red-500" />
            )}
          </TabsTrigger>
        </TabsList>
      </div>
      {["stdout", "stderr"].map((t) => (
        <TabsContent key={t} className="h-full relative" value={t}>
          <div className="h-[60vh] overflow-y-scroll">
            <pre id={t}>{data?.[t as keyof typeof data] || `no ${t} logs`}</pre>
          </div>
          <Button
            className="absolute bottom-4 right-4"
            onClick={scroll_to_bottom(t)}
          >
            <ChevronDown className="h-4 w-4" />
          </Button>
        </TabsContent>
      ))}
    </Tabs>
  );
};
