import { Section } from "@components/layouts";
import {
  DockerResourceLink,
  ShowHideButton,
  StatusBadge,
} from "@components/util";
import { container_state_intention } from "@lib/color";
import { useRead } from "@lib/hooks";
import { DataTable, SortableHeader } from "@ui/data-table";
import { Box } from "lucide-react";

export const Containers = ({
  id,
  show,
  setShow,
}: {
  id: string;
  show: boolean;
  setShow: (show: boolean) => void;
}) => {
  const containers = useRead("ListDockerContainers", { server: id }).data ?? [];

  return (
    <Section
      title="Containers"
      icon={<Box className="w-4 h-4" />}
      actions={<ShowHideButton show={show} setShow={setShow} />}
    >
      {show && (
        <DataTable
          tableKey="server-containers"
          data={containers}
          columns={[
            {
              accessorKey: "name",
              header: ({ column }) => (
                <SortableHeader column={column} title="Name" />
              ),
              cell: ({ row }) => (
                <DockerResourceLink
                  type="container"
                  server_id={id}
                  name={row.original.name}
                />
              ),
              size: 200,
            },
            {
              accessorKey: "image",
              header: ({ column }) => (
                <SortableHeader column={column} title="Image" />
              ),
            },
            {
              accessorKey: "network_mode",
              header: ({ column }) => (
                <SortableHeader column={column} title="Network" />
              ),
            },
            {
              accessorKey: "state",
              header: ({ column }) => (
                <SortableHeader column={column} title="State" />
              ),
              cell: ({ row }) => {
                const state = row.original?.state;
                return (
                  <StatusBadge
                    text={state}
                    intent={container_state_intention(state)}
                  />
                );
              },
            },
          ]}
        />
      )}
    </Section>
  );
};
