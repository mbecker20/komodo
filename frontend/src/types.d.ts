import { Types } from "@monitor/client";

export type UsableResource = Exclude<Types.ResourceTarget["type"], "System">;

type IdComponent = React.FC<{ id: string }>;
type OptionalIdComponent = React.FC<{ id?: string }>;

export interface RequiredResourceComponents {
  /** Summary card for use in dashboard */
  Dashboard: React.FC;

  /** New resource button / dialog */
  New: React.FC;

  /** A table component to view resource list */
  Table: React.FC<{ search?: string }>;

  /** Name of the resource */
  Name: IdComponent;

  /** Icon for the component */
  Icon: OptionalIdComponent;

  /** status metrics, like deployment state / status */
  Status: IdComponent[];

  /**
   * Some config items shown in header, like deployment server /image
   * or build repo / branch
   */
  Info: IdComponent[];

  /** Action buttons */
  Actions: IdComponent[];

  /** Resource specific sections */
  Page: { [section: string]: IdComponent };

  /** Config component for resource */
  Config: IdComponent;

  /** Danger zone for resource, containing eg rename, delete */
  DangerZone: IdComponent;
}
