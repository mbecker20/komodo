import { useRead } from "@hooks";
import { ServerCard } from "@resources/server/card";
import { Button } from "@ui/button";
import { Input } from "@ui/input";
import { PlusCircle } from "lucide-react";
import { useState } from "react";

const Resources = () => {};

export const Servers = () => {
  const servers = useRead({ type: "ListServers", params: {} }).data;
  const [search, set] = useState("");

  return (
    <div className="flex flex-col gap-12">
      <div className="flex justify-between">
        <h1 className="text-3xl">Servers</h1>
        <div className="flex gap-4">
          <Input
            className="w-[300px]"
            placeholder="Search"
            value={search}
            onChange={(e) => set(e.target.value)}
          />
          <Button
            className="w-[200px] flex items-center gap-2"
            variant="outline"
            intent="success"
          >
            <PlusCircle className="w-4 h-4 text-green-500" />
            New Server
          </Button>
        </div>
      </div>{" "}
      <div className="grid grid-cols-3 gap-8">
        {servers?.map(
          ({ id, name }) =>
            (search.includes(name) || name.includes(search)) && (
              <ServerCard key={id} id={id} />
            )
        )}
      </div>
    </div>
  );
};
