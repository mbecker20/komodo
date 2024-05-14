import { ConfirmButton } from "@components/util";
import { useExecute, useRead } from "@lib/hooks";
import { RequiredResourceComponents } from "@types";
import { Card, CardDescription, CardHeader, CardTitle } from "@ui/card";
import { Loader2, Route } from "lucide-react";
import { Link } from "react-router-dom";
import { ProcedureConfig } from "./config";
import { ProcedureTable } from "./table";
import { DeleteResource, NewResource } from "../common";
import { bg_color_class_by_intention, procedure_state_intention } from "@lib/color";
import { cn } from "@lib/utils";

const useProcedure = (id?: string) =>
  useRead("ListProcedures", {}).data?.find((d) => d.id === id);

export const ProcedureComponents: RequiredResourceComponents = {
  Dashboard: () => {
    const procedure_count = useRead("ListProcedures", {}).data?.length;
    return (
      <Link to="/procedures/" className="w-full">
        <Card className="hover:bg-accent/50 transition-colors cursor-pointer">
          <CardHeader>
            <div className="flex justify-between">
              <div>
                <CardTitle>Procedures</CardTitle>
                <CardDescription>{procedure_count} Total</CardDescription>
              </div>
              <Route className="w-4 h-4" />
            </div>
          </CardHeader>
        </Card>
      </Link>
    );
  },

  New: () => <NewResource type="Procedure" />,

  Table: ProcedureTable,

  Name: ({ id }) => <>{useProcedure(id)?.name}</>,
  name: (id) => useProcedure(id)?.name,

  Icon: () => <Route className="w-4" />,
  BigIcon: () => <Route className="w-8" />,

  Status: {
    State: ({ id }) => {
      let state = useProcedure(id)?.info.state;
      const color = bg_color_class_by_intention(procedure_state_intention(state));
      return (
        <Card className={cn("w-fit", color)}>
          <CardHeader className="py-0 px-2">{state}</CardHeader>
        </Card>
      );
    },
  },

  Info: {
    Type: ({ id }) => <div>{useProcedure(id)?.info.procedure_type}</div>,
  },

  Actions: {
    RunProcedure: ({ id }) => {
      const running = useRead(
        "GetProcedureActionState",
        { procedure: id },
        { refetchInterval: 5000 }
      ).data?.running;
      const { mutate, isPending } = useExecute("RunProcedure");
      return (
        <ConfirmButton
          title={running ? "Running" : "Run"}
          icon={
            running ? (
              <Loader2 className="w-4 h-4 animate-spin" />
            ) : (
              <Route className="h-4 w-4" />
            )
          }
          onClick={() => mutate({ procedure: id })}
          disabled={running || isPending}
        />
      );
    },
  },

  Page: {},

  Config: ProcedureConfig,

  DangerZone: ({ id }) => <DeleteResource type="Procedure" id={id} />,
};
