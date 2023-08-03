import {
  ColorType,
  IChartApi,
  ISeriesApi,
  Time,
  createChart,
} from "lightweight-charts";
import { useEffect, useRef } from "react";
import { useRead } from "@hooks";

export const BuildChart = () => {
  const container_ref = useRef<HTMLDivElement>(null);
  const line_ref = useRef<IChartApi>();
  const series_ref = useRef<ISeriesApi<"Histogram">>();
  const { data } = useRead("GetBuildMonthlyStats", {});

  const handleResize = () =>
    line_ref.current?.applyOptions({
      width: container_ref.current?.clientWidth,
    });

  useEffect(() => {
    if (!data) return;
    if (line_ref.current) line_ref.current.remove();
    const init = () => {
      if (!container_ref.current) return;
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
      series_ref.current = line_ref.current.addHistogramSeries({
        priceLineVisible: false,
      });
      const max = data.days.reduce((m, c) => Math.max(m, c.time), 0);
      series_ref.current.setData(
        data.days.map((d) => ({
          time: (d.ts / 1000) as Time,
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
    init();
    window.addEventListener("resize", handleResize);
    return () => {
      window.removeEventListener("resize", handleResize);
    };
  }, [data]);

  return <div className="w-full max-w-full h-full" ref={container_ref} />;
};
