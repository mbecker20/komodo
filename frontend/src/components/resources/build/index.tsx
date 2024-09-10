import { Section } from "@components/layouts";
import { useInvalidate, useRead, useWrite } from "@lib/hooks";
import { RequiredResourceComponents } from "@types";
import { Factory, FolderGit, Hammer, Loader2, RefreshCcw } from "lucide-react";
import { BuildConfig } from "./config";
import { BuildTable } from "./table";
import { DeleteResource, NewResource, ResourceLink } from "../common";
import { DeploymentTable } from "../deployment/table";
import { RunBuild } from "./actions";
import {
  build_state_intention,
  stroke_color_class_by_intention,
} from "@lib/color";
import { cn } from "@lib/utils";
import { useState } from "react";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@ui/tabs";
import { ResourceComponents } from "..";
import { Types } from "@komodo/client";
import { DashboardPieChart } from "@pages/home/dashboard";
import { StatusBadge } from "@components/util";
import { HoverCard, HoverCardContent, HoverCardTrigger } from "@ui/hover-card";
import { Card } from "@ui/card";
import { Badge } from "@ui/badge";
import { useToast } from "@ui/use-toast";
import { Button } from "@ui/button";
import { useBuilder } from "../builder";

export const useBuild = (id?: string) =>
  useRead("ListBuilds", {}, { refetchInterval: 5000 }).data?.find(
    (d) => d.id === id
  );

export const useFullBuild = (id: string) =>
  useRead("GetBuild", { build: id }, { refetchInterval: 5000 }).data;

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
  use_links: (id) => useFullBuild(id)?.config?.links,

  Description: () => <>Build docker images.</>,

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
      return <StatusBadge text={state} intent={build_state_intention(state)} />;
    },
    Built: ({ id }) => {
      const info = useFullBuild(id)?.info;
      if (!info?.built_hash) {
        return null;
      }
      return (
        <HoverCard openDelay={200}>
          <HoverCardTrigger asChild>
            <Card className="px-3 py-2 hover:bg-accent/50 transition-colors cursor-pointer">
              <div className="text-muted-foreground text-sm text-nowrap overflow-hidden overflow-ellipsis">
                built: {info.built_hash}
              </div>
            </Card>
          </HoverCardTrigger>
          <HoverCardContent align="start">
            <div className="grid gap-2">
              <Badge
                variant="secondary"
                className="w-fit text-muted-foreground"
              >
                commit message
              </Badge>
              {info.built_message}
            </div>
          </HoverCardContent>
        </HoverCard>
      );
    },
    Latest: ({ id }) => {
      const info = useFullBuild(id)?.info;
      if (!info?.latest_hash || info.latest_hash === info?.built_hash) {
        return null;
      }
      return (
        <HoverCard openDelay={200}>
          <HoverCardTrigger asChild>
            <Card className="px-3 py-2 hover:bg-accent/50 transition-colors cursor-pointer">
              <div className="text-muted-foreground text-sm text-nowrap overflow-hidden overflow-ellipsis">
                latest: {info.latest_hash}
              </div>
            </Card>
          </HoverCardTrigger>
          <HoverCardContent align="start">
            <div className="grid gap-2">
              <Badge
                variant="secondary"
                className="w-fit text-muted-foreground"
              >
                commit message
              </Badge>
              {info.latest_message}
            </div>
          </HoverCardContent>
        </HoverCard>
      );
    },
    Refresh: ({ id }) => {
      const { toast } = useToast();
      const inv = useInvalidate();
      const { mutate, isPending } = useWrite("RefreshBuildCache", {
        onSuccess: () => {
          inv(["ListBuilds"], ["GetBuild", { build: id }]);
          toast({ title: "Refreshed build status cache" });
        },
      });
      return (
        <Button
          variant="outline"
          size="icon"
          onClick={() => {
            mutate({ build: id });
            toast({ title: "Triggered refresh of build status cache" });
          }}
        >
          {isPending ? (
            <Loader2 className="w-4 h-4 animate-spin" />
          ) : (
            <RefreshCcw className="w-4 h-4" />
          )}
        </Button>
      );
    },
  },

  Info: {
    Builder: ({ id }) => {
      const info = useBuild(id)?.info;
      const builder = useBuilder(info?.builder_id);
      return builder?.id ? (
        <ResourceLink type="Builder" id={builder?.id} />
      ) : (
        <div className="flex gap-2 items-center text-sm">
          <Factory className="w-4 h-4" />
          <div>Unknown Builder</div>
        </div>
      );
    },
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
