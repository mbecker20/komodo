import {
  ActionWithDialog,
  ResourcePageHeader,
  StatusBadge,
} from "@components/util";
import { useExecute, useRead } from "@lib/hooks";
import { RequiredResourceComponents } from "@types";
import { Route } from "lucide-react";
import { ProcedureConfig } from "./config";
import { ProcedureTable } from "./table";
import { DeleteResource, NewResource } from "../common";
import {
  procedure_state_intention,
  stroke_color_class_by_intention,
} from "@lib/color";
import { cn } from "@lib/utils";
import { Types } from "komodo_client";
import { DashboardPieChart } from "@pages/home/dashboard";
import { RenameResource } from "@components/config/util";
import { GroupActions } from "@components/group-actions";

const useProcedure = (id?: string) =>
  useRead("ListProcedures", {}).data?.find((d) => d.id === id);

const ProcedureIcon = ({ id, size }: { id?: string; size: number }) => {
  const state = useProcedure(id)?.info.state;
  const color = stroke_color_class_by_intention(
    procedure_state_intention(state)
  );
  return <Route className={cn(`w-${size} h-${size}`, state && color)} />;
};

export const ProcedureComponents: RequiredResourceComponents = {
  list_item: (id) => useProcedure(id),
  resource_links: () => undefined,

  Description: () => <>Compose Komodo actions together.</>,

  Dashboard: () => {
    const summary = useRead("GetProceduresSummary", {}).data;
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

  New: () => <NewResource type="Procedure" />,

  GroupActions: () => <GroupActions type="Procedure" actions={["RunProcedure"]} />,

  Table: ({ resources }) => (
    <ProcedureTable procedures={resources as Types.ProcedureListItem[]} />
  ),

  Icon: ({ id }) => <ProcedureIcon id={id} size={4} />,
  BigIcon: ({ id }) => <ProcedureIcon id={id} size={8} />,

  State: ({ id }) => {
    let state = useProcedure(id)?.info.state;
    return (
      <StatusBadge text={state} intent={procedure_state_intention(state)} />
    );
  },

  Status: {},

  Info: {
    Stages: ({ id }) => <div>Stages: {useProcedure(id)?.info.stages}</div>,
  },

  Actions: {
    RunProcedure: ({ id }) => {
      const running = useRead(
        "GetProcedureActionState",
        { procedure: id },
        { refetchInterval: 5000 }
      ).data?.running;
      const { mutate, isPending } = useExecute("RunProcedure");
      const procedure = useProcedure(id);
      if (!procedure) return null;
      return (
        <ActionWithDialog
          name={procedure.name}
          title={running ? "Running" : "Run Procedure"}
          icon={<Route className="h-4 w-4" />}
          onClick={() => mutate({ procedure: id })}
          disabled={running || isPending}
          loading={running}
        />
      );
    },
  },

  Page: {},

  Config: ProcedureConfig,

  DangerZone: ({ id }) => (
    <>
      <RenameResource type="Procedure" id={id} />
      <DeleteResource type="Procedure" id={id} />
    </>
  ),

  ResourcePageHeader: ({ id }) => {
    const procedure = useProcedure(id);

    return (
      <ResourcePageHeader
        intent={procedure_state_intention(procedure?.info.state)}
        icon={<ProcedureIcon id={id} size={8} />}
        name={procedure?.name}
        state={procedure?.info.state}
        status={procedure?.info.stages.toString() + "Stages"}
      />
    );
  },
};
