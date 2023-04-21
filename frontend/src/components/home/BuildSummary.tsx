import { Component, Show } from "solid-js";
import { useAppState } from "../../state/StateProvider";
import Flex from "../shared/layout/Flex";
import Grid from "../shared/layout/Grid";
import LightweightChart from "../shared/LightweightChart";
import Loading from "../shared/loading/Loading";
import { COLORS } from "../../style/colors";

const BuildSummary: Component<{}> = (p) => {
  const { build_stats } = useAppState();
  return (
    <Grid class="full-size card" gridTemplateRows="auto 1fr">
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
      <Show
        when={build_stats.get()}
        fallback={
          <Grid class="full-size" placeItems="center">
            <Loading type="three-dot" />
          </Grid>
        }
      >
        <LightweightChart
          areas={[
            {
              line: build_stats.get()!.days.map((day) => ({
                value: day.count,
                time: day.ts / 1000,
              })),
              // color: COLORS.blue,
              lineColor: COLORS.blue,
              topColor: `${COLORS.blue}B3`,
              bottomColor: `${COLORS.blue}0D`,
              priceLineVisible: false,
              priceFormat: {
                minMove: 1,
              },
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
      </Show>
    </Grid>
  );
};

export default BuildSummary;
