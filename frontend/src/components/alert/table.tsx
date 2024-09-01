import { Types } from "@komodo/client";
import { DataTable } from "@ui/data-table";
import { AlertLevel } from ".";
import { AlertDetailsDialog } from "./details";
import { UsableResource } from "@types";
import { ResourceLink } from "@components/resources/common";
import { bg_color_class_by_intention } from "@lib/color";
import { Card, CardHeader } from "@ui/card";
import { cn } from "@lib/utils";

export const AlertsTable = ({
  alerts,
  showResolved,
}: {
  alerts: Types.Alert[];
  showResolved?: boolean;
}) => {
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
          header: "Resource",
          cell: ({ row }) => {
            const type = row.original.target.type as UsableResource;
            return <ResourceLink type={type} id={row.original.target.id} />;
          },
        },
        showResolved && {
          header: "Status",
          cell: ({ row }) => {
            const color = bg_color_class_by_intention(
              row.original.resolved ? "Good" : "Critical"
            );
            return (
              <Card className={cn("w-fit", color)}>
                <CardHeader className="py-0 px-2">
                  {row.original.resolved ? "Resolved" : "Open"}
                </CardHeader>
              </Card>
            );
          },
        },
        {
          header: "Level",
          cell: ({ row }) => <AlertLevel level={row.original.level} />,
        },
        {
          header: "Alert Type",
          accessorKey: "data.type",
        },
      ]}
    />
  );
};
