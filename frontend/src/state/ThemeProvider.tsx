import { Component, createContext, useContext } from "solid-js";
import { useLocalStorageToggle } from "../util/hooks";

const value = () => {
	const [isDark, toggle] = useLocalStorageToggle("dark-theme", true);
	return {
		isDark,
		toggle
	};
}

export type Value = ReturnType<typeof value>;

const context = createContext<Value>();

export const ThemeProvider: Component<{}> = (p) => {
	return (
		<context.Provider value={value()}>
			{p.children}
		</context.Provider>
	);
}

export function useTheme() {
	return useContext(context) as Value;
}