import { Section } from "@components/layouts";
import { ShowHideButton } from "@components/util";
import { useRead } from "@lib/hooks";
import { Button } from "@ui/button";
import { DataTable, SortableHeader } from "@ui/data-table";
import { Database } from "lucide-react";
import { Link } from "react-router-dom";

export const Volumes = ({
  id,
  show,
  setShow,
}: {
  id: string;
  show: boolean;
  setShow: (show: boolean) => void;
}) => {
  const volumes = useRead("ListDockerVolumes", { server: id }).data ?? [];
  console.log(volumes)

  return (
    <Section
      title="Volumes"
      icon={<Database className="w-4 h-4" />}
      actions={<ShowHideButton show={show} setShow={setShow} />}
    >
      {show && (
        <DataTable
          tableKey="server-volumes"
          data={volumes}
          columns={[
            {
              accessorKey: "name",
              header: ({ column }) => (
                <SortableHeader column={column} title="Name" />
              ),
              cell: ({ row }) => (
                <Link
                  to={`/servers/${id}/volume/${encodeURIComponent(
                    row.original.name
                  )}`}
                  className="px-0"
                >
                  <Button variant="link" className="px-0">
                    {row.original.name}
                  </Button>
                </Link>
              ),
              size: 200,
            },
            {
              accessorKey: "driver",
              header: ({ column }) => (
                <SortableHeader column={column} title="Driver" />
              ),
            },
            {
              accessorKey: "scope",
              header: ({ column }) => (
                <SortableHeader column={column} title="Scope" />
              ),
            },
            {
              accessorKey: "ref_count",
              header: ({ column }) => (
                <SortableHeader column={column} title="Ref count" />
              ),
            },
          ]}
        />
      )}
    </Section>
  );
};
