import {
  ActionButton,
  ActionWithDialog,
  ConfirmButton,
  TextUpdateMenu,
} from "@components/util";
import { useInvalidate, useRead, useWrite } from "@lib/hooks";
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
import {
  Dialog,
  DialogContent,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from "@ui/dialog";
import { Popover, PopoverContent, PopoverTrigger } from "@ui/popover";
import { Check, ChevronsUpDown, Copy, SearchX, Trash } from "lucide-react";
import { useState } from "react";
import { Link, useNavigate } from "react-router-dom";
import { ResourceComponents } from ".";
import { Input } from "@ui/input";
import { useToast } from "@ui/use-toast";
import { NewLayout } from "@components/layouts";
import { Types } from "@monitor/client";
import { ConfigItem, DoubleInput } from "@components/config/util";
import { usableResourcePath } from "@lib/utils";

export const ResourceDescription = ({
  type,
  id,
}: {
  type: UsableResource;
  id: string;
}) => {
  const { toast } = useToast();
  const inv = useInvalidate();

  const key =
    type === "ServerTemplate" ? "server_template" : type.toLowerCase();

  const resource = useRead(`Get${type}`, {
    [key]: id,
  } as any).data;

  const { mutate: update_description } = useWrite("UpdateDescription", {
    onSuccess: () => {
      inv([`Get${type}`]);
      toast({ title: `Updated description on ${type} ${resource?.name}` });
    },
  });

  return (
    <TextUpdateMenu
      title="Update Description"
      placeholder="Set Description"
      value={resource?.description}
      onUpdate={(description) =>
        update_description({
          target: { type, id },
          description,
        })
      }
      triggerClassName="text-muted-foreground w-[300px]"
    />
  );
};

export const ResourceSelector = ({
  type,
  selected,
  onSelect,
  disabled,
}: {
  type: UsableResource;
  selected: string | undefined;
  onSelect?: (id: string) => void;
  disabled?: boolean;
}) => {
  const [open, setOpen] = useState(false);
  const [input, setInput] = useState("");

  const resources = useRead(`List${type}s`, {}).data;
  const name = resources?.find((r) => r.id === selected)?.name;

  if (!resources) return null;

  return (
    <Popover open={open} onOpenChange={setOpen}>
      <PopoverTrigger asChild>
        <Button variant="secondary" className="flex gap-2" disabled={disabled}>
          {name ?? `Select ${type}`}
          {!disabled && <ChevronsUpDown className="w-3 h-3" />}
        </Button>
      </PopoverTrigger>
      <PopoverContent className="w-[200px] max-h-[200px] p-0" sideOffset={12}>
        <Command>
          <CommandInput
            placeholder={`Search ${type}s`}
            className="h-9"
            value={input}
            onValueChange={setInput}
          />
          <CommandList>
            <CommandEmpty className="flex justify-evenly items-center">
              {`No ${type}s Found`}
              <SearchX className="w-3 h-3" />
            </CommandEmpty>

            <CommandGroup>
              {resources.map((resource) => (
                <CommandItem
                  key={resource.id}
                  onSelect={() => {
                    onSelect && onSelect(resource.id);
                    setOpen(false);
                  }}
                  className="flex items-center justify-between cursor-pointer"
                >
                  <div className="p-1">{resource.name}</div>
                </CommandItem>
              ))}
            </CommandGroup>
          </CommandList>
        </Command>
      </PopoverContent>
    </Popover>
  );
};

export const ResourceLink = ({
  type,
  id,
}: {
  type: UsableResource;
  id: string;
}) => {
  const Components = ResourceComponents[type];
  return (
    <Link to={`/${usableResourcePath(type)}/${id}`}>
      <Button variant="link" className="flex gap-2 items-center p-0">
        <Components.Icon id={id} />
        <Components.Name id={id} />
      </Button>
    </Link>
  );
};

export const CopyResource = ({
  id,
  disabled,
  type,
}: {
  id: string;
  disabled?: boolean;
  type: Exclude<UsableResource, "Server">;
}) => {
  const [open, setOpen] = useState(false);
  const [name, setName] = useState("");

  const nav = useNavigate();
  const inv = useInvalidate();
  const { mutate } = useWrite(`Copy${type}`, {
    onSuccess: (res) => {
      inv([`List${type}s`]);
      nav(`/${usableResourcePath(type)}/${res._id?.$oid}`);
    },
  });

  return (
    <Dialog open={open} onOpenChange={setOpen}>
      <DialogTrigger asChild>
        <ActionButton
          title="Copy"
          icon={<Copy className="w-4 h-4" />}
          disabled={disabled}
          onClick={() => setOpen(true)}
        />
      </DialogTrigger>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Copy {type}</DialogTitle>
        </DialogHeader>
        <div className="flex flex-col gap-4 my-4">
          <p>Provide a name for the newly created {type}.</p>
          <Input value={name} onChange={(e) => setName(e.target.value)} />
        </div>
        <DialogFooter>
          <ConfirmButton
            title="Copy"
            icon={<Check className="w-4 h-4" />}
            disabled={!name}
            onClick={() => {
              mutate({ id, name });
              setOpen(false);
            }}
          />
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
};

export const NewResource = ({ type }: { type: UsableResource }) => {
  const nav = useNavigate();
  const { mutateAsync } = useWrite(`Create${type}`);
  const [name, setName] = useState("");
  const type_display =
    type === "ServerTemplate" ? "server-template" : type.toLowerCase();
  return (
    <NewLayout
      entityType={type}
      onSuccess={async () => {
        const res = await mutateAsync({ name, config: {} });
        nav(``);
        return res;
      }}
      enabled={!!name}
      onOpenChange={() => setName("")}
    >
      <div className="grid md:grid-cols-2">
        {type} Name
        <Input
          placeholder={`${type_display}-name`}
          value={name}
          onChange={(e) => setName(e.target.value)}
        />
      </div>
    </NewLayout>
  );
};

export const DeleteResource = ({
  type,
  id,
}: {
  type: UsableResource;
  id: string;
}) => {
  const nav = useNavigate();
  const key =
    type === "ServerTemplate" ? "server_template" : type.toLowerCase();
  const resource = useRead(`Get${type}`, {
    [key]: id,
  } as any).data;
  const { mutateAsync, isPending } = useWrite(`Delete${type}`);

  if (!resource) return null;

  return (
    <div className="flex items-center justify-between">
      <div className="w-full">Delete {type}</div>
      <ActionWithDialog
        name={resource.name}
        title="Delete"
        variant="destructive"
        icon={<Trash className="h-4 w-4" />}
        onClick={async () => {
          await mutateAsync({ id });
          nav(`/${usableResourcePath(type)}`);
        }}
        disabled={isPending}
        loading={isPending}
      />
    </div>
  );
};

export const LabelsConfig = ({
  labels,
  set,
  disabled,
}: {
  labels: Types.EnvironmentVar[];
  set: (input: Partial<Types.DeploymentConfig | Types.BuildConfig>) => void;
  disabled: boolean;
}) => (
  <ConfigItem label="Labels" className="items-start">
    <DoubleInput
      disabled={disabled}
      values={labels}
      leftval="variable"
      leftpl="Key"
      rightval="value"
      rightpl="Value"
      addName="Label"
      onLeftChange={(variable, i) => {
        labels[i].variable = variable;
        set({ labels: [...labels] });
      }}
      onRightChange={(value, i) => {
        labels[i].value = value;
        set({ labels: [...labels] });
      }}
      onAdd={() =>
        set({ labels: [...(labels ?? []), { variable: "", value: "" }] })
      }
      onRemove={(idx) =>
        set({ labels: [...labels.filter((_, i) => i !== idx)] })
      }
    />
  </ConfigItem>
);
