import { NewResource, Section } from "@components/layouts";
import {
  ActionWithDialog,
  ConfirmButton,
  CopyResource,
  ResourceLink,
} from "@components/util";
import { useExecute, useRead, useWrite } from "@lib/hooks";
import { Types } from "@monitor/client";
import { RequiredResourceComponents } from "@types";
import { Card, CardDescription, CardHeader, CardTitle } from "@ui/card";
import { Input } from "@ui/input";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@ui/select";
import { AlertTriangle, Loader2, Route, Trash } from "lucide-react";
import { useState } from "react";
import { Link, useNavigate } from "react-router-dom";
import { ProcedureConfig } from "./config";
import { ProcedureTable } from "./table";

const useProcedure = (id?: string) =>
  useRead("ListProcedures", {}).data?.find((d) => d.id === id);

export const ProcedureComponents: RequiredResourceComponents = {
  Name: ({ id }) => <>{useProcedure(id)?.name}</>,
  Description: ({ id }) => <>{useProcedure(id)?.info.procedure_type}</>,
  Link: ({ id }) => <ResourceLink type="Procedure" id={id} />,
  Info: [({ id }) => <>{useProcedure(id)?.info.procedure_type}</>],
  Icon: () => <Route className="w-4" />,
  Status: () => <>Procedure</>,
  Page: {
    Config: ProcedureConfig,
    Danger: ({ id }) => (
      <Section
        title="Danger Zone"
        icon={<AlertTriangle className="w-4 h-4" />}
        actions={<CopyResource type="Procedure" id={id} />}
      >
        <DeleteProcedure id={id} />
      </Section>
    ),
  },
  Actions: [
    ({ id }) => {
      const running = useRead("GetProcedureActionState", { procedure: id }).data
        ?.running;
      const { mutate, isPending } = useExecute("RunProcedure");
      return (
        <ConfirmButton
          title={running ? "Running" : "Run"}
          icon={
            running ? (
              <Loader2 className="w-4 h-4 animate-spin" />
            ) : (
              <Route className="h-4 w-4" />
            )
          }
          onClick={() => mutate({ procedure: id })}
          disabled={running || isPending}
        />
      );
    },
  ],
  Table: ProcedureTable,
  New: () => {
    const { mutateAsync } = useWrite("CreateProcedure");
    const [name, setName] = useState("");
    const [type, setType] = useState<Types.ProcedureConfig["type"]>("Sequence");
    return (
      <NewResource
        entityType="Procedure"
        onSuccess={() => mutateAsync({ name, config: { type, data: [] } })}
        enabled={!!name}
      >
        <div className="grid md:grid-cols-2 items-center">
          Procedure Name
          <Input
            placeholder="procedure-name"
            value={name}
            onChange={(e) => setName(e.target.value)}
          />
        </div>
        <div className="flex justify-between items-center">
          Procedure Type
          <Select value={type} onValueChange={setType as any}>
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
      </NewResource>
    );
  },
  Dashboard: () => {
    const procedure_count = useRead("ListProcedures", {}).data?.length;
    return (
      <Link to="/procedures/" className="w-full">
        <Card>
          <CardHeader className="justify-between">
            <div>
              <CardTitle>Procedures</CardTitle>
              <CardDescription>{procedure_count} Total</CardDescription>
            </div>
            <Route className="w-4 h-4" />
          </CardHeader>
        </Card>
      </Link>
    );
  },
};

const DeleteProcedure = ({ id }: { id: string }) => {
  const nav = useNavigate();
  const procedure = useRead("GetProcedure", { procedure: id }).data;
  const { mutateAsync, isPending } = useWrite("DeleteProcedure");

  if (!procedure) return null;

  return (
    <div className="flex items-center justify-between">
      <div className="w-full">Delete Procedure</div>
      <ActionWithDialog
        name={procedure.name}
        title="Delete"
        icon={<Trash className="h-4 w-4" />}
        onClick={async () => {
          await mutateAsync({ id });
          nav("/");
        }}
        disabled={isPending}
        loading={isPending}
      />
    </div>
  );
};
