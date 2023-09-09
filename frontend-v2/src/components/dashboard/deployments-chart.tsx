import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@ui/card";
import { PieChart } from "react-minimal-pie-chart";
import { useRead } from "@lib/hooks";
import { Rocket } from "lucide-react";

export const DeploymentsChart = () => {
  const summary = useRead("GetDeploymentsSummary", {}).data;

  return (
    <Card className="w-full">
      <CardHeader className="justify-between">
        <div>
          <CardTitle>Deployments</CardTitle>
          <CardDescription>{summary?.total} Total</CardDescription>
        </div>
        <Rocket className="w-4 h-4" />
      </CardHeader>
      <CardContent className="flex h-[200px] items-center justify-between">
        <div className="flex flex-col gap-2 text-muted-foreground w-full">
          <CardDescription>
            <span className="text-green-500 font-bold">
              {summary?.running}{" "}
            </span>
            Running
          </CardDescription>
          <CardDescription>
            <span className="text-red-500 font-bold">{summary?.stopped} </span>
            Stopped
          </CardDescription>
          <CardDescription>
            <span className="text-blue-500 font-bold">
              {summary?.not_deployed}{" "}
            </span>
            Not Deployed
          </CardDescription>
          <CardDescription>
            <span className="text-purple-500 font-bold">
              {summary?.unknown}{" "}
            </span>
            Unknown
          </CardDescription>
        </div>
        <div className="flex justify-end items-center w-full">
          <PieChart
            className="w-32 h-32"
            radius={42}
            lineWidth={30}
            data={[
              {
                color: "#22C55E",
                value: summary?.running ?? 0,
                title: "running",
                key: "running",
              },
              {
                color: "#EF0044",
                value: summary?.stopped ?? 0,
                title: "stopped",
                key: "stopped",
              },
              {
                color: "#3B82F6",
                value: summary?.not_deployed ?? 0,
                title: "not-deployed",
                key: "not-deployed",
              },
              {
                color: "purple",
                value: summary?.unknown ?? 0,
                title: "unknown",
                key: "unknown",
              },
            ]}
          />
        </div>
      </CardContent>
    </Card>
  );
};
