import { ConfigLayout } from "@components/config";
import { ConfirmButton } from "@components/util";
import { useRead, useWrite } from "@lib/hooks";
import { Types } from "@monitor/client";
import { DotsHorizontalIcon } from "@radix-ui/react-icons";
import { Button } from "@ui/button";
import {
  Command,
  CommandEmpty,
  CommandGroup,
  CommandInput,
  CommandItem,
} from "@ui/command";
import { DataTable } from "@ui/data-table";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuLabel,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from "@ui/dropdown-menu";
import { Popover, PopoverContent, PopoverTrigger } from "@ui/popover";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@ui/select";
import { Switch } from "@ui/switch";
import { CommandList } from "cmdk";
import {
  ArrowDown,
  ArrowUp,
  ChevronsUpDown,
  Plus,
  SearchX,
  Trash2,
} from "lucide-react";
import { useState } from "react";
import { ResourceSelector } from "../common";

export const ProcedureConfig = ({ id }: { id: string }) => {
  const procedure = useRead("GetProcedure", { procedure: id }).data;
  if (!procedure) return null;
  return <ProcedureConfigInner procedure={procedure} />;
};

const ProcedureConfigInner = ({
  procedure,
}: {
  procedure: Types.Procedure;
}) => {
  const [config, setConfig] = useState<Types.ProcedureConfig>(procedure.config);
  const { mutate } = useWrite("UpdateProcedure");
  return (
    <ConfigLayout
      config={config as any}
      onConfirm={() => mutate({ id: procedure._id!.$oid, config })}
      onReset={() => setConfig(procedure.config)}
      selector={
        <div className="flex gap-2 items-center text-sm">
          Procedure Type:
          <Select
            value={config.type}
            onValueChange={(type) =>
              setConfig({ type: type as any, data: config.data })
            }
          >
            <SelectTrigger className="w-32 capitalize">
              <SelectValue />
            </SelectTrigger>
            <SelectContent className="w-32">
              {["Sequence", "Parallel"].map((key) => (
                <SelectItem value={key} key={key} className="capitalize">
                  {key}
                </SelectItem>
              ))}
            </SelectContent>
          </Select>
        </div>
      }
    >
      <div className="grid gap-4">
        <div className="text-muted-foreground">
          {config.type === "Parallel"
            ? "Type Parallel: All of these executions will be started at the same time"
            : "Type Sequence: These executions will be started only after the previous one finishes"}
        </div>
        <DataTable
          data={config.data}
          noResults={
            <Button
              onClick={() =>
                setConfig({ ...config, data: [default_enabled_execution()] })
              }
            >
              Create Stage
            </Button>
          }
          columns={[
            {
              header: "Enabled",
              cell: ({
                row: {
                  original: { enabled },
                  index,
                },
              }) => {
                return (
                  <Switch
                    checked={enabled}
                    onClick={() =>
                      setConfig({
                        ...config,
                        data: config.data.map((item, i) =>
                          i === index ? { ...item, enabled: !enabled } : item
                        ),
                      })
                    }
                  />
                );
              },
            },
            {
              header: "Execution",
              cell: ({ row: { original, index } }) => (
                <ExecutionTypeSelector
                  type={original.execution.type}
                  onSelect={(type) =>
                    setConfig({
                      ...config,
                      data: config.data.map((item, i) =>
                        i === index
                          ? ({
                              ...item,
                              execution: {
                                type,
                                params:
                                  TARGET_COMPONENTS[
                                    type as Types.Execution["type"]
                                  ].params,
                              },
                            } as Types.EnabledExecution)
                          : item
                      ),
                    })
                  }
                />
              ),
            },
            {
              header: "Target",
              cell: ({
                row: {
                  original: {
                    execution: { type, params },
                  },
                  index,
                },
              }) => {
                const Component = TARGET_COMPONENTS[type].Component;
                return (
                  <Component
                    params={params as any}
                    setParams={(params) =>
                      setConfig({
                        ...config,
                        data: config.data.map((item, i) =>
                          i === index
                            ? {
                                ...item,
                                execution: { type, params: params as any },
                              }
                            : item
                        ),
                      })
                    }
                  />
                );
              },
            },
            {
              header: "Modify",
              cell: ({ row }) => {
                return (
                  <DropdownMenu>
                    <DropdownMenuTrigger asChild>
                      <Button variant="ghost" className="h-8 w-8 p-0">
                        <span className="sr-only">Open menu</span>
                        <DotsHorizontalIcon className="h-4 w-4" />
                      </Button>
                    </DropdownMenuTrigger>
                    <DropdownMenuContent align="end">
                      <DropdownMenuLabel>Actions</DropdownMenuLabel>
                      <DropdownMenuSeparator />
                      {row.index ? (
                        <DropdownMenuItem
                          className="flex gap-4 justify-between cursor-pointer"
                          onClick={() =>
                            setConfig({
                              ...config,
                              data: config.data.map((item, i) => {
                                // Make sure its not the first row
                                if (i === row.index && row.index !== 0) {
                                  return config.data[row.index - 1];
                                } else if (i === row.index - 1) {
                                  // Reverse the entry, moving this row "Up"
                                  return config.data[row.index];
                                } else {
                                  return item;
                                }
                              }),
                            })
                          }
                        >
                          Move Up <ArrowUp className="w-4 h-4" />
                        </DropdownMenuItem>
                      ) : undefined}
                      {row.index < config.data.length - 1 && (
                        <DropdownMenuItem
                          className="flex gap-4 justify-between cursor-pointer"
                          onClick={() =>
                            setConfig({
                              ...config,
                              data: config.data.map((item, i) => {
                                // The index also cannot be the last index, which cannot be moved down
                                if (
                                  i === row.index &&
                                  row.index !== config.data.length - 1
                                ) {
                                  return config.data[row.index + 1];
                                } else if (i === row.index + 1) {
                                  // Move the row "Down"
                                  return config.data[row.index];
                                } else {
                                  return item;
                                }
                              }),
                            })
                          }
                        >
                          Move Down <ArrowDown className="w-4 h-4" />
                        </DropdownMenuItem>
                      )}
                      <DropdownMenuSeparator />
                      <DropdownMenuItem
                        className="flex gap-4 justify-between cursor-pointer"
                        onClick={() =>
                          setConfig({
                            ...config,
                            data: [
                              ...config.data.slice(0, row.index),
                              default_enabled_execution(),
                              ...config.data.slice(row.index),
                            ],
                          })
                        }
                      >
                        Insert Above{" "}
                        <div className="flex">
                          <ArrowUp className="w-4 h-4" />
                          <Plus className="w-4 h-4" />
                        </div>
                      </DropdownMenuItem>
                      <DropdownMenuItem
                        className="flex gap-4 justify-between cursor-pointer"
                        onClick={() =>
                          setConfig({
                            ...config,
                            data: [
                              ...config.data.slice(0, row.index + 1),
                              default_enabled_execution(),
                              ...config.data.slice(row.index + 1),
                            ],
                          })
                        }
                      >
                        Insert Below{" "}
                        <div className="flex">
                          <ArrowDown className="w-4 h-4" />
                          <Plus className="w-4 h-4" />
                        </div>
                      </DropdownMenuItem>
                    </DropdownMenuContent>
                  </DropdownMenu>
                );
              },
            },
            {
              header: "Delete",
              cell: ({ row: { index } }) => (
                <ConfirmButton
                  title="Delete Row"
                  icon={<Trash2 className="w-4 h-4" />}
                  onClick={() =>
                    setConfig({
                      ...config,
                      data: config.data.filter((_, i) => i !== index),
                    })
                  }
                />
              ),
            },
          ]}
        />
      </div>
    </ConfigLayout>
  );
};

