import { useRead } from "@hooks";
import { ServerCard } from "@pages/dashboard";

export const Servers = () => {
  const servers = useRead({ type: "ListServers", params: {} }).data;

  return (
    <div className="flex flex-col gap-12">
      <h1 className="text-3xl">Servers</h1>
      <div className="grid grid-cols-4 gap-8">
        {servers?.map(({ id }) => (
          <ServerCard key={id} id={id} />
        ))}
      </div>
    </div>
  );
};
