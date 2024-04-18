import { Types } from "@monitor/client";
import { DataTable } from "@ui/data-table";
import { AlertLevel } from ".";
import { AlertDetailsDialog } from "./details";
import { UsableResource } from "@types";
import { ResourceLink } from "@components/resources/common";

export const AlertsTable = ({ alerts }: { alerts: Types.Alert[] }) => {
  return (
    <DataTable
      tableKey="alerts"
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
            const type = row.original.target.type as UsableResource;
            return <ResourceLink type={type} id={row.original.target.id} />;
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
      ]}
    />
  );
};
