import {
  Card,
  CardHeader,
  CardTitle,
  CardDescription,
  CardContent,
} from "@ui/card";
import { PieChart } from "react-minimal-pie-chart";
import { Link } from "react-router-dom";
import { useRead } from "@hooks";
import { ServerStatus } from "@monitor/client/dist/types";

export const ServersChart = () => {
  const { data } = useRead("ListServers", {});

  const running = data?.filter((d) => d.status === ServerStatus.Ok).length;
  const stopped = data?.filter((d) => d.status === ServerStatus.NotOk).length;
  const not_deployed = data?.filter(
    (d) => d.status === ServerStatus.Disabled
  ).length;

  return (
    <Link to="/servers" className="w-full">
      <Card hoverable>
        <CardHeader className="flex-row justify-between items-center">
          <CardTitle>Servers</CardTitle>
        </CardHeader>
        <CardContent className="flex gap-4 items-center w-full h-[200px]">
          <div className="flex flex-col gap-2 text-muted-foreground w-full">
            <CardDescription>
              <span className="text-green-500 font-bold">{running} </span>
              Healthy
            </CardDescription>
            <CardDescription>
              <span className="text-red-500 font-bold">{stopped} </span>
              Unhealthy
            </CardDescription>
            <CardDescription>
              <span className="text-blue-500 font-bold">{not_deployed} </span>
              Disabled
            </CardDescription>
          </div>
          <div className="flex justify-end items-center w-full">
            <PieChart
              className="w-32"
              segmentsShift={0.15}
              lineWidth={30}
              data={[
                {
                  color: "#22C55E",
                  value: running ?? 0,
                  title: "deployed",
                  key: "deployed",
                },
                {
                  color: "#EF0044",
                  value: stopped ?? 0,
                  title: "stopped",
                  key: "stopped",
                },
                {
                  color: "#3B82F6",
                  value: not_deployed ?? 0,
                  title: "not-deployed",
                  key: "not-deployed",
                },
              ]}
            />
          </div>
        </CardContent>
      </Card>
    </Link>
  );
};
