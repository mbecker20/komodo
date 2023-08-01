import {
  Card,
  CardHeader,
  CardTitle,
  CardDescription,
  CardContent,
} from "@ui/card";
import { WithLoading } from "@components/util";
import { PieChart } from "react-minimal-pie-chart";
import { Link } from "react-router-dom";
import { useRead } from "@hooks";

export const DeploymentsChart = () => {
  const summary = useRead("GetDeploymentsSummary", {}).data;

  return (
    <Link to="/deployments" className="w-full">
      <Card className="pb-4" hoverable>
        <CardHeader className="flex-row justify-between items-center">
          <CardTitle>Deployments</CardTitle>
        </CardHeader>
        <CardContent className="flex gap-4 items-center w-full">
          <div className="flex flex-col gap-2 text-muted-foreground w-full">
            <CardDescription>
              <span className="text-green-500 font-bold">
                {summary?.running}{" "}
              </span>
              Running
            </CardDescription>
            <CardDescription>
              <span className="text-red-500 font-bold">
                {summary?.stopped}{" "}
              </span>
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
              className="w-20 h-20"
              data={[
                {
                  color: "#22C55E",
                  value: summary?.running ?? 0,
                  title: "deployed",
                  key: "deployed",
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
    </Link>
  );
};
