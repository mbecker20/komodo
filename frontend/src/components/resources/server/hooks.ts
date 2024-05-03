import { Types } from "@monitor/client";
import { useAtom } from "jotai";
import { atomWithStorage } from "jotai/utils";

const statsGranularityAtom = atomWithStorage(
  "stats-granularity-v0",
  Types.Timelength.FiveMinutes
);

export const useStatsGranularity = () => useAtom(statsGranularityAtom);
