import { fmt_date_with_minutes } from "@lib/utils";
import { Types } from "@monitor/client";
import { DataTable } from "@ui/data-table";

export const UpdatesTable = ({
  updates,
	showTarget,
}: {
  updates: Types.UpdateListItem[];
	showTarget?: boolean;
}) => {
  return (
    <DataTable
      data={updates}
      columns={[
        // showTarget ? {
        // 	header: "Target",
        // 	accessorKey: "target.id",
        // } : undefined,
        {
          header: "Operation",
          accessorKey: "operation",
        },
        {
          header: "Status",
          accessorKey: "status",
        },
        {
          header: "Success",
          accessorFn: ({ success }) => success ? "Ok" : "Fail",
        },
        {
          header: "Start Time",
          accessorFn: ({ start_ts }) =>
            fmt_date_with_minutes(new Date(start_ts)),
        },
        {
          header: "Operator",
          accessorKey: "username",
        },
      ]}
    />
  );
};
