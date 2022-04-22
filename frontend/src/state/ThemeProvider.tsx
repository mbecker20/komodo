import { Component, createContext, createEffect, useContext } from "solid-js";
import { applyDarkTheme, removeDarkTheme } from "../util/helpers";
import { useLocalStorageToggle } from "../util/hooks";

const value = () => {
  const [isDark, toggleDarkTheme] = useLocalStorageToggle("dark-theme", true);

  // add dark class when new nodes added
  // const observer = new MutationObserver((mutations) => {
  //   mutations.forEach((mutation) => {
  //     mutation.addedNodes.forEach((node) => {
  //       if (node.nodeType === Node.ELEMENT_NODE) {
  // 				(node as Element).classList.add("dark");
  //       }
  //     });
  //   });
  // });

  createEffect(() => {
    if (isDark()) {
      // document.body.classList.add("dark");
			document.body.className += " dark"
      // applyDarkTheme(document.body);
      // observer.observe(document.body, { subtree: true, childList: true });
    } else {
      // document.body.classList.remove("dark");
			document.body.className = document.body.className.replace(" dark", "");
      // removeDarkTheme(document.body);
      // observer.disconnect();
    }
  });

  const themeClass = () => (isDark() ? "dark" : undefined);

  return {
    isDark,
    toggleDarkTheme,
    themeClass,
  };
};

export type Value = ReturnType<typeof value>;

const context = createContext<Value>();

export const ThemeProvider: Component<{}> = (p) => {
  return <context.Provider value={value()}>{p.children}</context.Provider>;
};

export function useTheme() {
  return useContext(context) as Value;
}
