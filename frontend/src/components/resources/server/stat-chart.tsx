import { hex_color_by_intention } from "@lib/color";
import { useRead } from "@lib/hooks";
import { Types } from "@komodo/client";
import { useMemo } from "react";
import { useStatsGranularity } from "./hooks";
import { Loader2 } from "lucide-react";
import { AxisOptions, Chart } from "react-charts";
import { convertTsMsToLocalUnixTsInMs } from "@lib/utils";
import { useTheme } from "@ui/theme";
import { fmt_date } from "@lib/formatting";

type StatType = "cpu" | "mem" | "disk";

type StatDatapoint = { date: number; value: number };

export const StatChart = ({
  server_id,
  type,
  className,
}: {
  server_id: string;
  type: StatType;
  className?: string;
}) => {
  const [granularity] = useStatsGranularity();

  const { data, isPending } = useRead("GetHistoricalServerStats", {
    server: server_id,
    granularity,
  });

  const stats = useMemo(
    () =>
      data?.stats
        .map((stat) => {
          return {
            date: convertTsMsToLocalUnixTsInMs(stat.ts),
            value: getStat(stat, type),
          };
        })
        .reverse(),
    [data]
  );

  return (
    <div className={className}>
      {isPending ? (
        <div className="w-full max-w-full h-full flex items-center justify-center">
          <Loader2 className="w-8 h-8 animate-spin" />
        </div>
      ) : (
        stats &&
        stats.length > 0 && <InnerStatChart type={type} stats={stats} />
      )}
    </div>
  );
};

export const InnerStatChart = ({
  type,
  stats,
}: {
  type: StatType;
  stats: StatDatapoint[] | undefined;
}) => {
  const { theme: _theme } = useTheme();
  const theme =
    _theme === "system"
      ? window.matchMedia("(prefers-color-scheme: dark)").matches
        ? "dark"
        : "light"
      : _theme;
  const min = stats?.[0]?.date ?? 0;
  const max = stats?.[stats.length - 1]?.date ?? 0;
  const diff = max - min;
  const timeAxis = useMemo((): AxisOptions<StatDatapoint> => {
    return {
      getValue: (datum) => new Date(datum.date),
      hardMax: new Date(max + diff * 0.02),
      hardMin: new Date(min - diff * 0.02),
      tickCount: 6,
      formatters: {
        // scale: (value?: Date) => fmt_date(value ?? new Date()),
        tooltip: (value?: Date) => (
          <div className="text-lg font-mono">
            {fmt_date(value ?? new Date())}
          </div>
        ),
        cursor: (_value?: Date) => false,
      },
    };
  }, []);
  const valueAxis = useMemo(
    (): AxisOptions<StatDatapoint>[] => [
      {
        getValue: (datum) => datum.value,
        elementType: "area",
        min: 0,
        max: 100,
        formatters: {
          tooltip: (value?: number) => (
            <div className="text-lg font-mono">
              {(value ?? 0) >= 10 ? value?.toFixed(2) : "0" + value?.toFixed(2)}
              %
            </div>
          ),
        },
      },
    ],
    []
  );
  return (
    <Chart
      options={{
        data: [
          {
            label: type,
            data: stats ?? [],
          },
        ],
        primaryAxis: timeAxis,
        secondaryAxes: valueAxis,
        defaultColors: [getColor(type)],
        dark: theme === "dark",
        padding: {
          left: 10,
          right: 10,
        },
        // tooltip: {
        //   showDatumInTooltip: () => false,
        // },
      }}
    />
  );

  // const container_ref = useRef<HTMLDivElement>(null);
  // const line_ref = useRef<IChartApi>();
  // const series_ref = useRef<ISeriesApi<"Area">>();
  // const lineColor = getColor(type);

  // const handleResize = () =>
  //   line_ref.current?.applyOptions({
  //     width: container_ref.current?.clientWidth,
  //   });

  // useEffect(() => {
  //   if (!stats) return;
  //   if (line_ref.current) line_ref.current.remove();

  //   const init = () => {
  //     if (!container_ref.current) return;

  //     // INIT LINE
  //     line_ref.current = createChart(container_ref.current, {
  //       width: container_ref.current.clientWidth,
  //       height: container_ref.current.clientHeight,
  //       layout: {
  //         background: { type: ColorType.Solid, color: "transparent" },
  //         // textColor: "grey",
  //         fontSize: 12,
  //       },
  //       grid: {
  //         horzLines: { color: "#3f454d25" },
  //         vertLines: { color: "#3f454d25" },
  //       },
  //       timeScale: { timeVisible: true },
  //       handleScale: false,
  //       handleScroll: false,
  //     });
  //     line_ref.current.timeScale().fitContent();

  //     // INIT SERIES
  //     series_ref.current = line_ref.current.addAreaSeries({
  //       priceLineVisible: false,
  //       title: `${type} %`,
  //       lineColor,
  //       topColor: `${lineColor}B3`,
  //       bottomColor: `${lineColor}0D`,
  //     });
  //     series_ref.current.setData(stats);
  //   };

  //   // Run the effect
  //   init();
  //   window.addEventListener("resize", handleResize);
  //   return () => {
  //     window.removeEventListener("resize", handleResize);
  //   };
  // }, [stats]);

  // return <div className="w-full max-w-full h-full" ref={container_ref} />;
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