const default_enabled_execution: () => Types.EnabledExecution = () => ({
  enabled: true,
  execution: {
    type: "None",
    params: {},
  },
});

const ExecutionTypeSelector = ({
  type,
  onSelect,
}: {
  type: Types.Execution["type"];
  onSelect: (type: Types.Execution["type"]) => void;
}) => {
  const [open, setOpen] = useState(false);
  const [search, setSearch] = useState("");
  return (
    <Popover open={open} onOpenChange={setOpen}>
      <PopoverTrigger asChild>
        <Button variant="secondary" className="flex gap-2">
          {type}
          <ChevronsUpDown className="w-3 h-3" />
        </Button>
      </PopoverTrigger>
      <PopoverContent className="w-[200px] max-h-[200px] p-0" sideOffset={12}>
        <Command>
          <CommandInput
            placeholder="Search Executions"
            className="h-9"
            value={search}
            onValueChange={setSearch}
          />
          <CommandList>
            <CommandEmpty className="flex justify-evenly items-center">
              Empty.
              <SearchX className="w-3 h-3" />
            </CommandEmpty>
            <CommandGroup className="overflow-auto">
              {[
                "RunProcedure",
                "RunBuild",
                "Deploy",
                "StartContainer",
                "StopContainer",
                "StopAllContainers",
                "RemoveContainer",
                "CloneRepo",
                "PullRepo",
              ].map((type) => (
                <CommandItem
                  key={type}
                  onSelect={() => onSelect(type as Types.Execution["type"])}
                  className="flex items-center justify-between"
                >
                  <div className="p-1">{type}</div>
                </CommandItem>
              ))}
            </CommandGroup>
          </CommandList>
        </Command>
      </PopoverContent>
    </Popover>
  );
};

