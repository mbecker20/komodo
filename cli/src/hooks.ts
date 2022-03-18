import { useCallback, useEffect, useState } from "react";
import { clearInterval } from "timers";

export function useSequence(): [
	current: number,
	next: () => void,
	prev: () => void
] {
	const [current, setCurrent] = useState(0);
	const next = useCallback(() => {
		setCurrent((current) => current + 1);
	}, []);
	const prev = useCallback(() => {
		setCurrent((current) => Math.max(current - 1, 0));
	}, []);
	return [current, next, prev];
}

export function useConfig<T>(
	init: T
): [T, (field: keyof T, val: number | string | boolean) => void] {
	const [config, setConfig] = useState(init);
	const set = useCallback((field: keyof T, val: string | number | boolean) => {
		setConfig((config) => ({ ...config, [field]: val }));
	}, []);
	return [config, set];
}

export function useBlinker(interval = 750) {
	const [on, setOn] = useState(false);
	useEffect(() => {
		const int = setInterval(() => {
			setOn(on => !on);
		}, interval)
		return () => clearInterval(int);
	}, []);
	return on;
}