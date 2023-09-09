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

export const ServersChart = () => {
  const { data } = useRead("GetServersSummary", {});
  return (
    <Link to="/servers/" className="w-full">
      <Card>
        <CardHeader className="justify-between">
          <div>
            <CardTitle>Servers</CardTitle>
            <CardDescription>{data?.total} Total</CardDescription>
          </div>
          <Server className="w-4 h-4" />
        </CardHeader>
        <CardContent className="flex h-[200px] items-center justify-between">
          <div className="flex flex-col gap-2 text-muted-foreground w-full">
            <CardDescription>
              <span className="text-green-500 font-bold">{data?.healthy} </span>
              Healthy
            </CardDescription>
            <CardDescription>
              <span className="text-red-500 font-bold">{data?.unhealthy} </span>
              Unhealthy
            </CardDescription>
            <CardDescription>
              <span className="text-blue-500 font-bold">{data?.disabled} </span>
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
                  color: "#22C55E",
                  value: data?.healthy ?? 0,
                  title: "healthy",
                  key: "healthy",
                },
                {
                  color: "#EF0044",
                  value: data?.unhealthy ?? 0,
                  title: "unhealthy",
                  key: "unhealthy",
                },
                {
                  color: "#3B82F6",
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
