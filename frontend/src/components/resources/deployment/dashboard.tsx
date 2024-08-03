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
  ColorIntention,
  hex_color_by_intention,
  text_color_class_by_intention,
} from "@lib/color";

const Item = ({
  intention,
  label,
  count,
}: {
  intention: ColorIntention;
  label: string;
  count: number | undefined;
}) => (
  <p className="flex gap-2 text-xs text-muted-foreground">
    <span className={cn(text_color_class_by_intention(intention), "font-bold")}>
      {count}
    </span>
    {label}
  </p>
);

export const DeploymentsChart = () => {
  const summary = useRead("GetDeploymentsSummary", {}).data;

  return (
    <div className="flex items-center gap-8">
      <div className="flex flex-col gap-2 w-24">
        <Item intention="Good" label="Running" count={summary?.running} />
        <Item intention="Critical" label="Stopped" count={summary?.stopped} />
        <Item
          intention="Neutral"
          label="Not Deployed"
          count={summary?.not_deployed}
        />
        <Item intention="Unknown" label="Unknown" count={summary?.unknown} />
      </div>
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
  );

  return (
    <Link to="/deployments">
      <Card className="hover:bg-accent/50 transition-colors cursor-pointer w-[300px]">
        <CardHeader>
          <div className="flex justify-between">
            <div>
              <CardTitle>Deployments</CardTitle>
              <CardDescription>{summary?.total} Total</CardDescription>
            </div>
            <Rocket className="w-4 h-4" />
          </div>
        </CardHeader>
        <CardContent className="flex h-[200px] items-center justify-between gap-4">
          <div className="flex flex-col gap-2 text-muted-foreground w-full text-nowrap">
            <CardDescription className="flex items-center gap-2">
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
            <CardDescription className="flex items-center gap-2">
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
            <CardDescription className="flex items-center gap-2">
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
            <CardDescription className="flex items-center gap-2">
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
