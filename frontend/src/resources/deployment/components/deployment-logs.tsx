import { Button } from "@ui/button";
import { Card, CardHeader, CardContent } from "@ui/card";
import { Tabs, TabsList, TabsTrigger, TabsContent } from "@ui/tabs"; // import { useDeploymentLog } from "@hooks/deployments";
import { AlertOctagon, ChevronDown } from "lucide-react";
import { useEffect } from "react";
import { useRead } from "@hooks";

const scroll_to_bottom = (id: string) => () =>
  document
    .getElementById(id)
    ?.scrollIntoView({ behavior: "smooth", block: "end", inline: "nearest" });

export const DeploymentLogs = ({
  deployment_id,
}: {
  deployment_id: string;
}) => {
  const { data, refetch } = useRead("GetLog", { deployment_id, tail: 200 });

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
      <Card>
        <CardHeader>
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
        </CardHeader>
        <CardContent>
          {["stdout", "stderr"].map((t) => (
            <TabsContent key={t} className="h-full relative" value={t}>
              <div className="h-[50vh] overflow-y-scroll">
                <pre id={t}>
                  {data?.[t as keyof typeof data] || `no ${t} logs`}
                </pre>
              </div>
              <Button
                className="absolute bottom-4 right-4"
                onClick={scroll_to_bottom(t)}
              >
                <ChevronDown className="h-4 w-4" />
              </Button>
            </TabsContent>
          ))}
        </CardContent>
      </Card>
    </Tabs>
  );
};
