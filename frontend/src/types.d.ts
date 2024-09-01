import { Types } from "@komodo/client";

export type UsableResource = Exclude<Types.ResourceTarget["type"], "System">;

type IdComponent = React.FC<{ id: string }>;
type OptionalIdComponent = React.FC<{ id?: string }>;

export interface RequiredResourceComponents {
  list_item: (id: string) => Types.ResourceListItem<unknown> | undefined;

  Description: React.FC;

  /** Summary card for use in dashboard */
  Dashboard: React.FC;

  /** New resource button / dialog */
  New: React.FC<{ server_id?: string; build_id?: string }>;

  /** A table component to view resource list */
  Table: React.FC<{ resources: Types.ResourceListItem<unknown>[] }>;

  /** Icon for the component */
  Icon: OptionalIdComponent;
  BigIcon: OptionalIdComponent;

  /** status metrics, like deployment state / status */
  Status: { [status: string]: IdComponent };

  /**
   * Some config items shown in header, like deployment server /image
   * or build repo / branch
   */
  Info: { [info: string]: IdComponent };

  /** Action buttons */
  Actions: { [action: string]: IdComponent };

  /** Resource specific sections */
  Page: { [section: string]: IdComponent };

  /** Config component for resource */
  Config: IdComponent;

  /** Danger zone for resource, containing eg rename, delete */
  DangerZone: IdComponent;
}
