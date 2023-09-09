import { useRead } from "@lib/hooks";
import { RequiredResourceComponents } from "@types";
import { GitBranch } from "lucide-react";

const useRepo = (id?: string) =>
  useRead("ListRepos", {}).data?.find((d) => d.id === id);

export const Repo: RequiredResourceComponents = {
  Name: ({ id }) => <>{useRepo(id)?.name}</>,
  Description: ({ id }) => <>{id}</>,
  Info: ({ id }) => <>{id}</>,
  Icon: () => <GitBranch className="w-4 h-4" />,
  Page: {},
  Actions: () => null,
  New: () => null,
};
