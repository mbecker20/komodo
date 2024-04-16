import { Section } from "@components/layouts";
import { ConfirmButton } from "@components/util";
import { useExecute, useRead } from "@lib/hooks";
import { RequiredResourceComponents } from "@types";
import { Card, CardDescription, CardHeader, CardTitle } from "@ui/card";
import { AlertTriangle, Loader2, Route } from "lucide-react";
import { Link } from "react-router-dom";
import { ProcedureConfig } from "./config";
import { ProcedureTable } from "./table";
import {
  CopyResource,
  DeleteResource,
  NewResource,
  ResourceLink,
} from "../common";

const useProcedure = (id?: string) =>
  useRead("ListProcedures", {}).data?.find((d) => d.id === id);

export const ProcedureComponents: RequiredResourceComponents = {
  Name: ({ id }) => <>{useProcedure(id)?.name}</>,
  Link: ({ id }) => <ResourceLink type="Procedure" id={id} />,
  Info: [({ id }) => <>{useProcedure(id)?.info.procedure_type}</>],
  Icon: () => <Route className="w-4" />,
  Status: () => <>Procedure</>,
  Page: {
    Config: ProcedureConfig,
    Danger: ({ id }) => (
      <Section
        title="Danger Zone"
        icon={<AlertTriangle className="w-4 h-4" />}
        actions={<CopyResource type="Procedure" id={id} />}
      >
        <DeleteResource type="Procedure" id={id} />
      </Section>
    ),
  },
  Actions: [
    ({ id }) => {
      const running = useRead("GetProcedureActionState", { procedure: id }).data
        ?.running;
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
  ],
  Table: ProcedureTable,
  New: () => <NewResource type="Procedure" />,
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
};
