import { NewDeployment } from "@resources/deployment/new";
import { Button } from "@ui/button";
import { Input } from "@ui/input";
import { PlusCircle } from "lucide-react";
import { ReactNode, useState } from "react";
import { Page } from "./page";

export const Resources = ({
  type,
  summary,
  icon,
  components,
}: {
  type: string;
  summary: ReactNode;
  icon: ReactNode;
  components: (search: string) => ReactNode;
}) => {
  const [search, setSearch] = useState("");
  const [open, setOpen] = useState(false);
  return (
    <Page
      title={<h1 className="text-4xl">{type}s</h1>}
      subtitle={
        <h2 className="text-lg text-muted-foreground flex items-center gap-2">
          {icon}
          {summary}
        </h2>
      }
      actions={
        <div className="flex gap-4">
          <Input
            className="w-[300px]"
            placeholder={`Search ${type}s`}
            value={search}
            onChange={(e) => setSearch(e.target.value)}
          />
          <Button
            className="w-[200px] flex items-center gap-2"
            variant="outline"
            intent="success"
            onClick={() => setOpen(true)}
          >
            <PlusCircle className="w-4 h-4 text-green-500" />
            New {type}
          </Button>
          <NewDeployment open={open} set={setOpen} />
        </div>
      }
      content={
        <div className="grid grid-cols-3 gap-8">{components(search)}</div>
      }
    />
  );
};
