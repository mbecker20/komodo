import { Types } from "@monitor/client";

export type UsableResource = Exclude<Types.ResourceTarget["type"], "System">;

// export type RequiredComponents =
//   | "Name"
//   | "Description"
//   | "Icon"
//   | "Info"
//   | "Actions";

// export type RequiredResourceComponents = {
//   [key in RequiredComponents]: React.FC<{ id: string }>;
// } & { Page: { [key: string]: React.FC<{ id: string }> } } & {
//   New: () => React.ReactNode;
// };

type IdComponent = React.FC<{ id: string }>;
type OptionalIdComponent = React.FC<{ id?: string }>;

export interface RequiredResourceComponents {
  Icon: OptionalIdComponent;

  Name: IdComponent;
  Description: IdComponent;
  Info: IdComponent;
  Actions: IdComponent;

  New: React.FC;

  Page: { [section: string]: IdComponent };
}
