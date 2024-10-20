import {
  ActionWithDialog,
  ResourcePageHeader,
  StatusBadge,
} from "@components/util";
import { useExecute, useRead } from "@lib/hooks";
import { RequiredResourceComponents } from "@types";
import { Clapperboard } from "lucide-react";
import { ActionConfig } from "./config";
import { ActionTable } from "./table";
import { DeleteResource, NewResource } from "../common";
import {
  action_state_intention,
  stroke_color_class_by_intention,
} from "@lib/color";
import { cn } from "@lib/utils";
import { Types } from "komodo_client";
import { DashboardPieChart } from "@pages/home/dashboard";
import { ActionInfo } from "./info";

const useAction = (id?: string) =>
  useRead("ListActions", {}).data?.find((d) => d.id === id);

const ActionIcon = ({ id, size }: { id?: string; size: number }) => {
  const state = useAction(id)?.info.state;
  const color = stroke_color_class_by_intention(action_state_intention(state));
  return <Clapperboard className={cn(`w-${size} h-${size}`, state && color)} />;
};

const ConfigInfo = ({ id }: { id: string }) => {
  return (
    <div className="flex flex-col gap-2">
      <ActionConfig id={id} />
      <ActionInfo id={id} />
    </div>
  );
};

export const ActionComponents: RequiredResourceComponents = {
  list_item: (id) => useAction(id),
  resource_links: () => undefined,

  Description: () => <>Custom scripts using the Komodo client.</>,

  Dashboard: () => {
    const summary = useRead("GetActionsSummary", {}).data;
    return (
      <DashboardPieChart
        data={[
          { title: "Ok", intention: "Good", value: summary?.ok ?? 0 },
          {
            title: "Running",
            intention: "Warning",
            value: summary?.running ?? 0,
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

  New: () => <NewResource type="Action" />,

  Table: ({ resources }) => (
    <ActionTable actions={resources as Types.ActionListItem[]} />
  ),

  Icon: ({ id }) => <ActionIcon id={id} size={4} />,
  BigIcon: ({ id }) => <ActionIcon id={id} size={8} />,

  State: ({ id }) => {
    let state = useAction(id)?.info.state;
    return <StatusBadge text={state} intent={action_state_intention(state)} />;
  },

  Status: {},

  Info: {},

  Actions: {
    RunAction: ({ id }) => {
      const running = useRead(
        "GetActionActionState",
        { action: id },
        { refetchInterval: 5000 }
      ).data?.running;
      const { mutate, isPending } = useExecute("RunAction");
      const action = useAction(id);
      if (!action) return null;
      return (
        <ActionWithDialog
          name={action.name}
          title={running ? "Running" : "Run Action"}
          icon={<Clapperboard className="h-4 w-4" />}
          onClick={() => mutate({ action: id })}
          disabled={running || isPending}
          loading={running}
        />
      );
    },
  },

  Page: {},

  Config: ConfigInfo,

  DangerZone: ({ id }) => <DeleteResource type="Action" id={id} />,

  ResourcePageHeader: ({ id }) => {
    const action = useAction(id);

    return (
      <ResourcePageHeader
        intent={action_state_intention(action?.info.state)}
        icon={<ActionIcon id={id} size={8} />}
        name={action?.name}
        state={action?.info.state}
        status={undefined}
      />
    );
  },
};
