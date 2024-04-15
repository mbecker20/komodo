import { ResourceLink } from "@components/resources/common";
import { useRead } from "@lib/hooks";
import { UsableResource } from "@types";
import { Button } from "@ui/button";
import { Dialog, DialogContent, DialogHeader, DialogTrigger } from "@ui/dialog";
import { useState } from "react";
import { AlertLevel } from ".";
import { fmt_date_with_minutes } from "@lib/formatting";

export const AlertDetailsDialog = ({ id }: { id: string }) => {
  const [open, set] = useState(false);
  const alert = useRead("GetAlert", { id }).data;
  return (
    <Dialog open={open} onOpenChange={set}>
      <DialogTrigger asChild>
        <Button variant="secondary" className="items-center gap-2">
          Details
        </Button>
      </DialogTrigger>
      <DialogContent className="w-fit min-w-[30vw] max-w-[90vw]">
        {alert && (
          <>
            <DialogHeader className="flex-row justify-between w-full">
              {alert && (
                <>
                  <div className="flex gap-4 items-center">
                    <ResourceLink
                      type={alert.target.type as UsableResource}
                      id={alert.target.id}
                    />
                    <AlertLevel level={alert.level} />
                  </div>
                  <div className="text-muted-foreground">
                    {fmt_date_with_minutes(new Date(alert.ts))}
                  </div>
                </>
              )}
            </DialogHeader>
            <pre>{JSON.stringify(alert.data, undefined, 2)}</pre>
          </>
        )}
      </DialogContent>
    </Dialog>
  );
};
