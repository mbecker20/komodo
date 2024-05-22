import { Page } from "@components/layouts";
import { ConfirmButton, TextUpdateMenu } from "@components/util";
import {
  useInvalidate,
  useRead,
  useSetTitle,
  useUser,
  useWrite,
} from "@lib/hooks";
import { Badge } from "@ui/badge";
import { Button } from "@ui/button";
import { DataTable, SortableHeader } from "@ui/data-table";
import {
  Dialog,
  DialogContent,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from "@ui/dialog";
import { Input } from "@ui/input";
import { useToast } from "@ui/use-toast";
import { Check, Loader2, PlusCircle, Trash, Variable } from "lucide-react";
import { useState } from "react";

export const Variables = () => {
  const user = useUser().data;
  const disabled = !user?.admin;
  useSetTitle("Variables");
  const [search, setSearch] = useState("");
  const { variables, secrets } = useRead("ListVariables", {}).data ?? {
    variables: [],
    secrets: [],
  };
  secrets.sort();
  const searchSplit = search?.toLowerCase().split(" ") || [];
  const filtered =
    variables?.filter((variable) => {
      if (searchSplit.length > 0) {
        const name = variable.name.toLowerCase();
        return searchSplit.every((search) => name.includes(search));
      } else return true;
    }) ?? [];
  const { toast } = useToast();
  const { mutate: updateValue } = useWrite("UpdateVariableValue");
  const inv = useInvalidate();
  const { mutate: updateDescription } = useWrite("UpdateVariableDescription", {
    onSuccess: () => {
      inv(["ListVariables"], ["GetVariable"]);
      toast({ title: "Updated variable description" });
    },
  });
  return (
    <Page
      title="Variables"
      icon={<Variable className="w-8 h-8" />}
      actions={<CreateVariable />}
    >
      <div className="flex flex-col gap-4">
        <Input
          placeholder="search..."
          value={search}
          onChange={(e) => setSearch(e.target.value)}
          className="w-[200px] lg:w-[300px]"
        />

        {/** VARIABLES */}
        <DataTable
          tableKey="variables"
          data={filtered}
          columns={[
            {
              accessorKey: "name",
              header: ({ column }) => (
                <SortableHeader column={column} title="Name" />
              ),
            },
            {
              accessorKey: "value",
              header: ({ column }) => (
                <SortableHeader column={column} title="Value" />
              ),
              cell: ({ row }) => {
                return (
                  <TextUpdateMenu
                    title={`${row.original.name} - Value`}
                    placeholder="Set value"
                    value={row.original.value}
                    onUpdate={(value) => {
                      if (row.original.value === value) {
                        return;
                      }
                      updateValue({ name: row.original.name, value });
                    }}
                    triggerClassName="w-full"
                    disabled={disabled}
                    fullWidth
                  />
                );
              },
            },
            {
              accessorKey: "description",
              header: "Description",
              cell: ({ row }) => {
                return (
                  <TextUpdateMenu
                    title={`${row.original.name} - Description`}
                    placeholder="Set description"
                    value={row.original.description}
                    onUpdate={(description) => {
                      if (row.original.description === description) {
                        return;
                      }
                      updateDescription({
                        name: row.original.name,
                        description,
                      });
                    }}
                    triggerClassName="w-full"
                    disabled={disabled}
                    fullWidth
                  />
                );
              },
            },
            {
              header: "Delete",
              cell: ({ row }) => <DeleteVariable name={row.original.name} />,
            },
          ]}
        />

        {/** SECRETS */}
        {secrets.length && (
          <div className="flex items-center gap-2 text-muted-foreground">
            <div>Core Secrets:</div>
            {secrets.map((secret) => (
              <Badge variant="secondary">{secret}</Badge>
            ))}
          </div>
        )}
      </div>
    </Page>
  );
};

const CreateVariable = () => {
  const { toast } = useToast();
  const [open, setOpen] = useState(false);
  const [name, setName] = useState("");
  const invalidate = useInvalidate();
  const { mutate, isPending } = useWrite("CreateVariable", {
    onSuccess: () => {
      invalidate(["ListVariables"], ["GetVariable"]);
      toast({ title: "Variable Created" });
      setOpen(false);
    },
    onError: (e) => {
      console.log("create variable error:" + e);
      toast({
        title: "Failed to create variable",
        description: "See console for details",
        variant: "destructive",
      });
      setOpen(false);
    },
  });
  const submit = () => mutate({ name });
  return (
    <Dialog open={open} onOpenChange={setOpen}>
      <DialogTrigger asChild>
        <Button className="items-center gap-2">
          New Variable <PlusCircle className="w-4 h-4" />
        </Button>
      </DialogTrigger>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Create Variable</DialogTitle>
        </DialogHeader>
        <div className="py-8 flex flex-col gap-4">
          <div className="flex items-center justify-between">
            Name
            <Input
              className="w-72"
              value={name}
              onChange={(e) =>
                setName(e.target.value.toUpperCase().replaceAll(" ", "_"))
              }
            />
          </div>
        </div>
        <DialogFooter className="flex justify-end">
          <Button className="gap-4" onClick={submit} disabled={isPending}>
            Submit
            {isPending ? (
              <Loader2 className="w-4 animate-spin" />
            ) : (
              <Check className="w-4" />
            )}
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
};

const DeleteVariable = ({ name }: { name: string }) => {
  const invalidate = useInvalidate();
  const { toast } = useToast();
  const { mutate, isPending } = useWrite("DeleteVariable", {
    onSuccess: () => {
      invalidate(["ListVariables"], ["GetVariable"]);
      toast({ title: "Variable Deleted" });
    },
    onError: (e) => {
      console.log("delete variable error:" + e);
      toast({
        title: "Failed to delete variable",
        description: "See console for details",
        variant: "destructive",
      });
    },
  });
  return (
    <ConfirmButton
      title="Delete"
      icon={<Trash className="w-4 h-4" />}
      onClick={() => mutate({ name })}
      loading={isPending}
    />
  );
};
