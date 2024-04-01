import { Types } from "@monitor/client";
import { ReactNode } from "react";

export type UsableResource = Exclude<Types.ResourceTarget["type"], "System">;

type IdComponent = React.FC<{ id: string }>;
type OptionalIdComponent = React.FC<{ id?: string }>;

export interface RequiredResourceComponents {
  Icon: OptionalIdComponent;

  New: React.FC;

  /// Used on the dashboard
  Dashboard: React.FC;

  Name: IdComponent;
  Description: IdComponent;
  Status: IdComponent;
  Link: IdComponent;
  
  Info: IdComponent[];
  Actions: IdComponent[];

  Table: React.FC;

  Page: { [section: string]: IdComponent };
}
