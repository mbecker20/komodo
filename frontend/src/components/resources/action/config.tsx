import { ConfigItem } from "@components/config/util";
import { Section } from "@components/layouts";
import { useRead, useWrite } from "@lib/hooks";
import { Types } from "komodo_client";
import { Card, CardHeader } from "@ui/card";
import { Input } from "@ui/input";
import { useEffect, useState } from "react";
import { CopyGithubWebhook, ResourceSelector } from "../common";
import { ConfigLayout } from "@components/config";
import { Popover, PopoverContent, PopoverTrigger } from "@ui/popover";
import { Button } from "@ui/button";
import {
  ArrowDown,
  ArrowUp,
  ChevronsUpDown,
  Info,
  Minus,
  MinusCircle,
  Plus,
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
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from "@ui/dropdown-menu";
import { DotsHorizontalIcon } from "@radix-ui/react-icons";
import { filterBySplit } from "@lib/utils";
import { useToast } from "@ui/use-toast";
import { fmt_upper_camelcase } from "@lib/formatting";

export const ActionConfig = ({ id }: { id: string }) => {
  const procedure = useRead("GetAction", { action: id }).data;
  if (!procedure) return null;
  return <ActionConfigInner procedure={procedure} />;
};

const ActionConfigInner = ({
  procedure,
}: {
  procedure: Types.Action;
}) => {
  const [branch, setBranch] = useState("main");
  const [config, setConfig] = useState<Partial<Types.ActionConfig>>({});
  const perms = useRead("GetPermissionLevel", {
    target: { type: "Action", id: procedure._id?.$oid! },
  }).data;
  const global_disabled =
    useRead("GetCoreInfo", {}).data?.ui_write_disabled ?? false;
  const { mutateAsync } = useWrite("UpdateAction");
  // const stages = config.stages || procedure.config?.stages || [];

  const disabled = global_disabled || perms !== Types.PermissionLevel.Write;

  return <></>
};