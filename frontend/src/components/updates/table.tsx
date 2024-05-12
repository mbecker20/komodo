import { fmt_date_with_minutes } from "@lib/formatting";
import { Types } from "@monitor/client";
import { ColumnDef } from "@tanstack/react-table";
import { DataTable } from "@ui/data-table";
import { useState } from "react";
import { UpdateDetailsInner, UpdateUser } from "./details";

export const UpdatesTable = ({
  updates,
  showTarget,
}: {
  updates: Types.UpdateListItem[];
  showTarget?: boolean;
}) => {
  let data: ColumnDef<Types.UpdateListItem, string>[] = [
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
      accessorFn: ({ success }) => (success ? "Ok" : "Fail"),
    },
    {
      header: "Start Time",
      accessorFn: ({ start_ts }) => fmt_date_with_minutes(new Date(start_ts)),
    },
    {
      header: "Operator",
      accessorKey: "operator",
      cell: ({ row }) => <UpdateUser user_id={row.original.operator} />,
    },
  ];
  // attach the target column on front
  if (showTarget) {
    data = [
      {
        header: "Target",
        accessorKey: "target.id",
      },
      ...data,
    ];
  }
  const [id, setId] = useState("");
  return (
    <>
      <DataTable
        tableKey="updates"
        data={updates}
        columns={data}
        onRowClick={(row) => setId(row.id)}
      />
      <UpdateDetailsInner id={id} open={!!id} setOpen={() => setId("")} />
    </>
  );
};
