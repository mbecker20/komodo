import { text_color_class_by_intention } from "@lib/color";
import { Types } from "@komodo/client";
import { DataTable } from "@ui/data-table";
import { useNavigate } from "react-router-dom";
import { ColumnDef } from "@tanstack/react-table";
import { MinusCircle } from "lucide-react";
import { ConfirmButton } from "@components/util";

export const UserTable = ({
  users,
  onUserRemove,
}: {
  users: Types.User[];
  onUserRemove?: (user_id: string) => void;
}) => {
  const nav = useNavigate();
  const columns: ColumnDef<Types.User, "User" | "Admin">[] = [
    { header: "Username", accessorKey: "username" },
    { header: "Type", accessorKey: "config.type" },
    {
      header: "Level",
      accessorFn: (user) => (user.admin ? "Admin" : "User"),
    },
    {
      header: "Enabled",
      cell: ({ row }) => {
        const enabledClass = row.original.enabled
          ? text_color_class_by_intention("Good")
          : text_color_class_by_intention("Critical");
        return (
          <div className={enabledClass}>
            {row.original.enabled ? "Enabled" : "Disabled"}
          </div>
        );
      },
    },
  ];
  if (onUserRemove) {
    columns.push({
      header: "Remove",
      cell: ({ row }) => (
        <ConfirmButton
          title="Remove"
          variant="destructive"
          icon={<MinusCircle className="w-4 h-4" />}
          onClick={(e) => {
            e.stopPropagation();
            onUserRemove(row.original._id?.$oid!);
          }}
        />
      ),
    });
  }
  return (
    <DataTable
      tableKey="users"
      data={users}
      columns={columns}
      onRowClick={(user) => nav(`/users/${user._id!.$oid}`)}
    />
  );
};
