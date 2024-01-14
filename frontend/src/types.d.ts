import { Types } from "@monitor/client";

export type UsableResource = Exclude<Types.ResourceTarget["type"], "System">;

type IdComponent = React.FC<{ id: string }>;
type OptionalIdComponent = React.FC<{ id?: string }>;

export interface RequiredResourceComponents {
  Icon: IdComponent;

  New: React.FC;

  Name: IdComponent;
  Description: IdComponent;
  Info: IdComponent;
  Actions: IdComponent;

  Table: React.FC;

  Page: { [section: string]: IdComponent };
}
