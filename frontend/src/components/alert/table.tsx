import { Types } from "komodo_client";
import { DataTable } from "@ui/data-table";
import { AlertLevel } from ".";
import { AlertDetailsDialog } from "./details";
import { UsableResource } from "@types";
import { ResourceLink } from "@components/resources/common";
import {
  alert_level_intention,
  text_color_class_by_intention,
} from "@lib/color";

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
            return (
              <div
                className={text_color_class_by_intention(
                  row.original.resolved
                    ? "Good"
                    : alert_level_intention(row.original.level)
                )}
              >
                {row.original.resolved ? "RESOLVED" : "OPEN"}
              </div>
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
