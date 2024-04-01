import { homeViewAtom } from "@components/topbar";
import { useAtom } from "jotai";
import { Dashboard } from "./dashboard";
import { AllResources } from "./all_resources";
import { Tree } from "./tree";

export const Home = () => {
  const [view, _] = useAtom(homeViewAtom);
  switch (view) {
    case "Dashboard":
      return <Dashboard />;
    case "Resources":
      return <AllResources />;
    case "Tree":
      return <Tree />;
  }
};
