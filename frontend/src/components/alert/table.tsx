import { ResourceComponents } from "@components/resources";
import { fmt_date_with_minutes } from "@lib/formatting";
import { Types } from "@monitor/client";
import { DataTable } from "@ui/data-table";
import { AlertLevel } from ".";
import { AlertDetailsDialog } from "./details";
import { UsableResource } from "@types";

export const AlertsTable = ({ alerts }: { alerts: Types.Alert[] }) => {
  return (
    <DataTable
      data={alerts ?? []}
      columns={[
        {
          header: "Details",
          cell: ({ row }) =>
            row.original._id?.$oid && (
              <AlertDetailsDialog id={row.original._id?.$oid} />
            ),
        },
        {
          header: "Target",
          cell: ({ row }) => {
            const Components =
              ResourceComponents[row.original.target.type as UsableResource];
            return Components ? (
              <Components.Link id={row.original.target.id} />
            ) : (
              "Unknown"
            );
          },
        },
        {
          header: "Level",
          cell: ({ row }) => <AlertLevel level={row.original.level} />,
        },
        {
          header: "Alert Type",
          accessorKey: "variant",
        },
        {
          header: "Opened",
          accessorFn: ({ ts }) => fmt_date_with_minutes(new Date(ts)),
        },
      ]}
    />
  );
};
