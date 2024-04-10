import { homeViewAtom } from "@components/topbar";
import { useAtom } from "jotai";
import { Dashboard } from "./dashboard";
import { AllResources } from "./all_resources";
import { Tree } from "./tree";
import { useSetTitle } from "@lib/hooks";

export const Home = () => {
  useSetTitle();
  const [view] = useAtom(homeViewAtom);
  switch (view) {
    case "Dashboard":
      return <Dashboard />;
    case "Resources":
      return <AllResources />;
    case "Tree":
      return <Tree />;
  }
};
