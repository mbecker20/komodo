import { Types } from "@monitor/client";

export type UsableResource = Exclude<Types.ResourceTarget["type"], "System">;

export type RequiredComponents =
  | "Name"
  | "Description"
  | "Icon"
  | "Info"
  | "Actions";

export type RequiredResourceComponents = {
  [key in RequiredComponents]: React.FC<{ id: string }>;
} & { Page: { [key: string]: React.FC<{ id: string }> } } & {
  New: () => React.ReactNode;
};
