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

type LinesData = {
  title: string;
  color: string;
  priceLineVisible?: boolean;
  line: LineDataPoint[];
};

type LineDataPoint = {
  time: number;
  value: number;
};

const LightweightChart: Component<{
  style?: JSX.CSSProperties;
  class?: string;
  lines?: () => LinesData[];
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
      // handleScroll: false,
      // handleScale: false,
    });
    chart.timeScale().fitContent();
    setChart(chart);
  });
  createEffect(() => {
    if (chart() && p.lines) {
      for (const series of lineSeries) {
        chart()!.removeSeries(series);
      }
      const series = p.lines().map((line) => {
        const series = chart()!.addLineSeries({
          color: line.color,
          title: line.title,
          priceLineVisible: line.priceLineVisible || false
        });
        series.setData(line.line as any);
        return series;
      });
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
      style={{ width: "100%", height: "100%", ...p.style }}
    />
  );
};

export default LightweightChart;
