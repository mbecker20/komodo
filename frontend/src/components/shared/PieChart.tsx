import {
  Component,
  createEffect,
  createMemo,
  createSignal,
  For,
  onCleanup,
  onMount,
} from "solid-js";

export type PieChartSection = {
  title: string;
  amount: number;
  color: string;
};

const Piechart: Component<{
  title: string;
  sections: (PieChartSection | undefined)[];
  donutProportion?: number;
  seperation?: number;
}> = (p) => {
  let ref: HTMLDivElement;
  let canvas: HTMLCanvasElement;
  const [chart, setChart] = createSignal<PieChartCanvas>();
  const [selected, setSelected] = createSignal<number>();
  const sections = createMemo(
    () =>
      p.sections
        .filter((s) => s)
        .sort((a, b) => {
          if (a!.amount > b!.amount) {
            return -1;
          } else {
            return 1;
          }
        }) as PieChartSection[]
  );
  const onResize = () =>
    chart()?.updateCanvasDim(ref.clientWidth, ref.clientHeight);
  onMount(() => {
    const chart = new PieChartCanvas(
      canvas,
      sections(),
      setSelected,
      p.donutProportion,
      p.seperation
    );
    setChart(chart);
    onResize();
    window.addEventListener("resize", onResize);
  });
  onCleanup(() => {
    window.removeEventListener("resize", onResize);
  });
  createEffect(() => {
    chart()?.updateSections(sections());
    chart()?.draw();
  });
  return (
    <div
      ref={ref!}
      style={{
        width: "100%",
        height: "100%",
        "box-sizing": "border-box",
        display: "grid",
        position: "relative",
      }}
    >
      <div
        style={{
          position: "absolute",
          width: "100%",
          height: "100%",
          display: "grid",
          "place-items": "center",
        }}
      >
        <div style={{ display: "grid", gap: "0.2rem", "z-index": -1 }}>
          <h2>{p.title}</h2>
          <For each={sections()}>
            {(section, index) => (
              <div
                style={{
                  display: "flex",
                  gap: "0.5rem",
                  "justify-content": "space-between",
                  opacity:
                    selected() === undefined || selected() === index()
                      ? 1
                      : 0.5,
                }}
              >
                {section.title}:
                <div style={{ color: section.color }}>{section.amount}</div>
              </div>
            )}
          </For>
        </div>
      </div>
      <canvas ref={canvas!} style={{ "z-index": 1 }} />
    </div>
  );
};

export default Piechart;

type InnerPieChartSection = PieChartSection & {
  startAngle: number;
  endAngle: number;
};

class PieChartCanvas {
  sections: InnerPieChartSection[];
  selected?: number;
  cx = 0;
  cy = 0;
  r = 0;

  constructor(
    private canvas: HTMLCanvasElement,
    sections: PieChartSection[],
    private onSelectedUpdate: (selected: number | undefined) => void,
    private donutProportion = 0.3,
    private seperation = 0.02 // private initAngle = -Math.PI / 8
  ) {
    this.sections = [];
    this.updateSections(sections);
    this.canvas.addEventListener("mousemove", (e) => this.onMouseOver(e));
    this.canvas.addEventListener("mouseout", () => {
      this.selected = undefined;
      this.onSelectedUpdate(this.selected);
      this.draw();
    });
  }

  draw() {
    const ctx = this.canvas.getContext("2d");

    if (!ctx) {
      return;
    }

    ctx.clearRect(0, 0, this.canvas.width, this.canvas.height);

    for (const segIndex in this.sections) {
      const seg = this.sections[segIndex];
      const outerStartAngle = seg.startAngle + this.seperation;
      const outerEndAngle = seg.endAngle - this.seperation;
      const innerStartAngle =
        seg.startAngle + this.seperation / this.donutProportion;
      const innerEndAngle =
        seg.endAngle - this.seperation / this.donutProportion;

      ctx.fillStyle =
        this.selected === undefined || Number(segIndex) === this.selected
          ? seg.color
          : `${seg.color}80`;

      ctx.beginPath();
      ctx.moveTo(
        this.cx + this.donutProportion * this.r * Math.cos(innerStartAngle),
        this.cy + this.donutProportion * this.r * Math.sin(innerStartAngle)
      );
      ctx.lineTo(
        this.cx + this.r * Math.cos(outerStartAngle),
        this.cy + this.r * Math.sin(outerStartAngle)
      );
      ctx.arc(this.cx, this.cy, this.r, outerStartAngle, outerEndAngle);
      ctx.lineTo(
        this.cx + this.donutProportion * this.r * Math.cos(innerEndAngle),
        this.cy + this.donutProportion * this.r * Math.sin(innerEndAngle)
      );
      ctx.arc(
        this.cx,
        this.cy,
        this.donutProportion * this.r,
        innerEndAngle,
        innerStartAngle,
        true
      );
      ctx.fill();
    }
  }

  updateSections(sections: PieChartSection[]) {
    let startAngle = 0;
    const total = sections.reduce((prev, curr) => prev + curr.amount, 0);
    this.sections = sections.map((s) => {
      const proportion = s.amount / total;
      const rads = Math.PI * 2 * proportion;
      startAngle += rads;
      return {
        ...s,
        startAngle: startAngle - rads,
        endAngle: startAngle,
      };
    });
    this.draw();
  }

  onMouseOver(e: MouseEvent) {
    const rect = this.canvas.getBoundingClientRect();
    const x = e.x - rect.x - this.cx;
    const y = e.y - rect.y - this.cy;
    if (x * x + y * y > this.r * this.r) {
      this.selected = undefined;
      this.onSelectedUpdate(this.selected);
      this.draw();
      return;
    }
    const atan = Math.atan(y / x);
    const angle =
      x >= 0 ? (y >= 0 ? atan : 2 * Math.PI + atan) : Math.PI + atan;
    for (const secIndex in this.sections) {
      if (angle < this.sections[secIndex].endAngle) {
        this.selected = Number(secIndex);
        this.onSelectedUpdate(this.selected);
        this.draw();
        break;
      }
    }
    // console.log("x", x);
    // console.log("y", y);
    // console.log(atan);
    console.log(this.selected);
  }

  updateCanvasDim(width: number, height: number) {
    this.canvas.width = width;
    this.canvas.height = height;
    this.cx = this.canvas.width / 2;
    this.cy = this.canvas.height / 2;
    this.r =
      this.canvas.width < this.canvas.height
        ? this.canvas.width / 2 - 8
        : this.canvas.height / 2 - 8;
    this.draw();
  }
}
