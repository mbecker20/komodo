import { useRead } from "@lib/hooks";
import { Button } from "@ui/button";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from "@ui/dropdown-menu";
import { AlertTriangle, Clock } from "lucide-react";
import { AlertLevel } from ".";
import { ResourceLink } from "@components/resources/common";
import { UsableResource } from "@types";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from "@ui/dialog";
import { Types } from "@monitor/client";
import { useState } from "react";

export const TopbarAlerts = () => {
  const { data } = useRead("ListAlerts", { query: { resolved: false } });
  const [alert, setAlert] = useState<Types.Alert>();

  return (
    <>
      <DropdownMenu>
        <DropdownMenuTrigger disabled={!data?.alerts.length}>
          <Button variant="ghost" size="icon" className="relative">
            <AlertTriangle className="w-4 h-4" />
            {!!data?.alerts.length && (
              <div className="absolute top-0 right-0 w-4 h-4 bg-red-500 flex items-center justify-center text-[10px] text-white rounded-full">
                {data.alerts.length}
              </div>
            )}
          </Button>
        </DropdownMenuTrigger>
        <DropdownMenuContent>
          {data?.alerts.map((alert) => (
            <DropdownMenuItem className="flex items-center gap-8 border-b last:border-none">
              <div className="w-24">
                <AlertLevel level={alert.level} />
              </div>
              <div className="w-64">
                <ResourceLink
                  type={alert.target.type as UsableResource}
                  id={alert.target.id}
                />
              </div>
              <p className="w-64">{alert.data.type}</p>
            </DropdownMenuItem>
          ))}
        </DropdownMenuContent>
      </DropdownMenu>
    </>
  );
};

const AlertDetails = ({
  alert,
  onClose,
}: {
  alert: Types.Alert | undefined;
  onClose: () => void;
}) => (
  <Dialog open={!!alert}>
    <DialogTrigger asChild>
      <Button variant="secondary" className="items-center gap-2">
        Details
      </Button>
    </DialogTrigger>
    <DialogContent>
      <DialogHeader>
        <DialogTitle>{alert?.target.type}</DialogTitle>
        <DialogDescription>
          <ResourceLink
            type={alert?.target.type as UsableResource}
            id={alert?.target.id!}
          />
        </DialogDescription>
      </DialogHeader>
      <div className="py-8 flex flex-col gap-4">
        <p className="flex gap-4">
          <Clock /> {new Date(alert?.ts!).toLocaleString()}
        </p>
      </div>
    </DialogContent>
  </Dialog>
);
