import { Page } from "@components/layouts";
import { ResourceComponents } from "@components/resources";
import { UpdatesTable } from "@components/updates/table";
import { useRead, useResourceParamType, useSetTitle } from "@lib/hooks";
import { Types } from "@monitor/client";
import { CaretSortIcon } from "@radix-ui/react-icons";
import { UsableResource } from "@types";
import { Button } from "@ui/button";
import {
  Command,
  CommandEmpty,
  CommandGroup,
  CommandInput,
  CommandItem,
  CommandList,
} from "@ui/command";
import { Popover, PopoverContent, PopoverTrigger } from "@ui/popover";
import { Bell, SearchX } from "lucide-react";
import { useState } from "react";
import { useParams } from "react-router-dom";

export const Updates = () => {
  const type = useResourceParamType()!;
  const id = useParams().id as string;
  if (type && id) {
    return <ResourceUpdates type={type} id={id} />;
  } else {
    return <AllUpdates />;
  }
};

const AllUpdates = () => {
  useSetTitle("Updates");
  const [operation, setOperation] = useState<Types.Operation | undefined>();
  const [page, setPage] = useState(0);
  const updates = useRead("ListUpdates", { query: { operation }, page }).data;
  return (
    <Page
      title="Updates"
      icon={<Bell className="w-8 h-8" />}
      actions={
        <OperationSelector selected={operation} onSelect={setOperation} />
      }
    >
      <div className="flex flex-col gap-4">
        <UpdatesTable updates={updates?.updates ?? []} showTarget />
        <div className="flex gap-4 justify-center items-center text-muted-foreground">
          <Button
            variant="outline"
            onClick={() => setPage(page - 1)}
            disabled={page === 0}
          >
            Prev Page
          </Button>
          Page: {page + 1}
          <Button
            variant="outline"
            onClick={() => updates?.next_page && setPage(updates.next_page)}
            disabled={!updates?.next_page}
          >
            Next Page
          </Button>
        </div>
      </div>
    </Page>
  );
};

const ResourceUpdates = ({
  type,
  id,
}: {
  type: UsableResource;
  id: string;
}) => {
  const name = useRead(`List${type}s`, {}).data?.find((r) => r.id === id)?.name;
  useSetTitle(name && `${name} | Updates`);
  const [operation, setOperation] = useState<Types.Operation | undefined>();
  const [page, setPage] = useState(0);
  const updates = useRead("ListUpdates", {
    query: {
      "target.type": type,
      "target.id": id,
      operation,
    },
    page,
  }).data;
  const Components = ResourceComponents[type];
  return (
    <Page
      title={<Components.Name id={id} />}
      titleRight={<h2 className="text-muted-foreground">Updates</h2>}
      icon={<Components.BigIcon id={id} />}
      actions={
        <OperationSelector
          selected={operation}
          onSelect={setOperation}
          options={OPERATIONS_BY_RESOURCE[type]}
        />
      }
    >
      <div className="flex flex-col gap-4">
        <UpdatesTable updates={updates?.updates ?? []} />
        <div className="flex gap-4 justify-center items-center text-muted-foreground">
          <Button
            variant="outline"
            onClick={() => setPage(page - 1)}
            disabled={page === 0}
          >
            Prev Page
          </Button>
          Page: {page + 1}
          <Button
            variant="outline"
            onClick={() => updates?.next_page && setPage(updates.next_page)}
            disabled={!updates?.next_page}
          >
            Next Page
          </Button>
        </div>
      </div>
    </Page>
  );
};

const OPERATIONS_BY_RESOURCE: { [key: string]: Types.Operation[] } = {
  Server: [
    Types.Operation.CreateServer,
    Types.Operation.UpdateServer,
    Types.Operation.RenameServer,
    Types.Operation.PruneImagesServer,
    Types.Operation.PruneContainersServer,
    Types.Operation.PruneNetworksServer,
    Types.Operation.CreateNetwork,
    Types.Operation.DeleteNetwork,
    Types.Operation.StopAllContainers,
  ],
  Deployment: [
    Types.Operation.CreateDeployment,
    Types.Operation.UpdateDeployment,
    Types.Operation.RenameDeployment,
    Types.Operation.DeployContainer,
    Types.Operation.StopContainer,
    Types.Operation.StartContainer,
    Types.Operation.RemoveContainer,
  ],
  Build: [
    Types.Operation.CreateBuild,
    Types.Operation.UpdateBuild,
    Types.Operation.RunBuild,
    Types.Operation.CancelBuild,
  ],
  Repo: [
    Types.Operation.CreateRepo,
    Types.Operation.UpdateRepo,
    Types.Operation.CloneRepo,
    Types.Operation.PullRepo,
  ],
  Procedure: [
    Types.Operation.CreateProcedure,
    Types.Operation.UpdateProcedure,
    Types.Operation.RunProcedure,
  ],
  Builder: [Types.Operation.CreateBuilder, Types.Operation.UpdateBuilder],
  Alerter: [Types.Operation.CreateAlerter, Types.Operation.UpdateAlerter],
  ServerTemplate: [
    Types.Operation.CreateServerTemplate,
    Types.Operation.UpdateServerTemplate,
    Types.Operation.LaunchServer,
  ],
};

const OperationSelector = ({
  selected,
  onSelect,
  options = Object.values(Types.Operation).filter(
    (o) => o !== Types.Operation.None
  ),
}: {
  selected: Types.Operation | undefined;
  onSelect: (operation: Types.Operation | undefined) => void;
  options?: Types.Operation[];
}) => {
  const [open, setOpen] = useState(false);
  const [input, setInput] = useState("");
  return (
    <Popover open={open} onOpenChange={setOpen}>
      <PopoverTrigger asChild>
        <div className="h-full w-[200px] cursor-pointer flex items-center justify-between whitespace-nowrap rounded-md border border-input bg-transparent px-3 py-2 text-sm shadow-sm ring-offset-background placeholder:text-muted-foreground focus:outline-none focus:ring-1 focus:ring-ring disabled:cursor-not-allowed disabled:opacity-50 [&>span]:line-clamp-1">
          {selected ?? "Select Operation"}
          <CaretSortIcon className="h-4 w-4 opacity-50" />
        </div>
      </PopoverTrigger>
      <PopoverContent align="end" className="w-[200px] max-h-[200px] p-0">
        <Command>
          <CommandInput
            placeholder="Search Operations"
            value={input}
            onValueChange={setInput}
            className="h-9"
          />
          <CommandList>
            <CommandEmpty className="flex justify-evenly items-center">
              No Operations Found
              <SearchX className="w-3 h-3" />
            </CommandEmpty>

            <CommandGroup>
              <CommandItem
                className="cursor-pointer"
                onSelect={() => {
                  onSelect(undefined);
                  setOpen(false);
                }}
              >
                <div>All</div>
              </CommandItem>

              {options.map((operation) => (
                <CommandItem
                  className="cursor-pointer"
                  onSelect={() => {
                    onSelect(operation);
                    setOpen(false);
                  }}
                >
                  {operation}
                </CommandItem>
              ))}
            </CommandGroup>
          </CommandList>
        </Command>
      </PopoverContent>
    </Popover>
  );
};
