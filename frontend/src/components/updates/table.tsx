import { fmt_date_with_minutes, fmt_operation } from "@lib/formatting";
import { Types } from "komodo_client";
import { DataTable } from "@ui/data-table";
import { useState } from "react";
import { UpdateDetailsInner, UpdateUser } from "./details";
import { ResourceLink } from "@components/resources/common";
import { Settings } from "lucide-react";
import { StatusBadge } from "@components/util";

export const UpdatesTable = ({
  updates,
  showTarget,
}: {
  updates: Types.UpdateListItem[];
  showTarget?: boolean;
}) => {
  const [id, setId] = useState("");
  return (
    <>
      <DataTable
        tableKey="updates"
        data={updates}
        columns={[
          {
            header: "Operation",
            accessorKey: "operation",
            cell: ({ row }) => {
              const more =
                row.original.status === Types.UpdateStatus.InProgress
                  ? "in progress"
                  : row.original.status === Types.UpdateStatus.Queued
                  ? "queued"
                  : undefined;
              return (
                <div className="flex items-center gap-2">
                  {fmt_operation(row.original.operation)}{" "}
                  {more && (
                    <div className="text-sm text-muted-foreground">{more}</div>
                  )}
                </div>
              );
            },
          },
          showTarget && {
            header: "Target",
            cell: ({ row }) =>
              row.original.target.type === "System" ? (
                <div className="flex items-center gap-2">
                  <Settings className="w-4 h-4" />
                  System
                </div>
              ) : (
                <ResourceLink
                  type={row.original.target.type}
                  id={row.original.target.id}
                />
              ),
          },
          {
            header: "Result",
            cell: ({ row }) => {
              const { success, status } = row.original;
              return (
                <StatusBadge
                  intent={
                    status === Types.UpdateStatus.Complete
                      ? success
                        ? "Good"
                        : "Critical"
                      : "None"
                  }
                  text={
                    status === Types.UpdateStatus.Complete
                      ? success
                        ? "Success"
                        : "Failed"
                      : "Processing"
                  }
                />
              );
            },
          },
          {
            header: "Start Time",
            accessorFn: ({ start_ts }) =>
              fmt_date_with_minutes(new Date(start_ts)),
          },
          {
            header: "Operator",
            accessorKey: "operator",
            cell: ({ row }) => <UpdateUser user_id={row.original.operator} />,
          },
        ]}
        onRowClick={(row) => setId(row.id)}
      />
      <UpdateDetailsInner id={id} open={!!id} setOpen={() => setId("")} />
    </>
  );
};
