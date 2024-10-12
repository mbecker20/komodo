import { text_color_class_by_intention } from "@lib/color";
import { Types } from "@komodo/client";
import { DataTable } from "@ui/data-table";
import { useNavigate } from "react-router-dom";
import { ColumnDef } from "@tanstack/react-table";
import { MinusCircle } from "lucide-react";
import { ConfirmButton } from "@components/util";
import { useUser } from "@lib/hooks";

export const UserTable = ({
  users,
  onUserRemove,
  onUserDelete,
  userDeleteDisabled,
  onSelfClick,
}: {
  users: Types.User[];
  onUserRemove?: (user_id: string) => void;
  onUserDelete?: (user_id: string) => void;
  userDeleteDisabled?: (user_id: string) => boolean;
  onSelfClick?: () => void;
}) => {
  const user = useUser().data;
  const nav = useNavigate();
  const columns: ColumnDef<Types.User, "User" | "Admin" | "Super Admin">[] = [
    { header: "Username", accessorKey: "username" },
    { header: "Type", accessorKey: "config.type" },
    {
      header: "Level",
      accessorFn: (user) =>
        user.admin ? (user.super_admin ? "Super Admin" : "Admin") : "User",
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
  if (onUserDelete) {
    columns.push({
      header: "Delete",
      cell: ({ row }) => (
        <ConfirmButton
          title="Delete"
          variant="destructive"
          icon={<MinusCircle className="w-4 h-4" />}
          onClick={(e) => {
            e.stopPropagation();
            onUserDelete(row.original._id?.$oid!);
          }}
          disabled={
            row.original._id?.$oid
              ? userDeleteDisabled?.(row.original._id.$oid) ?? true
              : true
          }
        />
      ),
    });
  }
  return (
    <DataTable
      tableKey="users"
      data={users}
      columns={columns}
      onRowClick={(row) =>
        row._id?.$oid === user?._id?.$oid
          ? onSelfClick?.()
          : nav(`/users/${row._id!.$oid}`)
      }
    />
  );
};
