import { Section } from "@components/layouts";
import {
  bg_color_class_by_intention,
  deployment_state_intention,
  stroke_color_class_by_intention,
} from "@lib/color";
import { snake_case_to_upper_space_case } from "@lib/formatting";
import { useRead } from "@lib/hooks";
import { cn } from "@lib/utils";
import { DataTable, SortableHeader } from "@ui/data-table";
import { useStack } from ".";
import { Types } from "@monitor/client";
import { ReactNode } from "react";
import { Link } from "react-router-dom";
import { Button } from "@ui/button";
import { Layers2 } from "lucide-react";

export const StackServices = ({
  id,
  titleOther,
}: {
  id: string;
  titleOther: ReactNode;
}) => {
  const state = useStack(id)?.info.state ?? Types.StackState.Unknown;
  const services = useRead("ListStackServices", { stack: id }).data;
  // const [hidden, setHidden] = useLocalStorage(`services-hidden-${id}`, false);
  if (
    !services ||
    services.length === 0 ||
    [Types.StackState.Unknown, Types.StackState.Down].includes(state)
  ) {
    return null;
  }
  return (
    <Section
      // title="Services"
      // icon={<Layers2 className="w-4 h-4" />}
      // actions={
      //   <Button variant="outline" onClick={() => setHidden(!hidden)}>
      //     {hidden ? "show" : "hide"}
      //   </Button>
      // }
      titleOther={titleOther}
    >
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
                  to={`/stacks/${id}/${row.original.service}`}
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
            // size: 120,
          },
        ]}
      />
    </Section>
  );
};
