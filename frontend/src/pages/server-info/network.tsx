import { Section } from "@components/layouts";
import { ResourceLink } from "@components/resources/common";
import { useServer } from "@components/resources/server";
import { useRead, useSetTitle } from "@lib/hooks";
import { has_minimum_permissions } from "@lib/utils";
import { Types } from "@monitor/client";
import { Button } from "@ui/button";
import { DataTable, SortableHeader } from "@ui/data-table";
import { ChevronLeft, Loader2, Network } from "lucide-react";
import { useNavigate, useParams } from "react-router-dom";

export const NetworkPage = () => {
  const { type, id, network } = useParams() as {
    type: string;
    id: string;
    network: string;
  };
  if (type !== "servers") {
    return <div>This resource type does not have any networks.</div>;
  }
  return <NetworkPageInner id={id} network={network} />;
};

const NetworkPageInner = ({
  id,
  network: _network,
}: {
  id: string;
  network: string;
}) => {
  const server = useServer(id);
  useSetTitle(`${server?.name} | network | ${_network}`);
  const nav = useNavigate();
  const perms = useRead("GetPermissionLevel", {
    target: { type: "Server", id },
  }).data;
  const { data, isPending, isError } = useRead("ListDockerNetworks", {
    server: id,
  });
  // const isSystemNetwork = ["none", "host", "bridge"].includes(_network);
  // const containers =
  //   useRead("ListDockerContainers", {
  //     server: id,
  //   }).data?.filter((container) =>
  //     isSystemNetwork
  //       ? container.network_mode === _network
  //       : container.networks && container.networks.includes(_network)
  //   ) ?? [];
  const network = data?.find((network) => network.Name === _network);

  if (isPending) {
    return (
      <div className="flex justify-center w-full py-4">
        <Loader2 className="w-8 h-8 animate-spin" />
      </div>
    );
  }
  if (isError) {
    return (
      <div className="flex w-full py-4">
        Failed to get network list for server.
      </div>
    );
  }
  if (!network) {
    return (
      <div className="flex w-full py-4">
        No network found with given name: {_network}
      </div>
    );
  }

  const disabled = !has_minimum_permissions(perms, Types.PermissionLevel.Write);

  console.log(network)
  const containers = Object.values(network.Containers ?? {});

  return (
    <div className="flex flex-col gap-16">
      {/* HEADER */}
      <div className="flex flex-col gap-4">
        {/* BACK */}
        <div className="flex items-center justify-between mb-4">
          <Button
            className="gap-2"
            variant="secondary"
            onClick={() => nav("/servers/" + id)}
          >
            <ChevronLeft className="w-4" /> Back
          </Button>

          {/* <Button className="gap-2" variant="destructive">
            <Trash className="w-4" /> Delete
          </Button> */}
        </div>

        {/* HEADER */}
        <div className="flex flex-col gap-4">
          <div className="flex items-center gap-4">
            <div className="mt-1">
              <Network className="w-8 h-8" />
            </div>
            <h1 className="text-3xl">{network.Name}</h1>
          </div>
        </div>

        {/* INFO */}
        <div className="flex flex-wrap gap-4 items-center text-muted-foreground">
          <div>Network</div>
          |
          <ResourceLink type="Server" id={id} />
        </div>
      </div>

      <Section title="Containers">
        <DataTable
          tableKey="network-containers"
          data={containers}
          columns={[
            {
              accessorKey: "name",
              header: ({ column }) => (
                <SortableHeader column={column} title="Name" />
              ),
            },
            {
              accessorKey: "IPv4Address",
              header: ({ column }) => (
                <SortableHeader column={column} title="IPv4" />
              ),
            },
            {
              accessorKey: "IPv6Address",
              header: ({ column }) => (
                <SortableHeader column={column} title="IPv6" />
              ),
            },
            {
              accessorKey: "MacAddress",
              header: ({ column }) => (
                <SortableHeader column={column} title="Mac" />
              ),
            },
          ]}
        />
      </Section>
    </div>
  );
};
