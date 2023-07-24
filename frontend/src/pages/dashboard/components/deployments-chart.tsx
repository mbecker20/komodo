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
import { DockerContainerState } from "@monitor/client/dist/types";

export const DeploymentsChart = () => {
  const { data, isLoading, isError } = useRead({
    type: "ListDeployments",
    params: {},
  });

  const running = data?.filter(
    (d) => d.state === DockerContainerState.Running
  ).length;
  const stopped = data?.filter(
    (d) => d.state === DockerContainerState.Exited
  ).length;
  const not_deployed = data?.filter(
    (d) => d.state === DockerContainerState.Unknown
  ).length;

  return (
    <Link to="/deployments" className="w-full">
      <Card className="pb-4" hoverable>
        <CardHeader className="flex-row justify-between items-center">
          <CardTitle>Deployments</CardTitle>
        </CardHeader>
        <CardContent className="flex gap-4 items-center w-full">
          <WithLoading {...{ isLoading, isError }}>
            <div className="flex flex-col gap-2 text-muted-foreground w-full">
              <CardDescription>
                <span className="text-green-500 font-bold">{running} </span>
                Running
              </CardDescription>
              <CardDescription>
                <span className="text-red-500 font-bold">{stopped} </span>
                Stopped
              </CardDescription>
              <CardDescription>
                <span className="text-blue-500 font-bold">{not_deployed} </span>
                Not Deployed
              </CardDescription>
            </div>
            <div className="flex justify-end items-center w-full">
              <PieChart
                className="w-20 h-20"
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
          </WithLoading>
        </CardContent>
      </Card>
    </Link>
  );
};
