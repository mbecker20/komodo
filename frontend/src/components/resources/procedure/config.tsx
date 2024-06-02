import { ConfigItem } from "@components/config/util";
import { Section } from "@components/layouts";
import { useRead, useWrite } from "@lib/hooks";
import { Types } from "@monitor/client";
import { Card, CardHeader } from "@ui/card";
import { Input } from "@ui/input";
import { useState } from "react";
import { CopyGithubWebhook, ResourceSelector } from "../common";
import { ConfigLayout } from "@components/config";
import { Popover, PopoverContent, PopoverTrigger } from "@ui/popover";
import { Button } from "@ui/button";
import {
  ChevronsUpDown,
  Info,
  MinusCircle,
  PlusCircle,
  SearchX,
  Settings,
} from "lucide-react";
import {
  Command,
  CommandEmpty,
  CommandGroup,
  CommandInput,
  CommandItem,
  CommandList,
} from "@ui/command";
import { Switch } from "@ui/switch";
import { DataTable } from "@ui/data-table";
import { HoverCard, HoverCardContent, HoverCardTrigger } from "@ui/hover-card";

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
  const [branch, setBranch] = useState("main");
  const [config, setConfig] = useState<Partial<Types.ProcedureConfig>>({});
  const perms = useRead("GetPermissionLevel", {
    target: { type: "Procedure", id: procedure._id?.$oid! },
  }).data;
  const global_disabled =
    useRead("GetCoreInfo", {}).data?.ui_write_disabled ?? false;
  const { mutateAsync } = useWrite("UpdateProcedure");
  const stages = config.stages || procedure.config.stages || [];

  const disabled = global_disabled || perms !== Types.PermissionLevel.Write;

  const add_stage = () =>
    setConfig((config) => ({
      ...config,
      stages: [
        ...stages,
        {
          name: `Stage ${stages.length + 1}`,
          enabled: true,
          executions: [],
        },
      ],
    }));

  return (
    <div className="flex flex-col gap-8">
      <ConfigLayout
        titleOther={
          <div className="flex items-center gap-4 text-muted-foreground">
            <div className="flex items-center gap-2">
              <Settings className="w-4 h-4" />
              <h2 className="text-xl">Config</h2>
            </div>
            <HoverCard openDelay={200}>
              <HoverCardTrigger asChild>
                <Button variant="outline">
                  <Info className="w-4 h-4" />
                </Button>
              </HoverCardTrigger>
              <HoverCardContent align="start">
                <div>
                  The executions in a stage are all run in parallel. The stages
                  themselves are run sequentially.
                </div>
              </HoverCardContent>
            </HoverCard>
          </div>
        }
        disabled={disabled}
        config={config}
        onConfirm={async () => {
          await mutateAsync({ id: procedure._id!.$oid, config });
          // setConfig({});
        }}
        onReset={() => setConfig({})}
      >
        {stages.map((stage, index) => (
          <Stage
            stage={stage}
            setStage={(stage) =>
              setConfig((config) => ({
                ...config,
                stages: stages.map((s, i) => (index === i ? stage : s)),
              }))
            }
            removeStage={() =>
              setConfig((config) => ({
                ...config,
                stages: stages.filter((_, i) => index !== i),
              }))
            }
            disabled={disabled}
          />
        ))}
        <Button onClick={add_stage} className="w-fit self-end">
          Add Stage
        </Button>
      </ConfigLayout>
      <Section>
        <Card>
          <CardHeader className="p-4">
            <ConfigItem label="Github Webhook" className="items-start">
              <div className="flex flex-col gap-4">
                <div className="flex items-center gap-4">
                  <div className="flex items-center gap-2">
                    <div className="text-nowrap text-muted-foreground">
                      Listen on branch:
                    </div>
                    <Input
                      placeholder="Branch"
                      value={branch}
                      onChange={(e) => setBranch(e.target.value)}
                      className="w-[200px]"
                    />
                  </div>
                  <CopyGithubWebhook
                    path={`/procedure/${procedure._id?.$oid!}/${branch}`}
                  />
                </div>
                <div className="flex items-center justify-end gap-4 w-full">
                  <div className="text-muted-foreground">Enabled:</div>
                  <Switch
                    checked={
                      config.webhook_enabled ?? procedure.config.webhook_enabled
                    }
                    onCheckedChange={(webhook_enabled) =>
                      setConfig({ ...config, webhook_enabled })
                    }
                    disabled={disabled}
                  />
                </div>
              </div>
            </ConfigItem>
          </CardHeader>
        </Card>
      </Section>
    </div>
  );
};

const Stage = ({
  stage,
  setStage,
  removeStage,
  disabled,
}: {
  stage: Types.ProcedureStage;
  setStage: (stage: Types.ProcedureStage) => void;
  removeStage: () => void;
  disabled: boolean;
}) => {
  return (
    <Card className="p-4 flex flex-col gap-4">
      <div className="flex justify-between items-center">
        <Input
          value={stage.name}
          onChange={(e) => setStage({ ...stage, name: e.target.value })}
          className="w-[300px] text-md"
        />
        <div className="flex gap-4 items-center">
          <div>Enabled:</div>
          <Switch
            checked={stage.enabled}
            onCheckedChange={(enabled) => setStage({ ...stage, enabled })}
          />
          <Button variant="secondary" onClick={removeStage}>
            <MinusCircle className="w-4 h-4" />
          </Button>
        </div>
      </div>
      <DataTable
        tableKey="procedure-stage-executions"
        data={stage.executions}
        noResults={
          <Button
            onClick={() =>
              setStage({
                ...stage,
                executions: [default_enabled_execution()],
              })
            }
            disabled={disabled}
          >
            Add Execution
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
                    setStage({
                      ...stage,
                      executions: stage.executions.map((item, i) =>
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
                  setStage({
                    ...stage,
                    executions: stage.executions.map((item, i) =>
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
                    setStage({
                      ...stage,
                      executions: stage.executions.map((item, i) =>
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
            cell: ({ row: { index } }) => (
              <div className="flex items-center gap-2">
                <Button
                  variant="secondary"
                  onClick={() =>
                    setStage({
                      ...stage,
                      executions: [
                        ...stage.executions.slice(0, index + 1),
                        default_enabled_execution(),
                        ...stage.executions.slice(index + 1),
                      ],
                    })
                  }
                >
                  <PlusCircle className="w-4 h-4" />
                </Button>
                <Button
                  variant="secondary"
                  onClick={() =>
                    setStage({
                      ...stage,
                      executions: stage.executions.filter(
                        (_, i) => i !== index
                      ),
                    })
                  }
                >
                  <MinusCircle className="w-4 h-4" />
                </Button>
              </div>
            ),
          },
        ]}
      />
    </Card>
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
      console.log(params.deployment);
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
  PruneContainers: {
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
  PruneImages: {
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
  PruneNetworks: {
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
