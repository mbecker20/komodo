import { NewDeployment } from "@resources/deployment/new";
import { Button } from "@ui/button";
import { Input } from "@ui/input";
import { PlusCircle } from "lucide-react";
import { ReactNode, useState } from "react";

export const Resources = ({
  type,
  info,
  icon,
  components,
}: {
  type: string;
  info: string;
  icon: ReactNode;
  components: (search: string) => ReactNode;
}) => {
  const [search, setSearch] = useState("");
  const [open, setOpen] = useState(false);
  return (
    <div className="flex flex-col gap-12">
      <div className="flex justify-between">
        <div>
          <h1 className="text-4xl">{type}s</h1>
          <div className="flex items-center gap-2 text-muted-foreground">
            {icon}
            <h2 className="text-xl">{info}</h2>
          </div>
        </div>
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
      </div>
      <div className="grid grid-cols-3 gap-8">{components(search)}</div>
    </div>
  );
};
