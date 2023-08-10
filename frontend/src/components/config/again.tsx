import { Resource } from "@monitor/client/dist/types";
import { ReactNode } from "react";

const keys = <T extends {}>(obj: T) => Object.keys(obj) as Array<keyof T>;

export const ConfigAgain = <T extends Resource<unknown, unknown>["config"]>({
  config,
  update,
  components,
}: //   set,
{
  config: T;
  update: Partial<T>;
  components: {
    [K in keyof T]: (value: T[K]) => ReactNode;
  };
  //   set: React.Dispatch<React.SetStateAction<Partial<T>>>;
}) => {
  return (
    <>
      {keys(components).map((key) => (
        <>{components[key](update[key] ?? config[key])}</>
      ))}
    </>
  );
};
