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
import { Link } from "react-router-dom";
import { cn } from "@lib/utils";
import {
  hex_color_by_intention,
  text_color_class_by_intention,
} from "@lib/color";

export const DeploymentsChart = () => {
  const summary = useRead("GetDeploymentsSummary", {}).data;

  return (
    <Link to="/deployments" className="w-full">
      <Card className="hover:bg-accent/50 transition-colors cursor-pointer">
        <CardHeader>
          <div className="flex justify-between">
            <div>
              <CardTitle>Deployments</CardTitle>
              <CardDescription>{summary?.total} Total</CardDescription>
            </div>
            <Rocket className="w-4 h-4" />
          </div>
        </CardHeader>
        <CardContent className="hidden xl:flex h-[200px] items-center justify-between">
          <div className="flex flex-col gap-2 text-muted-foreground w-full">
            <CardDescription>
              <span
                className={cn(
                  text_color_class_by_intention("Good"),
                  "font-bold"
                )}
              >
                {summary?.running}{" "}
              </span>
              Running
            </CardDescription>
            <CardDescription>
              <span
                className={cn(
                  text_color_class_by_intention("Critical"),
                  "font-bold"
                )}
              >
                {summary?.stopped}{" "}
              </span>
              Stopped
            </CardDescription>
            <CardDescription>
              <span
                className={cn(
                  text_color_class_by_intention("Neutral"),
                  "font-bold"
                )}
              >
                {summary?.not_deployed}{" "}
              </span>
              Not Deployed
            </CardDescription>
            <CardDescription>
              <span
                className={cn(
                  text_color_class_by_intention("Unknown"),
                  "font-bold"
                )}
              >
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
                  color: hex_color_by_intention("Good"),
                  value: summary?.running ?? 0,
                  title: "running",
                  key: "running",
                },
                {
                  color: hex_color_by_intention("Critical"),
                  value: summary?.stopped ?? 0,
                  title: "stopped",
                  key: "stopped",
                },
                {
                  color: hex_color_by_intention("Neutral"),
                  value: summary?.not_deployed ?? 0,
                  title: "not-deployed",
                  key: "not-deployed",
                },
                {
                  color: hex_color_by_intention("Unknown"),
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
