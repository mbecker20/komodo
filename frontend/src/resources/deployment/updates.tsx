import { useRead } from "@hooks";
import { Table, TableCell, TableHead, TableHeader, TableRow } from "@ui/table";
import { fmt_update_date } from "@util/helpers";
import { Check, X } from "lucide-react";
import { useParams } from "react-router-dom";

export const DeploymentUpdates = () => {
  const deploymentId = useParams().deploymentId;
  const updates = useRead("ListUpdates", { target: { id: deploymentId } }).data;

  return (
    <Table>
      <TableHeader>
        <TableRow>
          <TableHead>Start</TableHead>
          <TableHead>End</TableHead>
          <TableHead>Operation</TableHead>
          <TableHead>Operator</TableHead>
          <TableHead>Success</TableHead>
        </TableRow>
      </TableHeader>
      {updates?.updates.map((update) => (
        <TableRow>
          <TableCell>{fmt_update_date(new Date(update.start_ts))}</TableCell>
          <TableCell>
            {update.end_ts
              ? fmt_update_date(new Date(update.end_ts))
              : "ongoing..."}
          </TableCell>
          <TableCell>{update.operation}</TableCell>
          <TableCell>{update.operator}</TableCell>
          <TableCell>
            {update.success ? (
              <Check className="w-4 h-4 stroke-green-500" />
            ) : (
              <X className="w-4 h-4 stroke-red-500" />
            )}
          </TableCell>
        </TableRow>
      ))}
    </Table>
  );
};
