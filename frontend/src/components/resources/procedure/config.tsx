import { ConfigLayout } from "@components/config";
import { ResourceSelector } from "@components/config/util";
import { useRead, useWrite } from "@lib/hooks";
import { Types } from "@monitor/client";
import { UsableResource } from "@types";
import { DataTable } from "@ui/data-table";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@ui/select";
import { Switch } from "@ui/switch";
import { useState } from "react";

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
            : ""}
        </div>
        <DataTable
          data={config.data}
          columns={[
            {
              header: "Execution",
              accessorKey: "execution.type",
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
                const Component = EXEC_TYPES[type].Component;
                return (
                  <Component
                    params={params as any}
                    setParams={(params) =>
                      setConfig({
                        ...config,
                        data: config.data.map((item, i) =>
                          i === index ? { ...item, params } : item
                        ),
                      })
                    }
                  />
                );
              },
            },
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
              header: "Menu",
            },
          ]}
        />
      </div>
    </ConfigLayout>
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

const TypeSelector = ({
  type,
  selected,
  onSelect,
}: {
  type: UsableResource;
  selected: string;
  onSelect: (value: string) => void;
}) => (
  <div className="flex gap-2 items-center">
    {type}
    <ResourceSelector type={type} selected={selected} onSelect={onSelect} />
  </div>
);

const EXEC_TYPES: ExecutionConfigs = {
  None: {
    params: {},
    Component: () => <></>,
  },
  CloneRepo: {
    params: { repo: "" },
    Component: ({ params, setParams }) => (
      <TypeSelector
        type="Repo"
        selected={params.repo}
        onSelect={(repo) => setParams({ repo })}
      />
    ),
  },
  Deploy: {
    params: { deployment: "" },
    Component: ({ params, setParams }) => (
      <TypeSelector
        type="Deployment"
        selected={params.deployment}
        onSelect={(deployment) => setParams({ deployment })}
      />
    ),
  },
  PruneDockerContainers: {
    params: { server: "" },
    Component: ({ params, setParams }) => (
      <TypeSelector
        type="Server"
        selected={params.server}
        onSelect={(server) => setParams({ server })}
      />
    ),
  },
  PruneDockerImages: {
    params: { server: "" },
    Component: ({ params, setParams }) => (
      <TypeSelector
        type="Server"
        selected={params.server}
        onSelect={(server) => setParams({ server })}
      />
    ),
  },
  PruneDockerNetworks: {
    params: { server: "" },
    Component: ({ params, setParams }) => (
      <TypeSelector
        type="Server"
        selected={params.server}
        onSelect={(server) => setParams({ server })}
      />
    ),
  },
  PullRepo: {
    params: { repo: "" },
    Component: ({ params, setParams }) => (
      <TypeSelector
        type="Repo"
        selected={params.repo}
        onSelect={(repo) => setParams({ repo })}
      />
    ),
  },
  RemoveContainer: {
    params: { deployment: "" },
    Component: ({ params, setParams }) => (
      <TypeSelector
        type="Deployment"
        selected={params.deployment}
        onSelect={(deployment) => setParams({ deployment })}
      />
    ),
  },
  RunBuild: {
    params: { build: "" },
    Component: ({ params, setParams }) => (
      <TypeSelector
        type="Build"
        selected={params.build}
        onSelect={(build) => setParams({ build })}
      />
    ),
  },
  RunProcedure: {
    params: { procedure: "" },
    Component: ({ params, setParams }) => (
      <TypeSelector
        type="Procedure"
        selected={params.procedure}
        onSelect={(procedure) => setParams({ procedure })}
      />
    ),
  },
  StartContainer: {
    params: { deployment: "" },
    Component: ({ params, setParams }) => (
      <TypeSelector
        type="Deployment"
        selected={params.deployment}
        onSelect={(deployment) => setParams({ deployment })}
      />
    ),
  },
  StopAllContainers: {
    params: { server: "" },
    Component: ({ params, setParams }) => (
      <TypeSelector
        type="Server"
        selected={params.server}
        onSelect={(id) => setParams({ server: id })}
      />
    ),
  },
  StopContainer: {
    params: { deployment: "" },
    Component: ({ params, setParams }) => (
      <TypeSelector
        type="Deployment"
        selected={params.deployment}
        onSelect={(id) => setParams({ deployment: id })}
      />
    ),
  },
};
