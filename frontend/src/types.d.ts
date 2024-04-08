import { Types } from "@monitor/client";

export type UsableResource = Exclude<Types.ResourceTarget["type"], "System">;

type IdComponent = React.FC<{ id: string }>;
type OptionalIdComponent = React.FC<{ id?: string }>;

export interface RequiredResourceComponents {
  Icon: OptionalIdComponent;

  New: React.FC;

  /// Used on the dashboard
  Dashboard: React.FC;

  Name: IdComponent;
  Status: IdComponent;
  Link: IdComponent;

  Info: IdComponent[];
  Actions: IdComponent[];

  Table: React.FC<{ search?: string }>;

  Page: { [section: string]: IdComponent };
}
