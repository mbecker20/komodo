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
  const perms = useRead("GetPermissionLevel", {
    target: { type: "Procedure", id: procedure._id?.$oid! },
  }).data;
  const [config, setConfig] = useState<Partial<Types.ProcedureConfig>>({});
  const { mutate } = useWrite("UpdateProcedure");
  const executions = config.executions || procedure.config.executions || [];

  const disabled = perms !== Types.PermissionLevel.Write;

  return (
    <ConfigLayout
      disabled={disabled}
      config={config as any}
      onConfirm={() => mutate({ id: procedure._id!.$oid, config })}
      onReset={() => setConfig(procedure.config)}
      selector={
        <div className="flex gap-2 items-center text-sm">
          Procedure Type:
          <Select
            value={config.procedure_type || procedure.config.procedure_type}
            onValueChange={(type) =>
              setConfig({ ...config, procedure_type: type as any })
            }
            disabled={disabled}
          >
            <SelectTrigger className="w-32 capitalize" disabled={disabled}>
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
          {config.procedure_type === Types.ProcedureType.Parallel
            ? "Type Parallel: All of these executions will be started at the same time"
            : "Type Sequence: These executions will be started only after the previous one finishes"}
        </div>
        <DataTable
          tableKey="procedure-stages"
          data={executions}
          noResults={
            <Button
              onClick={() =>
                setConfig({
                  ...config,
                  executions: [default_enabled_execution()],
                })
              }
              disabled={disabled}
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
                        executions: executions.map((item, i) =>
                          i === index ? { ...item, enabled: !enabled } : item
                        ),
                      })
                    }
                    disabled={disabled}
                  />
                );
              },
            },
            {
              header: "Execution",
              cell: ({ row: { original, index } }) => (
                <ExecutionTypeSelector
                  disabled={disabled}
                  type={original.execution.type}
                  onSelect={(type) =>
                    setConfig({
                      ...config,
                      executions: executions.map((item, i) =>
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
                    disabled={disabled}
                    params={params as any}
                    setParams={(params: any) =>
                      setConfig({
                        ...config,
                        executions: executions.map((item, i) =>
                          i === index
                            ? {
                                ...item,
                                execution: { type, params },
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
                    <DropdownMenuTrigger asChild disabled={disabled}>
                      <Button
                        variant="ghost"
                        className="h-8 w-8 p-0"
                        disabled={disabled}
                      >
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
                              executions: executions.map((item, i) => {
                                // Make sure its not the first row
                                if (i === row.index && row.index !== 0) {
                                  return executions[row.index - 1];
                                } else if (i === row.index - 1) {
                                  // Reverse the entry, moving this row "Up"
                                  return executions[row.index];
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
                      {row.index < executions.length - 1 && (
                        <DropdownMenuItem
                          className="flex gap-4 justify-between cursor-pointer"
                          onClick={() =>
                            setConfig({
                              ...config,
                              executions: executions.map((item, i) => {
                                // The index also cannot be the last index, which cannot be moved down
                                if (
                                  i === row.index &&
                                  row.index !== executions.length - 1
                                ) {
                                  return executions[row.index + 1];
                                } else if (i === row.index + 1) {
                                  // Move the row "Down"
                                  return executions[row.index];
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
                            executions: [
                              ...executions.slice(0, row.index),
                              default_enabled_execution(),
                              ...executions.slice(row.index),
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
                            executions: [
                              ...executions.slice(0, row.index + 1),
                              default_enabled_execution(),
                              ...executions.slice(row.index + 1),
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
                      executions: executions.filter((_, i) => i !== index),
                    })
                  }
                  disabled={disabled}
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
  disabled,
}: {
  type: Types.Execution["type"];
  onSelect: (type: Types.Execution["type"]) => void;
  disabled: boolean;
}) => {
  const [open, setOpen] = useState(false);
  const [search, setSearch] = useState("");
  return (
    <Popover open={open} onOpenChange={setOpen}>
      <PopoverTrigger asChild>
        <Button variant="secondary" className="flex gap-2" disabled={disabled}>
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
  disabled: boolean;
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
    Component: ({ params, setParams, disabled }) => (
      <ResourceSelector
        type="Repo"
        selected={params.repo}
        onSelect={(repo) => setParams({ repo })}
        disabled={disabled}
      />
    ),
  },
  Deploy: {
    params: { deployment: "" },
    Component: ({ params, setParams, disabled }) => {
      console.log(params.deployment)
      return (
        <ResourceSelector
          type="Deployment"
          selected={params.deployment}
          onSelect={(deployment) => setParams({ deployment })}
          disabled={disabled}
        />
      );
    },
  },
  PruneDockerContainers: {
    params: { server: "" },
    Component: ({ params, setParams, disabled }) => (
      <ResourceSelector
        type="Server"
        selected={params.server}
        onSelect={(server) => setParams({ server })}
        disabled={disabled}
      />
    ),
  },
  PruneDockerImages: {
    params: { server: "" },
    Component: ({ params, setParams, disabled }) => (
      <ResourceSelector
        type="Server"
        selected={params.server}
        onSelect={(server) => setParams({ server })}
        disabled={disabled}
      />
    ),
  },
  PruneDockerNetworks: {
    params: { server: "" },
    Component: ({ params, setParams, disabled }) => (
      <ResourceSelector
        type="Server"
        selected={params.server}
        onSelect={(server) => setParams({ server })}
        disabled={disabled}
      />
    ),
  },
  PullRepo: {
    params: { repo: "" },
    Component: ({ params, setParams, disabled }) => (
      <ResourceSelector
        type="Repo"
        selected={params.repo}
        onSelect={(repo) => setParams({ repo })}
        disabled={disabled}
      />
    ),
  },
  RemoveContainer: {
    params: { deployment: "" },
    Component: ({ params, setParams, disabled }) => (
      <ResourceSelector
        type="Deployment"
        selected={params.deployment}
        onSelect={(deployment) => setParams({ deployment })}
        disabled={disabled}
      />
    ),
  },
  RunBuild: {
    params: { build: "" },
    Component: ({ params, setParams, disabled }) => (
      <ResourceSelector
        type="Build"
        selected={params.build}
        onSelect={(build) => setParams({ build })}
        disabled={disabled}
      />
    ),
  },
  RunProcedure: {
    params: { procedure: "" },
    Component: ({ params, setParams, disabled }) => (
      <ResourceSelector
        type="Procedure"
        selected={params.procedure}
        onSelect={(procedure) => setParams({ procedure })}
        disabled={disabled}
      />
    ),
  },
  StartContainer: {
    params: { deployment: "" },
    Component: ({ params, setParams, disabled }) => (
      <ResourceSelector
        type="Deployment"
        selected={params.deployment}
        onSelect={(deployment) => setParams({ deployment })}
        disabled={disabled}
      />
    ),
  },
  StopAllContainers: {
    params: { server: "" },
    Component: ({ params, setParams, disabled }) => (
      <ResourceSelector
        type="Server"
        selected={params.server}
        onSelect={(id) => setParams({ server: id })}
        disabled={disabled}
      />
    ),
  },
  StopContainer: {
    params: { deployment: "" },
    Component: ({ params, setParams, disabled }) => (
      <ResourceSelector
        type="Deployment"
        selected={params.deployment}
        onSelect={(id) => setParams({ deployment: id })}
        disabled={disabled}
      />
    ),
  },
};
