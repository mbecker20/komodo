import { useParams } from "@solidjs/router";
import { ParentComponent, createContext, useContext, createSignal, createResource } from "solid-js";
import { client } from "../..";
import { useAppState } from "../../state/StateProvider";
import { SystemInformation, Timelength } from "../../types";
import { useLocalStorage } from "../../util/hooks";

export enum StatsView {
	Current = "current",
	Historical = "historical",
	Info = "info"
}

const value = () => {
	const params = useParams();
  const [view, setView] = useLocalStorage(StatsView.Current, "stats-view-v2");
  const [timelength, setTimelength] = useLocalStorage(
    Timelength.OneMinute,
    "stats-timelength-v3"
  );
	const [pollRate, setPollRate] = useLocalStorage(
    Timelength.OneSecond,
    `${params.id}-stats-poll-v3`
  );
  const [page, setPage] = createSignal(0);
	return {
		view,
		setView,
		timelength,
		setTimelength,
		page,
		setPage,
		pollRate,
		setPollRate,
	};
}

export type StatsState = ReturnType<typeof value>;

const context = createContext<StatsState>();

export const StatsProvider: ParentComponent<{}> = (p) => {
	return (
		<context.Provider value={value()}>
			{p.children}
		</context.Provider>
	);
}

export function useStatsState() {
	return useContext(context) as StatsState;
}