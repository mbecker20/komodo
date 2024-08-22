import { Section } from "@components/layouts";
import { ResourceLink } from "@components/resources/common";
import { useServer } from "@components/resources/server";
import { DockerLabelsSection, DockerOptions } from "@components/util";
import { useRead, useSetTitle } from "@lib/hooks";
import { Button } from "@ui/button";
import { DataTable, SortableHeader } from "@ui/data-table";
import {
  Box,
  ChevronLeft,
  Info,
  Loader2,
  Network,
  Waypoints,
} from "lucide-react";
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
  return <NetworkPageInner id={id} network={decodeURIComponent(network)} />;
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
  // const perms = useRead("GetPermissionLevel", {
  //   target: { type: "Server", id },
  // }).data;
  const {
    data: network,
    isPending,
    isError,
  } = useRead("InspectDockerNetwork", {
    server: id,
    network: _network,
  });

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
        Failed to inspect network.
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

  // const disabled = !has_minimum_permissions(perms, Types.PermissionLevel.Write);

  const containers = Object.values(network.Containers ?? {});

  const ipam_driver = network.IPAM?.Driver;
  const ipam_config =
    network.IPAM?.Config.map((config) => ({
      ...config,
      Driver: ipam_driver,
    })) ?? [];

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

        {/* TITLE */}
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
          <ResourceLink type="Server" id={id} />|
          <div className="flex gap-2">
            <span>IPV6:</span>
            <span>{network.EnableIPv6 ? "Enabled" : "Disabled"}</span>
          </div>
          {network.Id ? (
            <>
              |
              <div className="flex gap-2">
                Id:
                <div className="max-w-[150px] overflow-hidden text-ellipsis">
                  {network.Id}
                </div>
              </div>
            </>
          ) : null}
        </div>
      </div>

      {/* TOP LEVEL NETWORK INFO */}
      <Section title="Details" icon={<Info className="w-4 h-4" />}>
        <DataTable
          tableKey="network-info"
          data={[network]}
          columns={[
            {
              accessorKey: "Driver",
              header: "Driver",
            },
            {
              accessorKey: "Scope",
              header: "Scope",
            },
            {
              accessorKey: "Attachable",
              header: "Attachable",
            },
            {
              accessorKey: "Internal",
              header: "Internal",
            },
          ]}
        />
        <DockerOptions options={network.Options} />
      </Section>

      {ipam_config.length > 0 && (
        <Section title="IPAM" icon={<Waypoints className="w-4 h-4" />}>
          <DataTable
            tableKey="network-ipam"
            data={ipam_config}
            columns={[
              {
                accessorKey: "Driver",
                header: "Driver",
              },
              {
                accessorKey: "Subnet",
                header: "Subnet",
              },
              {
                accessorKey: "Gateway",
                header: "Gateway",
              },
              {
                accessorKey: "IPRange",
                header: "IPRange",
              },
            ]}
          />
          <DockerOptions options={network.IPAM?.Options} />
        </Section>
      )}

      {containers.length > 0 && (
        <Section title="Containers" icon={<Box className="w-4 h-4" />}>
          <DataTable
            tableKey="network-containers"
            data={containers}
            columns={[
              {
                accessorKey: "Name",
                header: ({ column }) => (
                  <SortableHeader column={column} title="Name" />
                ),
              },
              {
                accessorKey: "IPv4Address",
                header: ({ column }) => (
                  <SortableHeader column={column} title="IPv4" />
                ),
                cell: ({ row }) => row.original.IPv4Address || "None",
              },
              {
                accessorKey: "IPv6Address",
                header: ({ column }) => (
                  <SortableHeader column={column} title="IPv6" />
                ),
                cell: ({ row }) => row.original.IPv6Address || "None",
              },
              {
                accessorKey: "MacAddress",
                header: ({ column }) => (
                  <SortableHeader column={column} title="Mac" />
                ),
                cell: ({ row }) => row.original.MacAddress || "None",
              },
            ]}
          />
        </Section>
      )}

      <DockerLabelsSection labels={network.Labels} />
    </div>
  );
};