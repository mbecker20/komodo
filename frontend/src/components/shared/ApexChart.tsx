import { Component, createEffect, onCleanup, onMount } from "solid-js";
import ApexCharts, { ApexOptions } from "apexcharts";

const ApexChart: Component<{ options: ApexOptions }> = (p) => {
  let element: HTMLDivElement;
  let chart: ApexCharts;

  const init = () => {
    chart = new ApexCharts(element, p.options);
    chart.render();
  };

  onMount(() => init());

  createEffect(() => chart?.updateOptions(p.options));

  onCleanup(() => chart?.destroy());

  return <div ref={element!} />;
};

export default ApexChart;
