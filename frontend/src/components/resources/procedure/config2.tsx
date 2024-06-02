import { useRead } from "@lib/hooks";
import { Types } from "@monitor/client";

export const ProcedureConfig = ({ id }: { id: string }) => {
  const procedure = useRead("GetProcedure", { procedure: id }).data;
  if (!procedure) return null;
  return <ProcedureConfigInner procedure={procedure} />;
};

const ProcedureConfigInner = ({
  procedure,
}: {
  procedure: Types.Procedure;
}) => {
  // const [branch, setBranch] = useState("main");
  // const [config, setConfig] = useState<Partial<Types.ProcedureConfig>>({});
  // const perms = useRead("GetPermissionLevel", {
  //   target: { type: "Procedure", id: procedure._id?.$oid! },
  // }).data;
  // const global_disabled =
  //   useRead("GetCoreInfo", {}).data?.ui_write_disabled ?? false;
  // const { mutateAsync } = useWrite("UpdateProcedure");
  // const stages = config.stages || procedure.config.stages || [];

  // const disabled = global_disabled || perms !== Types.PermissionLevel.Write;
  return <>{procedure.name}</>

  // return (
  //   <>
  //     <ConfigLayout
  //       disabled={disabled}
  //       config={config}
  //       onConfirm={async () => {
  //         await mutateAsync({ id: procedure._id!.$oid, config });
  //         setConfig({});
  //       }}
  //       onReset={() => setConfig({})}
  //     >
  //       <div></div>
  //     </ConfigLayout>
  //     <Section>
  //       <Card>
  //         <CardHeader className="p-4">
  //           <ConfigItem label="Github Webhook" className="items-start">
  //             <div className="flex flex-col gap-4">
  //               <div className="flex items-center gap-4">
  //                 <div className="flex items-center gap-2">
  //                   <div className="text-nowrap text-muted-foreground">
  //                     Listen on branch:
  //                   </div>
  //                   <Input
  //                     placeholder="Branch"
  //                     value={branch}
  //                     onChange={(e) => setBranch(e.target.value)}
  //                     className="w-[200px]"
  //                   />
  //                 </div>
  //                 <CopyGithubWebhook
  //                   path={`/procedure/${procedure._id?.$oid!}/${branch}`}
  //                 />
  //               </div>
  //               <div className="flex items-center justify-end gap-4 w-full">
  //                 <div className="text-muted-foreground">Enabled:</div>
  //                 <Switch
  //                   checked={
  //                     config.webhook_enabled ?? procedure.config.webhook_enabled
  //                   }
  //                   onCheckedChange={(webhook_enabled) =>
  //                     setConfig({ ...config, webhook_enabled })
  //                   }
  //                   disabled={disabled}
  //                 />
  //               </div>
  //             </div>
  //           </ConfigItem>
  //         </CardHeader>
  //       </Card>
  //     </Section>
  //   </>
  // );
};
