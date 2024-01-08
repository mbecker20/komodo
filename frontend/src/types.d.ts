import { Types } from "@monitor/client";

export type UsableResource = Exclude<Types.ResourceTarget["type"], "System">;

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
