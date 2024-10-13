import { Section } from "@components/layouts";
import { MonacoDiffEditor, MonacoEditor } from "@components/monaco";
import { useExecute, useRead } from "@lib/hooks";
import { Card, CardContent, CardHeader } from "@ui/card";
import { ReactNode } from "react";
import { ResourceLink } from "../common";
import { UsableResource } from "@types";
import { diff_type_intention, text_color_class_by_intention } from "@lib/color";
import { cn, sanitizeOnlySpan } from "@lib/utils";
import { ConfirmButton } from "@components/util";
import { SquarePlay } from "lucide-react";
import { useEditPermissions } from "@pages/resource";
import { useFullResourceSync } from ".";

export const ResourceSyncPending = ({
  id,
  titleOther,
}: {
  id: string;
  titleOther: ReactNode;
}) => {
  const syncing = useRead("GetResourceSyncActionState", { sync: id }).data
    ?.syncing;
  const sync = useFullResourceSync(id);
  const { canExecute } = useEditPermissions({ type: "ResourceSync", id });
  const { mutate, isPending } = useExecute("RunSync");
  const loading = isPending || syncing;
  return (
    <Section titleOther={titleOther}>
      {/* Pending Error */}
      {sync?.info?.pending_error && sync.info.pending_error.length ? (
        <Card>
          <CardHeader
            className={cn(
              "font-mono pb-2",
              text_color_class_by_intention("Critical")
            )}
          >
            Error
          </CardHeader>
          <CardContent>
            <pre
              dangerouslySetInnerHTML={{
                __html: sanitizeOnlySpan(sync.info.pending_error),
              }}
            />
          </CardContent>
        </Card>
      ) : undefined}

      {/* Pending Deploy */}
      {sync?.info?.pending_deploy?.to_deploy ? (
        <Card>
          <CardHeader
            className={cn(
              "font-mono pb-2",
              text_color_class_by_intention("Warning")
            )}
          >
            Deploy {sync.info.pending_deploy.to_deploy} Resource
            {sync.info.pending_deploy.to_deploy > 1 ? "s" : ""}
          </CardHeader>
          <CardContent>
            <pre
              dangerouslySetInnerHTML={{
                __html: sanitizeOnlySpan(sync.info.pending_deploy.log),
              }}
            />
          </CardContent>
        </Card>
      ) : undefined}

      {/* Pending Resource Update */}
      {sync?.info?.resource_updates?.map((update) => {
        return (
          <Card key={update.target.type + update.target.id}>
            <CardHeader className="pb-4 flex flex-row justify-between items-center">
              <div className="flex items-center gap-4 font-mono">
                <div
                  className={text_color_class_by_intention(
                    diff_type_intention(update.data.type)
                  )}
                >
                  {update.data.type} {update.target.type}
                </div>
                <div className="text-muted-foreground">|</div>
                {update.data.type === "Create" ? (
                  <div>{update.data.data.name}</div>
                ) : (
                  <ResourceLink
                    type={update.target.type as UsableResource}
                    id={update.target.id}
                  />
                )}
              </div>
              {canExecute && (
                <ConfirmButton
                  title="Execute Change"
                  icon={<SquarePlay className="w-4 h-4" />}
                  onClick={() =>
                    mutate({
                      sync: id,
                      resource_type: update.target.type,
                      resources: [
                        update.data.type === "Create"
                          ? update.data.data.name!
                          : update.target.id,
                      ],
                    })
                  }
                  loading={loading}
                />
              )}
            </CardHeader>
            <CardContent>
              {update.data.type === "Create" && (
                <MonacoEditor
                  value={update.data.data.proposed}
                  language="toml"
                  readOnly
                />
              )}
              {update.data.type === "Update" && (
                <MonacoDiffEditor
                  original={update.data.data.current}
                  modified={update.data.data.proposed}
                  language="toml"
                  readOnly
                />
              )}
              {update.data.type === "Delete" && (
                <MonacoEditor
                  value={update.data.data.current}
                  language="toml"
                  readOnly
                />
              )}
            </CardContent>
          </Card>
        );
      })}
      {/* Pending Variable Update */}
      {sync?.info?.variable_updates?.map((data, i) => {
        return (
          <Card key={i}>
            <CardHeader
              className={cn(
                "font-mono pb-2",
                text_color_class_by_intention(diff_type_intention(data.type))
              )}
            >
              {data.type} Variable
            </CardHeader>
            <CardContent>
              {data.type === "Create" && (
                <MonacoEditor
                  value={data.data.proposed}
                  language="toml"
                  readOnly
                />
              )}
              {data.type === "Update" && (
                <MonacoDiffEditor
                  original={data.data.current}
                  modified={data.data.proposed}
                  language="toml"
                  readOnly
                />
              )}
              {data.type === "Delete" && (
                <MonacoEditor
                  value={data.data.current}
                  language="toml"
                  readOnly
                />
              )}
            </CardContent>
          </Card>
        );
      })}
      {/* Pending User Group Update */}
      {sync?.info?.user_group_updates?.map((data, i) => {
        return (
          <Card key={i}>
            <CardHeader
              className={cn(
                "font-mono pb-2",
                text_color_class_by_intention(diff_type_intention(data.type))
              )}
            >
              {data.type} User Group
            </CardHeader>
            <CardContent>
              {data.type === "Create" && (
                <MonacoEditor
                  value={data.data.proposed}
                  language="toml"
                  readOnly
                />
              )}
              {data.type === "Update" && (
                <MonacoDiffEditor
                  original={data.data.current}
                  modified={data.data.proposed}
                  language="toml"
                  readOnly
                />
              )}
              {data.type === "Delete" && (
                <MonacoEditor
                  value={data.data.current}
                  language="toml"
                  readOnly
                />
              )}
            </CardContent>
          </Card>
        );
      })}
    </Section>
  );
};

