import { TagsWithBadge } from "@components/tags";
import { ConfirmButton } from "@components/util";
import { useExecute, useRead } from "@lib/hooks";
import { fmt_date_with_minutes } from "@lib/utils";
import { RequiredResourceComponents } from "@types";
import { Card, CardDescription, CardHeader, CardTitle } from "@ui/card";
import { DataTable } from "@ui/data-table";
import { Link, Loader2, Route } from "lucide-react";

const useProcedure = (id?: string) =>
  useRead("ListProcedures", {}).data?.find((d) => d.id === id);

export const ProcedureComponents: RequiredResourceComponents = {
  Name: ({ id }) => <>{useProcedure(id)?.name}</>,
  Description: ({ id }) => <>{useProcedure(id)?.info.procedure_type}</>,
  Info: ({ id }) => <>{id}</>,
  Icon: () => <Route className="w-4" />,
  Page: {
    // Config: ({ id }) => <ProcedureConfig id={id} />,
  },
  Actions: ({ id }) => {
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
  Table: () => {
    const procedures = useRead("ListProcedures", {}).data;
    return (
      <DataTable
        data={procedures ?? []}
        columns={[
          {
            accessorKey: "id",
            header: "Name",
            cell: ({ row }) => {
              const id = row.original.id;
              return (
                <Link
                  to={`/procedures/${id}`}
                  className="flex items-center gap-2"
                >
                  <ProcedureComponents.Icon id={id} />
                  <ProcedureComponents.Name id={id} />
                </Link>
              );
            },
          },
          {
            header: "Tags",
            cell: ({ row }) => {
              return (
                <div className="flex gap-1">
                  <TagsWithBadge tag_ids={row.original.tags} />
                </div>
              );
            },
          },
          {
            header: "Created",
            accessorFn: ({ created_at }) =>
              fmt_date_with_minutes(new Date(created_at)),
          },
        ]}
      />
    );
  },
  New: () => <></>,
  Dashboard: () => {
    const procedure_count = useRead("ListProcedures", {}).data?.length;
    return (
      <Link to="/procedures/" className="w-full">
        <Card>
          <CardHeader className="justify-between">
            <div>
              <CardTitle>Procedures</CardTitle>
              <CardDescription>{procedure_count} Total</CardDescription>
            </div>
            <Route className="w-4 h-4" />
          </CardHeader>
        </Card>
      </Link>
    );
  },
};
