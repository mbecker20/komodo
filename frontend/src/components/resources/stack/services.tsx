import { Section } from "@components/layouts";
import {
  bg_color_class_by_intention,
  deployment_state_intention,
} from "@lib/color";
import { snake_case_to_upper_space_case } from "@lib/formatting";
import { useRead } from "@lib/hooks";
import { cn } from "@lib/utils";
import { DataTable, SortableHeader } from "@ui/data-table";
import { Layers2 } from "lucide-react";
import { useStack } from ".";
import { Types } from "@monitor/client";

export const Services = ({ id }: { id: string }) => {
  const state = useStack(id)?.info.state ?? Types.StackState.Unknown;
  const services = useRead("ListStackServices", { stack: id }).data;
  if (
    !services ||
    services.length === 0 ||
    [Types.StackState.Unknown, Types.StackState.Down].includes(state)
  ) {
    return null;
  }
  return (
    <Section title="Services" icon={<Layers2 className="w-4 h-4" />}>
      <DataTable
        tableKey="StackServices"
        data={services}
        columns={[
          {
            accessorKey: "service",
            header: ({ column }) => (
              <SortableHeader column={column} title="Service" />
            ),
            cell: ({ row }) => <>{row.original.service}</>,
            size: 200,
          },
          {
            accessorKey: "container.status",
            header: ({ column }) => (
              <SortableHeader column={column} title="Service" />
            ),
            cell: ({ row }) => {
              const state = row.original.container?.state;
              const color = bg_color_class_by_intention(
                deployment_state_intention(state)
              );
              return (
                <p
                  className={cn(
                    "p-1 w-fit text-[10px] text-white rounded-md",
                    color
                  )}
                >
                  {snake_case_to_upper_space_case(
                    state ?? "Unknown"
                  ).toUpperCase()}
                </p>
              );
            },
            size: 120,
          },
        ]}
      />
    </Section>
  );
};
