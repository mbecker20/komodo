import { Section } from "@components/layouts";
import {
  deployment_state_intention,
  stroke_color_class_by_intention,
} from "@lib/color";
import { useRead } from "@lib/hooks";
import { cn } from "@lib/utils";
import { DataTable, SortableHeader } from "@ui/data-table";
import { useStack } from ".";
import { Types } from "@monitor/client";
import { ReactNode } from "react";
import { Link } from "react-router-dom";
import { Button } from "@ui/button";
import { Layers2 } from "lucide-react";
import { StatusBadge } from "@components/util";

export const StackServices = ({
  id,
  titleOther,
}: {
  id: string;
  titleOther: ReactNode;
}) => {
  const state = useStack(id)?.info.state ?? Types.StackState.Unknown;
  const services = useRead(
    "ListStackServices",
    { stack: id },
    { refetchInterval: 5000 }
  ).data;
  if (
    !services ||
    services.length === 0 ||
    [Types.StackState.Unknown, Types.StackState.Down].includes(state)
  ) {
    return null;
  }
  return (
    <Section titleOther={titleOther}>
      <div className="lg:min-h-[300px]">
        <DataTable
          tableKey="StackServices"
          data={services}
          columns={[
            {
              accessorKey: "service",
              header: ({ column }) => (
                <SortableHeader column={column} title="Service" />
              ),
              cell: ({ row }) => {
                const state = row.original.container?.state;
                const color = stroke_color_class_by_intention(
                  deployment_state_intention(state)
                );
                return (
                  <Link
                    to={`/stacks/${id}/service/${row.original.service}`}
                    onClick={(e) => e.stopPropagation()}
                  >
                    <Button
                      variant="link"
                      className="flex gap-2 items-center p-0"
                    >
                      <Layers2 className={cn("w-4 h-4", color)} />
                      {row.original.service}
                    </Button>
                  </Link>
                );
              },
              // size: 200,
            },
            {
              accessorKey: "container.image",
              header: ({ column }) => (
                <SortableHeader column={column} title="Image" />
              ),
              cell: ({ row }) => <>{row.original.container?.image}</>,
              // size: 200,
            },
            {
              accessorKey: "container.status",
              header: ({ column }) => (
                <SortableHeader column={column} title="Service" />
              ),
              cell: ({ row }) => {
                const state = row.original.container?.state;
                return (
                  <StatusBadge
                    text={state}
                    intent={deployment_state_intention(state)}
                  />
                );
              },
              // size: 120,
            },
          ]}
        />
      </div>
    </Section>
  );
};
