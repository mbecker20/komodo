import {
  ColorType,
  createChart,
  IChartApi,
  ISeriesApi,
} from "lightweight-charts";
import {
  Component,
  createEffect,
  createSignal,
  JSX,
  onCleanup,
  onMount,
} from "solid-js";

export type LineData = {
  title: string;
  color: string;
  priceLineVisible?: boolean;
  line: LineDataPoint[];
};

export type LineDataPoint = {
  time: number;
  value: number;
};

const LightweightChart: Component<{
  lines: () => LineData[];
  class?: string;
  style?: JSX.CSSProperties;
  width?: string;
  height?: string;
  disableScroll?: boolean;
  onCreateLineSeries?: (series: ISeriesApi<"Line">) => void;
}> = (p) => {
  let el: HTMLDivElement;
  const [chart, setChart] = createSignal<IChartApi>();
  let lineSeries: ISeriesApi<"Line">[] = [];
  const [loaded, setLoaded] = createSignal(false);
  onMount(() => {
    if (loaded()) return;
    setLoaded(true);
    const chart = createChart(el!, {
      width: el!.clientWidth,
      height: el!.clientHeight,
      layout: {
        background: { type: ColorType.Solid, color: "transparent" },
        textColor: "white",
      },
      grid: {
        horzLines: { color: "#3f454d" },
        vertLines: { color: "#3f454d" },
      },
      timeScale: { timeVisible: true },
      handleScroll: p.disableScroll ? false : true,
      handleScale: p.disableScroll ? false : true,
    });
    chart.timeScale().fitContent();
    setChart(chart);
  });
  createEffect(() => {
    if (chart()) {
      for (const series of lineSeries) {
        chart()!.removeSeries(series);
      }
      const series = p.lines().map((line) => {
        const series = chart()!.addLineSeries({
          color: line.color,
          title: line.title,
          priceLineVisible: line.priceLineVisible || false,
        });
        series.setData(line.line as any);
        if (p.onCreateLineSeries) {
          p.onCreateLineSeries(series);
        }
        return series;
      });
      chart()!.timeScale().fitContent();
      lineSeries = series;
    }
  });
  const handleResize = () => {
    if (el && chart()) {
      chart()!.applyOptions({ width: el.clientWidth });
    }
  };
  addEventListener("resize", handleResize);
  onCleanup(() => {
    chart()?.remove();
    removeEventListener("resize", handleResize);
  });
  return (
    <div
      ref={el!}
      class={p.class}
      style={{
        width: p.width || "100%",
        height: p.height || "100%",
        ...p.style,
      }}
    />
  );
};

export default LightweightChart;
