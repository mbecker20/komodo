import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@ui/card";
import { PieChart } from "react-minimal-pie-chart";
import { useRead } from "@lib/hooks";
import { Hammer } from "lucide-react";
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

export const BuildDashboard = () => {
  const summary = useRead("GetBuildsSummary", {}).data;
  return (
    <div className="flex items-center gap-8">
      <div className="flex flex-col gap-2 w-24">
        <Item intention="Good" label="Ok" count={summary?.ok} />
        <Item intention="Warning" label="Building" count={summary?.building} />
        <Item intention="Critical" label="Failed" count={summary?.failed} />
        <Item intention="Unknown" label="Unknown" count={summary?.unknown} />
      </div>
      <PieChart
        className="w-32 h-32"
        radius={42}
        lineWidth={30}
        data={[
          {
            color: hex_color_by_intention("Good"),
            value: summary?.ok ?? 0,
            title: "ok",
            key: "ok",
          },
          {
            color: hex_color_by_intention("Warning"),
            value: summary?.building ?? 0,
            title: "building",
            key: "building",
          },
          {
            color: hex_color_by_intention("Critical"),
            value: summary?.failed ?? 0,
            title: "failed",
            key: "failed",
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
    <Link to="/builds">
      <Card className="hover:bg-accent/50 transition-colors cursor-pointer w-[300px]">
        <CardHeader>
          <div className="flex justify-between">
            <div>
              <CardTitle>Builds</CardTitle>
              <CardDescription>{summary?.total} Total</CardDescription>
            </div>
            <Hammer className="w-4 h-4" />
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
                {summary?.ok}{" "}
              </span>
              Ok
            </CardDescription>
            <CardDescription className="flex items-center gap-2">
              <span
                className={cn(
                  text_color_class_by_intention("Warning"),
                  "font-bold"
                )}
              >
                {summary?.building}{" "}
              </span>
              Building
            </CardDescription>
            <CardDescription className="flex items-center gap-2">
              <span
                className={cn(
                  text_color_class_by_intention("Critical"),
                  "font-bold"
                )}
              >
                {summary?.failed}{" "}
              </span>
              Failed
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
                  value: summary?.ok ?? 0,
                  title: "ok",
                  key: "ok",
                },
                {
                  color: hex_color_by_intention("Warning"),
                  value: summary?.building ?? 0,
                  title: "building",
                  key: "building",
                },
                {
                  color: hex_color_by_intention("Critical"),
                  value: summary?.failed ?? 0,
                  title: "failed",
                  key: "failed",
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
