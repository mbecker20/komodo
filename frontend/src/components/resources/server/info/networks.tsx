import { Section } from "@components/layouts";
import { DockerResourceLink, ShowHideButton } from "@components/util";
import { useRead } from "@lib/hooks";
import { Badge } from "@ui/badge";
import { DataTable, SortableHeader } from "@ui/data-table";
import { Network } from "lucide-react";
import { useCallback } from "react";

export const Networks = ({
  id,
  show,
  setShow,
}: {
  id: string;
  show: boolean;
  setShow: (show: boolean) => void;
}) => {
  const networks =
    useRead("ListDockerNetworks", { server: id }, { refetchInterval: 5000 })
      .data ?? [];
  const containers = useRead("ListDockerContainers", { server: id }).data ?? [];

  const no_containers = useCallback(
    (network_name: string) => {
      return containers.every(
        (container) => !container.networks?.includes(network_name)
      );
    },
    [containers]
  );

  return (
    <Section
      title="Networks"
      icon={<Network className="w-4 h-4" />}
      actions={<ShowHideButton show={show} setShow={setShow} />}
    >
      {show && (
        <DataTable
          tableKey="server-networks"
          data={networks}
          columns={[
            {
              accessorKey: "name",
              header: ({ column }) => (
                <SortableHeader column={column} title="Name" />
              ),
              cell: ({ row }) => (
                <div className="flex items-center gap-2">
                  <DockerResourceLink
                    type="network"
                    server_id={id}
                    name={row.original.name}
                    extra={
                      ["none", "host", "bridge"].includes(
                        row.original.name ?? ""
                      ) ? (
                        <Badge variant="outline">System</Badge>
                      ) : (
                        row.original.name &&
                        no_containers(row.original.name) && (
                          <Badge variant="destructive">Unused</Badge>
                        )
                      )
                    }
                  />
                </div>
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
              accessorKey: "attachable",
              header: ({ column }) => (
                <SortableHeader column={column} title="Attachable" />
              ),
            },
            {
              accessorKey: "ipam_driver",
              header: ({ column }) => (
                <SortableHeader column={column} title="IPAM Driver" />
              ),
            },
          ]}
        />
      )}
    </Section>
  );
};
