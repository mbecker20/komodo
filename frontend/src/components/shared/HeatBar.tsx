import {
  Component,
  createSignal,
  For,
  JSX,
  onCleanup,
  onMount,
} from "solid-js";
import Flex from "./layout/Flex";

const BAR_GAP = 4;
const BLUE: [number, number, number] = [24, 78, 159];
const RED: [number, number, number] = [149, 46, 35];

const HeatBar: Component<{
  total: number;
  filled: number;
  containerClass?: string;
  containerStyle?: JSX.CSSProperties;
  barHeight?: string;
  onClick?: () => void;
}> = (p) => {
  let el: HTMLDivElement;
  const [width, setWidth] = createSignal<number>();
  const handleResize = () => {
    if (el) {
      setWidth((el.clientWidth - (p.total - 1) * BAR_GAP) / p.total);
    }
  };
  onMount(() => handleResize());
  addEventListener("resize", handleResize);
  onCleanup(() => {
    removeEventListener("resize", handleResize);
  });
  return (
    <Flex
      ref={el!}
      gap={`${BAR_GAP}px`}
      class={p.containerClass}
      style={{
        cursor: p.onClick && "pointer",
        ...p.containerStyle,
      }}
      onClick={p.onClick}
    >
      <For each={[...Array(p.total).keys()]}>
        {(index) => (
          <div
            style={{
              height: p.barHeight || "2rem",
              width: `${width()!}px`,
              "background-color":
                index <= p.filled
                  ? blendColors(BLUE, RED, index / p.total)
                  : "transparent",
            }}
          />
        )}
      </For>
    </Flex>
  );
};

export default HeatBar;

function blendColors(
  [r1, g1, b1]: [number, number, number],
  [r2, b2, g2]: [number, number, number],
  perc: number /* 0-1 */
) {
  const r = Math.floor(r1 + (r2 - r1) * perc);
  const g = Math.floor(g1 + (g2 - g1) * perc);
  const b = Math.floor(b1 + (b2 - b1) * perc);
  return `rgb(${r},${g},${b})`;
}