// const PENDING_TYPE_KEYS: Array<[string, string]> = [
//   ["Server", "server_updates"],
//   ["Deploy", "deploy_updates"],
//   ["Deployment", "deployment_updates"],
//   ["Stack", "stack_updates"],
//   ["Build", "build_updates"],
//   ["Repo", "repo_updates"],
//   ["Procedure", "procedure_updates"],
//   ["Alerter", "alerter_updates"],
//   ["Builder", "builder_updates"],
//   ["Server Template", "server_template_updates"],
//   ["Resource Sync", "resource_sync_updates"],
//   ["Variable", "variable_updates"],
//   ["User Group", "user_group_updates"],
// ];

// export const ResourceSyncPending = ({
//   id,
//   titleOther,
// }: {
//   id: string;
//   titleOther: ReactNode;
// }) => {
//   const sync = useRead(
//     "GetResourceSync",
//     { sync: id },
//     { refetchInterval: 5000 }
//   ).data;
//   const pending = sync?.info?.pending;
//   return (
//     <Section titleOther={titleOther}>
//       {pending?.hash && pending.message && (
//         <Card>
//           <div className="flex items-center gap-4 px-8 py-4">
//             <div className="text-muted-foreground">Latest Commit</div>
//             <div className="text-muted-foreground">|</div>
//             <div>{pending.hash}</div>
//             <div className="text-muted-foreground">|</div>
//             <div>{pending.message}</div>
//           </div>
//         </Card>
//       )}
//       {pending?.data.type === "Ok" &&
//         PENDING_TYPE_KEYS.map(([type, key]) => (
//           <PendingView
//             key={type}
//             type={type}
//             pending={pending.data.data[key]}
//           />
//         ))}
//       {pending?.data.type === "Err" && (
//         <Card>
//           <CardHeader className="flex items-center justify-between gap-4">
//             <CardTitle>Pending Error</CardTitle>
//           </CardHeader>
//           <CardContent>
//             <pre
//               dangerouslySetInnerHTML={{
//                 __html: sanitizeOnlySpan(pending.data.data.message),
//               }}
//             />
//           </CardContent>
//         </Card>
//       )}
//     </Section>
//   );
// };

// const PendingView = ({
//   type,
//   pending,
// }: {
//   type: string;
//   pending: Types.SyncUpdate | undefined;
// }) => {
//   if (!pending) return;

//   return (
//     <Card>
//       <div className="flex items-center gap-4 px-8 py-4">
//         <CardTitle>{type} Updates</CardTitle>
//         <div className="flex gap-4 items-center m-0">
//           {pending.to_create ? (
//             <>
//               <div className="text-muted-foreground">|</div>
//               <div className="flex gap-2 items-center">
//                 <div className="text-muted-foreground">To Create:</div>
//                 <div>{pending.to_create}</div>
//               </div>
//             </>
//           ) : undefined}
//           {pending.to_update ? (
//             <>
//               <div className="text-muted-foreground">|</div>
//               <div className="flex gap-2 items-center">
//                 <div className="text-muted-foreground">To Update:</div>
//                 <div>{pending.to_update}</div>
//               </div>
//             </>
//           ) : undefined}
//           {pending.to_delete ? (
//             <>
//               <div className="text-muted-foreground">|</div>
//               <div className="flex gap-2 items-center">
//                 <div className="text-muted-foreground">To Delete:</div>
//                 <div>{pending.to_delete}</div>
//               </div>
//             </>
//           ) : undefined}
//         </div>
//       </div>
//       <CardContent>
//         <pre
//           dangerouslySetInnerHTML={{
//             __html: sanitizeOnlySpan(pending.log),
//           }}
//         />
//       </CardContent>
//     </Card>
//   );
// };
