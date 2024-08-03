import { Section } from "@components/layouts";
import { useRead } from "@lib/hooks";
import { RequiredResourceComponents } from "@types";
import { FolderGit, Hammer } from "lucide-react";
import { BuildConfig } from "./config";
import { BuildTable } from "./table";
import { DeleteResource, NewResource } from "../common";
import { DeploymentTable } from "../deployment/table";
import { RunBuild } from "./actions";
import {
  bg_color_class_by_intention,
  build_state_intention,
  stroke_color_class_by_intention,
} from "@lib/color";
import { Card, CardHeader } from "@ui/card";
import { cn } from "@lib/utils";
import { useState } from "react";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@ui/tabs";
import { ResourceComponents } from "..";
import { Types } from "@monitor/client";
import { DashboardPieChart } from "@pages/home/dashboard";

const useBuild = (id?: string) =>
  useRead("ListBuilds", {}).data?.find((d) => d.id === id);

const BuildIcon = ({ id, size }: { id?: string; size: number }) => {
  const state = useBuild(id)?.info.state;
  const color = stroke_color_class_by_intention(build_state_intention(state));
  return <Hammer className={cn(`w-${size} h-${size}`, state && color)} />;
};

const ConfigOrDeployments = ({ id }: { id: string }) => {
  const [view, setView] = useState("Config");
  const deployments = useRead("ListDeployments", {}).data?.filter(
    (deployment) => deployment.info.build_id === id
  );
  const deploymentsDisabled = (deployments?.length || 0) === 0;
  const titleOther = (
    <TabsList className="justify-start w-fit">
      <TabsTrigger value="Config" className="w-[110px]">
        Config
      </TabsTrigger>
      <TabsTrigger
        value="Deployments"
        className="w-[110px]"
        disabled={deploymentsDisabled}
      >
        Deployments
      </TabsTrigger>
    </TabsList>
  );
  return (
    <Tabs
      value={deploymentsDisabled ? "Config" : view}
      onValueChange={setView}
      className="grid gap-4"
    >
      <TabsContent value="Config">
        <BuildConfig id={id} titleOther={titleOther} />
      </TabsContent>
      <TabsContent value="Deployments">
        <Section
          titleOther={titleOther}
          actions={<ResourceComponents.Deployment.New build_id={id} />}
        >
          <DeploymentTable deployments={deployments ?? []} />
        </Section>
      </TabsContent>
    </Tabs>
  );
};

export const BuildComponents: RequiredResourceComponents = {
  list_item: (id) => useBuild(id),

  Dashboard: () => {
    const summary = useRead("GetBuildsSummary", {}).data;
    return (
      <DashboardPieChart
        data={[
          { title: "Ok", intention: "Good", value: summary?.ok ?? 0 },
          {
            title: "Building",
            intention: "Warning",
            value: summary?.building ?? 0,
          },
          {
            title: "Failed",
            intention: "Critical",
            value: summary?.failed ?? 0,
          },
          {
            title: "Unknown",
            intention: "Unknown",
            value: summary?.unknown ?? 0,
          },
        ]}
      />
    );
  },

  New: () => <NewResource type="Build" />,

  Table: ({ resources }) => (
    <BuildTable builds={resources as Types.BuildListItem[]} />
  ),

  Icon: ({ id }) => <BuildIcon id={id} size={4} />,
  BigIcon: ({ id }) => <BuildIcon id={id} size={8} />,

  Status: {
    State: ({ id }) => {
      let state = useBuild(id)?.info.state;
      const color = bg_color_class_by_intention(build_state_intention(state));
      return (
        <Card className={cn("w-fit", color)}>
          <CardHeader className="py-0 px-2">{state}</CardHeader>
        </Card>
      );
    },
  },

  Info: {
    Repo: ({ id }) => {
      const repo = useBuild(id)?.info.repo;
      return (
        <div className="flex items-center gap-2">
          <FolderGit className="w-4 h-4" />
          {repo}
        </div>
      );
    },
    Branch: ({ id }) => {
      const branch = useBuild(id)?.info.branch;
      return (
        <div className="flex items-center gap-2">
          <FolderGit className="w-4 h-4" />
          {branch}
        </div>
      );
    },
  },

  Actions: { RunBuild },

  Page: {},

  Config: ConfigOrDeployments,

  DangerZone: ({ id }) => <DeleteResource type="Build" id={id} />,
};
