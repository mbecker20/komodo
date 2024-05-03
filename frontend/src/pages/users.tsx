import { Page, Section } from "@components/layouts";
import { NewServiceUser, NewUserGroup } from "@components/users/new";
import { UserTable } from "@components/users/table";
import { useRead, useSetTitle } from "@lib/hooks";
import { DataTable } from "@ui/data-table";
import { Input } from "@ui/input";
import { UserCircle2, Users } from "lucide-react";
import { useState } from "react";
import { useNavigate } from "react-router-dom";

export const UsersPage = () => {
  useSetTitle("Users");
  const nav = useNavigate();
  const groups = useRead("ListUserGroups", {}).data;
  const users = useRead("ListUsers", {}).data;
  const [search, setSearch] = useState("");
  const searchSplit = search.split(" ");
  return (
    <Page
      actions={
        <Input
          placeholder="Search"
          value={search}
          onChange={(e) => setSearch(e.target.value)}
          className="w-[250px]"
        />
      }
    >
      {/* User Groups */}
      <Section
        title="User Groups"
        icon={<Users className="w-4 h-4" />}
        actions={<NewUserGroup />}
      >
        <DataTable
          tableKey="user-groups"
          data={
            groups?.filter((group) =>
              searchSplit.every((term) => group.name.includes(term))
            ) ?? []
          }
          columns={[
            { header: "Name", accessorKey: "name" },
            {
              header: "Members",
              accessorFn: (group) => group.users.length,
            },
          ]}
          onRowClick={(group) => nav(`/user-groups/${group._id!.$oid}`)}
        />
      </Section>

      {/* Users */}
      <Section
        title="Users"
        icon={<UserCircle2 className="w-4 h-4" />}
        actions={<NewServiceUser />}
      >
        <UserTable
          users={
            users?.filter((user) =>
              searchSplit.every((term) => user.username.includes(term))
            ) ?? []
          }
        />
      </Section>
    </Page>
  );
};
