import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@ui/card";
import { PieChart } from "react-minimal-pie-chart";
import { useRead } from "@lib/hooks";
import { Server } from "lucide-react";
import { Link } from "react-router-dom";
import {
  hex_color_by_intention,
  text_color_class_by_intention,
} from "@lib/color";
import { cn } from "@lib/utils";

export const ServersChart = () => {
  const { data } = useRead("GetServersSummary", {});
  return (
    <Link to="/servers/">
      <Card className="hover:bg-accent/50 transition-colors cursor-pointer w-[300px]">
        <CardHeader>
          <div className="flex justify-between">
            <div>
              <CardTitle>Servers</CardTitle>
              <CardDescription>{data?.total} Total</CardDescription>
            </div>
            <Server className="w-4 h-4" />
          </div>
        </CardHeader>
        <CardContent className="hidden xl:flex h-[200px] items-center justify-between">
          <div className="flex flex-col gap-2 text-muted-foreground w-full text-nowrap">
            <CardDescription className="flex items-center gap-2">
              <span
                className={cn(
                  text_color_class_by_intention("Good"),
                  "font-bold"
                )}
              >
                {data?.healthy}{" "}
              </span>
              Healthy
            </CardDescription>
            <CardDescription className="flex items-center gap-2">
              <span
                className={cn(
                  text_color_class_by_intention("Critical"),
                  "font-bold"
                )}
              >
                {data?.unhealthy}{" "}
              </span>
              Unhealthy
            </CardDescription>
            <CardDescription className="flex items-center gap-2">
              <span
                className={cn(
                  text_color_class_by_intention("Neutral"),
                  "font-bold"
                )}
              >
                {data?.disabled}{" "}
              </span>
              Disabled
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
                  value: data?.healthy ?? 0,
                  title: "healthy",
                  key: "healthy",
                },
                {
                  color: hex_color_by_intention("Critical"),
                  value: data?.unhealthy ?? 0,
                  title: "unhealthy",
                  key: "unhealthy",
                },
                {
                  color: hex_color_by_intention("Neutral"),
                  value: data?.disabled ?? 0,
                  title: "disabled",
                  key: "disabled",
                },
              ]}
            />
          </div>
        </CardContent>
      </Card>
    </Link>
  );
};
