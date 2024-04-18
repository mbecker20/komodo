import { Section } from "@components/layouts";
import { useRead } from "@lib/hooks";
import { RequiredResourceComponents } from "@types";
import { FolderGit, Hammer, Rocket } from "lucide-react";
import { BuildConfig } from "./config";
import { BuildChart } from "./dashboard";
import { BuildTable } from "./table";
import { DeleteResource, NewResource } from "../common";
import { DeploymentTable } from "../deployment/table";
import { RunBuild } from "./actions";
import { IconStrictId } from "./icon";

const useBuild = (id?: string) =>
  useRead("ListBuilds", {}).data?.find((d) => d.id === id);

export const BuildComponents: RequiredResourceComponents = {
  Dashboard: BuildChart,

  New: () => <NewResource type="Build" />,

  Table: BuildTable,

  Name: ({ id }) => <>{useBuild(id)?.name}</>,

  Icon: ({ id }) => {
    if (id) return <IconStrictId id={id} />;
    else return <Hammer className="w-4 h-4" />;
  },

  Status: [],

  Info: [
    ({ id }) => {
      const repo = useBuild(id)?.info.repo;
      return (
        <div className="flex items-center gap-2">
          <FolderGit className="w-4 h-4" />
          {repo}
        </div>
      );
    },
    ({ id }) => {
      const branch = useBuild(id)?.info.branch;
      return (
        <div className="flex items-center gap-2">
          <FolderGit className="w-4 h-4" />
          {branch}
        </div>
      );
    },
  ],

  Actions: [RunBuild],

  Page: {
    Deployments: ({ id }) => {
      const deployments = useRead("ListDeployments", {}).data?.filter(
        (deployment) => deployment.info.build_id === id
      );
      return (
        <Section title="Deployments" icon={<Rocket className="w-4 h-4" />}>
          <DeploymentTable deployments={deployments} />
        </Section>
      );
    },
  },

  Config: BuildConfig,

  DangerZone: ({ id }) => <DeleteResource type="Build" id={id} />,
};
