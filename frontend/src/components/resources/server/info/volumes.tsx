import { Section } from "@components/layouts";
import { DockerResourceLink, ShowHideButton } from "@components/util";
import { useRead } from "@lib/hooks";
import { DataTable, SortableHeader } from "@ui/data-table";
import { Database } from "lucide-react";

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
                <DockerResourceLink
                  type="volume"
                  server_id={id}
                  name={row.original.name}
                />
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
          ]}
        />
      )}
    </Section>
  );
};
