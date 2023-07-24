import { useRead } from "@hooks";
import { BuildCard } from "@resources/build/card";

export const Builds = () => {
  const builds = useRead({ type: "ListBuilds", params: {} }).data;

  return (
    <div className="flex flex-col gap-12">
      <h1 className="text-3xl">Builds</h1>
      <div className="grid grid-cols-4 gap-8">
        {builds?.map(({ id }) => (
          <BuildCard key={id} id={id} />
        ))}
      </div>
    </div>
  );
};
