import { Page } from "@components/layouts";
import { useRead } from "@lib/hooks";
import { DataTable } from "@ui/data-table";

export const UsersPage = () => {
  const users = useRead("GetUsers", {}).data;
  return (
    <Page title="Users">
      <DataTable
        data={users ?? []}
        columns={[
          { header: "Username", accessorKey: "username" },
          { header: "Admin", accessorKey: "admin" },
          { header: "Enabled", accessorKey: "enabled" },
        ]}
      />
    </Page>
  );
};
