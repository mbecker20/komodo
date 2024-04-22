import { hex_color_by_intention } from "@lib/color";
import { useRead } from "@lib/hooks";
import { convertTsMsToLocalUnixTsInSecs } from "@lib/utils";
import { Types } from "@monitor/client";
import {
  ColorType,
  IChartApi,
  ISeriesApi,
  Time,
  createChart,
} from "lightweight-charts";
import { useEffect, useRef } from "react";
import { useStatsInterval } from "./hooks";
import { Loader2 } from "lucide-react";

type StatType = "cpu" | "mem" | "disk";

export const StatChart = ({
  server_id,
  type,
  className,
}: {
  server_id: string;
  type: StatType;
  className?: string;
}) => {
  const [interval] = useStatsInterval();

  const { data, isPending } = useRead("GetHistoricalServerStats", {
    server: server_id,
    interval,
  });

  const stats = data?.stats
    .map((stat) => {
      return {
        time: convertTsMsToLocalUnixTsInSecs(stat.ts) as Time,
        value: getStat(stat, type),
      };
    })
    .reverse();

  return (
    <div className={className}>
      {isPending ? (
        <div className="w-full max-w-full h-full flex items-center justify-center">
          <Loader2 className="w-8 h-8 animate-spin" />
        </div>
      ) : (
        <InnerStatChart type={type} stats={stats} />
      )}
    </div>
  );
};

export const InnerStatChart = ({
  type,
  stats,
}: {
  type: StatType;
  stats:
    | {
        time: Time;
        value: number;
      }[]
    | undefined;
}) => {
  const container_ref = useRef<HTMLDivElement>(null);
  const line_ref = useRef<IChartApi>();
  const series_ref = useRef<ISeriesApi<"Area">>();
  const lineColor = getColor(type);

  const handleResize = () =>
    line_ref.current?.applyOptions({
      width: container_ref.current?.clientWidth,
    });

  useEffect(() => {
    if (!stats) return;
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
          horzLines: { color: "#3f454d" },
          vertLines: { color: "#3f454d" },
        },
        timeScale: { timeVisible: true },
        handleScale: false,
        handleScroll: false,
      });
      line_ref.current.timeScale().fitContent();

      // INIT SERIES
      series_ref.current = line_ref.current.addAreaSeries({
        priceLineVisible: false,
        title: `${type} %`,
        lineColor,
        topColor: `${lineColor}B3`,
        bottomColor: `${lineColor}0D`,
      });
      series_ref.current.setData(stats);
    };

    // Run the effect
    init();
    window.addEventListener("resize", handleResize);
    return () => {
      window.removeEventListener("resize", handleResize);
    };
  }, [stats]);

  return <div className="w-full max-w-full h-full" ref={container_ref} />;
};

const getStat = (stat: Types.SystemStatsRecord, type: StatType) => {
  if (type === "cpu") return stat.cpu_perc || 0;
  if (type === "mem") return (100 * stat.mem_used_gb) / stat.mem_total_gb;
  if (type === "disk") return (100 * stat.disk_used_gb) / stat.disk_total_gb;
  return 0;
};

const getColor = (type: StatType) => {
  if (type === "cpu") return hex_color_by_intention("Good");
  if (type === "mem") return hex_color_by_intention("Warning");
  if (type === "disk") return hex_color_by_intention("Neutral");
  return hex_color_by_intention("Unknown");
};