type ExecutionType = Types.Execution["type"];

type ExecutionConfigComponent<
  T extends ExecutionType,
  P = Extract<Types.Execution, { type: T }>["params"]
> = React.FC<{
  params: P;
  setParams: React.Dispatch<React.SetStateAction<P>>;
}>;

type ExecutionConfigParams<T extends ExecutionType> = Extract<
  Types.Execution,
  { type: T }
>["params"];

type ExecutionConfigs = {
  [ExType in ExecutionType]: {
    Component: ExecutionConfigComponent<ExType>;
    params: ExecutionConfigParams<ExType>;
  };
};

const TARGET_COMPONENTS: ExecutionConfigs = {
  None: {
    params: {},
    Component: () => <></>,
  },
  CloneRepo: {
    params: { repo: "" },
    Component: ({ params, setParams }) => (
      <ResourceSelector
        type="Repo"
        selected={params.repo}
        onSelect={(repo) => setParams({ repo })}
      />
    ),
  },
  Deploy: {
    params: { deployment: "" },
    Component: ({ params, setParams }) => (
      <ResourceSelector
        type="Deployment"
        selected={params.deployment}
        onSelect={(deployment) => setParams({ deployment })}
      />
    ),
  },
  PruneDockerContainers: {
    params: { server: "" },
    Component: ({ params, setParams }) => (
      <ResourceSelector
        type="Server"
        selected={params.server}
        onSelect={(server) => setParams({ server })}
      />
    ),
  },
  PruneDockerImages: {
    params: { server: "" },
    Component: ({ params, setParams }) => (
      <ResourceSelector
        type="Server"
        selected={params.server}
        onSelect={(server) => setParams({ server })}
      />
    ),
  },
  PruneDockerNetworks: {
    params: { server: "" },
    Component: ({ params, setParams }) => (
      <ResourceSelector
        type="Server"
        selected={params.server}
        onSelect={(server) => setParams({ server })}
      />
    ),
  },
  PullRepo: {
    params: { repo: "" },
    Component: ({ params, setParams }) => (
      <ResourceSelector
        type="Repo"
        selected={params.repo}
        onSelect={(repo) => setParams({ repo })}
      />
    ),
  },
  RemoveContainer: {
    params: { deployment: "" },
    Component: ({ params, setParams }) => (
      <ResourceSelector
        type="Deployment"
        selected={params.deployment}
        onSelect={(deployment) => setParams({ deployment })}
      />
    ),
  },
  RunBuild: {
    params: { build: "" },
    Component: ({ params, setParams }) => (
      <ResourceSelector
        type="Build"
        selected={params.build}
        onSelect={(build) => setParams({ build })}
      />
    ),
  },
  RunProcedure: {
    params: { procedure: "" },
    Component: ({ params, setParams }) => (
      <ResourceSelector
        type="Procedure"
        selected={params.procedure}
        onSelect={(procedure) => setParams({ procedure })}
      />
    ),
  },
  StartContainer: {
    params: { deployment: "" },
    Component: ({ params, setParams }) => (
      <ResourceSelector
        type="Deployment"
        selected={params.deployment}
        onSelect={(deployment) => setParams({ deployment })}
      />
    ),
  },
  StopAllContainers: {
    params: { server: "" },
    Component: ({ params, setParams }) => (
      <ResourceSelector
        type="Server"
        selected={params.server}
        onSelect={(id) => setParams({ server: id })}
      />
    ),
  },
  StopContainer: {
    params: { deployment: "" },
    Component: ({ params, setParams }) => (
      <ResourceSelector
        type="Deployment"
        selected={params.deployment}
        onSelect={(id) => setParams({ deployment: id })}
      />
    ),
  },
};
