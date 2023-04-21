import { Component, Show, createMemo } from "solid-js";
import { useAppState } from "../../state/StateProvider";
import Flex from "../shared/layout/Flex";
import Grid from "../shared/layout/Grid";
import LightweightChart from "../shared/LightweightChart";
import Loading from "../shared/loading/Loading";
import { COLORS } from "../../style/colors";
import { BuildStatsResponse } from "../../util/client_types";
import { useLocalStorage } from "../../util/hooks";

const BuildSummary: Component<{}> = (p) => {
  const { build_stats } = useAppState();
  return (
    <Grid class="full-size card" gridTemplateRows="auto 1fr" style={{ "padding-bottom": "0.6rem" }}>
      <Show
        when={build_stats.get()}
        fallback={
          <Grid class="full-size" placeItems="center">
            <Loading type="three-dot" />
          </Grid>
        }
      >
        <Flex
          justifyContent="space-between"
          alignItems="center"
          style={{ height: "fit-content" }}
        >
          <h2>last 30 days</h2>
          <Flex>
            <Flex alignItems="center" gap="0.5rem">
              <div class="dimmed">build time: </div>
              <h2>{build_stats.get()?.total_time.toFixed(1)} hrs</h2>
            </Flex>
            <Flex alignItems="center" gap="0.5rem">
              <div class="dimmed">build count: </div>
              <h2>{build_stats.get()?.total_count.toFixed()}</h2>
            </Flex>
          </Flex>
        </Flex>
        <BuildStatsChart build_stats={build_stats.get()!} />
      </Show>
    </Grid>
  );
};

export default BuildSummary;

const BuildStatsChart: Component<{ build_stats: BuildStatsResponse }> = (p) => {
  const [mode, setMode] = useLocalStorage<"time" | "count">(
    "time",
    "build-stats-chart-mode-v2"
  );
  const max = createMemo(() => {
    return p.build_stats.days.reduce((max, day) => {
      if (mode() === "count") {
        if (day.count > max) {
          return day.count;
        } else return max;
      } else {
        if (day.time > max) {
          return day.time;
        } else return max;
      }
    }, 0);
  });
  return (
    <div class="full-size" style={{ position: "relative" }}>
      <LightweightChart
        class="full-size"
        style={{ "min-height": "200px" }}
        histograms={[
          {
            line: p.build_stats.days.map((day) => ({
              value: mode() === "count" ? day.count : (day.time * 60),
              time: day.ts / 1000,
              color:
                mode() === "count"
                  ? day.count > max() * 0.7
                    ? COLORS.red
                    : day.count > max() * 0.35
                    ? COLORS.orange
                    : COLORS.green
                  : day.time > max() * 0.7
                  ? COLORS.red
                  : day.time > max() * 0.35
                  ? COLORS.orange
                  : COLORS.green,
            })),
            priceLineVisible: false,
            // priceFormat:
            //   mode() === "count"
            //     ? {
            //         minMove: 1,
            //       }
            //     : undefined,
            priceFormat: {
              minMove: 1,
            }
          },
        ]}
        timeVisible={false}
        options={{
          grid: {
            horzLines: { visible: false },
            vertLines: { visible: false },
          },
        }}
        disableScroll
      />
      <button
        class="blue opaque"
        style={{
          position: "absolute",
          top: 0,
          right: 0,
          "z-index": 20,
          padding: "0.3rem",
        }}
        onClick={() => setMode(mode => {
          if (mode === "count") {
            return "time"
          } else {
            return "count"
          }
        })}
      >
        {mode()}{mode() === "time" ? " (min)" : ""}
      </button>
    </div>
  );
};
