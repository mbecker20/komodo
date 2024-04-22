import { Types } from "@monitor/client";
import { useAtom } from "jotai";
import { atomWithStorage } from "jotai/utils";

const statsIntervalAtom = atomWithStorage(
  "stats-interval-v0",
  Types.Timelength.FiveMinutes
);

export const useStatsInterval = () => useAtom(statsIntervalAtom);
