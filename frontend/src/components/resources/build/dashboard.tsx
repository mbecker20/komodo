import {
  ColorType,
  IChartApi,
  ISeriesApi,
  Time,
  createChart,
} from "lightweight-charts";
import { useEffect, useRef } from "react";
import { useRead } from "@lib/hooks";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@ui/card";
import { Hammer } from "lucide-react";
import { Link } from "react-router-dom";
import { convertTsMsToLocalUnixTsInSecs } from "@lib/utils";

export const BuildChart = () => {
  const container_ref = useRef<HTMLDivElement>(null);
  const line_ref = useRef<IChartApi>();
  const series_ref = useRef<ISeriesApi<"Histogram">>();
  const build_stats = useRead("GetBuildMonthlyStats", {}).data;
  const summary = useRead("GetBuildsSummary", {}).data;

  const handleResize = () =>
    line_ref.current?.applyOptions({
      width: container_ref.current?.clientWidth,
    });

  useEffect(() => {
    if (!build_stats) return;
    if (line_ref.current) line_ref.current.remove();
    const init = () => {
      if (!container_ref.current) return;

      // INIT LINE
      line_ref.current = createChart(container_ref.current, {
        width: container_ref.current.clientWidth,
        height: container_ref.current.clientHeight,
        layout: {
          background: { type: ColorType.Solid, color: "transparent" },
          textColor: "grey",
          fontSize: 12,
        },
        grid: {
          horzLines: { color: "transparent" },
          vertLines: { color: "transparent" },
        },
        handleScale: false,
        handleScroll: false,
      });
      line_ref.current.timeScale().fitContent();

      // INIT SERIES
      series_ref.current = line_ref.current.addHistogramSeries({
        priceLineVisible: false,
      });
      const max = build_stats.days.reduce((m, c) => Math.max(m, c.time), 0);
      series_ref.current.setData(
        build_stats.days.map((d) => ({
          time: convertTsMsToLocalUnixTsInSecs(d.ts) as Time,
          value: d.count,
          color:
            d.time > max * 0.7
              ? "darkred"
              : d.time > max * 0.35
              ? "darkorange"
              : "darkgreen",
        })) ?? []
      );
    };

    // Run the effect
    init();
    window.addEventListener("resize", handleResize);
    return () => {
      window.removeEventListener("resize", handleResize);
    };
  }, [build_stats]);

  return (
    <Link to="/builds" className="w-full">
      <Card className="hover:bg-accent/50 transition-colors cursor-pointer">
        <CardHeader>
          <div className="flex justify-between">
            <div>
              <CardTitle>Builds</CardTitle>
              <CardDescription className="flex gap-2">
                <div>{summary?.total} Total</div> |{" "}
                <div>{build_stats?.total_time.toFixed(2)} Hours</div>
              </CardDescription>
            </div>
            <Hammer className="w-4 h-4" />
          </div>
        </CardHeader>
        <CardContent className="h-[200px]">
          <div className="w-full max-w-full h-full" ref={container_ref} />
        </CardContent>
      </Card>
    </Link>
  );
};
