// import { ResourceSelector } from "@components/config/util";
// import { NewResource } from "@components/layouts";
// import { ConfirmButton } from "@components/util";
// import { useExecute, useRead, useWrite } from "@lib/hooks";
// import { fmt_date_with_minutes } from "@lib/utils";
// import { Types } from "@monitor/client";
// import { Execution } from "@monitor/client/dist/types";
// import { Icon } from "@radix-ui/react-select";
// import { RequiredResourceComponents, UsableResource } from "@types";
// import { Button } from "@ui/button";
// import { Card, CardDescription, CardHeader, CardTitle } from "@ui/card";
// import { DataTable } from "@ui/data-table";
// import { Input } from "@ui/input";
// import {
//   Select,
//   SelectContent,
//   SelectGroup,
//   SelectItem,
//   SelectTrigger,
//   SelectValue,
// } from "@ui/select";
// import { Loader2, Route, Save } from "lucide-react";
// import React, { useEffect, useState } from "react";
// import { Link } from "react-router-dom";

// const useProcedure = (id?: string) =>
//   useRead("ListProcedures", {}).data?.find((d) => d.id === id);

// const Name = ({ id }: { id: string }) => <>{useProcedure(id)?.name}</>;

// // const NewProcedure = ({ parent }: { parent?: Types.Procedure }) => {
// //   const [name, setName] = useState("");
// //   const [type, setType] = useState<Types.ProcedureConfig["type"]>("Execution");

// //   const update_parent = useWrite("UpdateProcedure").mutate;

// //   const { mutateAsync } = useWrite("CreateProcedure", {
// //     onSuccess: ({ _id }) => {
// //       if (!parent?._id?.$oid || !_id?.$oid) return;
// //       if (
// //         parent.config.type === "Sequence" ||
// //         parent.config.type === "Parallel"
// //       ) {
// //         update_parent({
// //           id: parent._id.$oid,
// //           config: {
// //             ...parent.config,
// //             data: [...parent.config.data, { enabled: true, id: _id!.$oid }],
// //           },
// //         });
// //       }
// //     },
// //   });

// //   return (
// //     <NewResource
// //       type="Procedure"
// //       onSuccess={() =>
// //         mutateAsync({
// //           name,
// //           config: {
// //             type,
// //             data: get_default_data(type),
// //           } as Types.ProcedureConfig,
// //         })
// //       }
// //       enabled={!!name}
// //     >
// //       <div className="grid md:grid-cols-2">
// //         Procedure Name
// //         <Input
// //           placeholder="procedure-name"
// //           value={name}
// //           onChange={(e) => setName(e.target.value)}
// //         />
// //       </div>
// //       <div className="grid md:grid-cols-2">
// //         Procedure Type
// //         <Select
// //           value={type}
// //           onValueChange={(value) => setType(value as typeof type)}
// //         >
// //           <SelectTrigger>
// //             <SelectValue placeholder="Select Type" />
// //           </SelectTrigger>
// //           <SelectContent>
// //             <SelectGroup>
// //               <SelectItem value="Execution">Execution</SelectItem>
// //               <SelectItem value="Sequence">Sequence</SelectItem>
// //               <SelectItem value="Paralell">Paralell</SelectItem>
// //             </SelectGroup>
// //           </SelectContent>
// //         </Select>
// //       </div>
// //     </NewResource>
// //   );
// // };

// // type ExecutionType = Extract<
// //   Types.ProcedureConfig,
// //   { type: "Execution" }
// // >["data"]["type"];

// // type ExecutionConfigComponent<
// //   T extends ExecutionType,
// //   P = Extract<Execution, { type: T }>["params"]
// // > = React.FC<{
// //   params: P;
// //   setParams: React.Dispatch<React.SetStateAction<P>>;
// // }>;

// // type ExecutionConfigParams<T extends ExecutionType> = Extract<
// //   Execution,
// //   { type: T }
// // >["params"];

// // type ExecutionConfigs = {
// //   [ExType in ExecutionType]: {
// //     component: ExecutionConfigComponent<ExType>;
// //     params: ExecutionConfigParams<ExType>;
// //   };
// // };

// // const TypeSelector = ({
// //   type,
// //   selected,
// //   onSelect,
// // }: {
// //   type: UsableResource;
// //   selected: string;
// //   onSelect: (value: string) => void;
// // }) => (
// //   <div className="flex items-center justify-between">
// //     {type}
// //     <ResourceSelector type={type} selected={selected} onSelect={onSelect} />
// //   </div>
// // );

// // const EXEC_TYPES: ExecutionConfigs = {
// //   None: {
// //     params: {},
// //     component: () => <></>,
// //   },
// //   CloneRepo: {
// //     params: { id: "" },
// //     component: ({ params, setParams }) => (
// //       <TypeSelector
// //         type="Repo"
// //         selected={params.id}
// //         onSelect={(id) => setParams((p) => ({ ...p, id }))}
// //       />
// //     ),
// //   },
// //   Deploy: {
// //     params: { deployment_id: "" },
// //     component: ({ params, setParams }) => (
// //       <TypeSelector
// //         type="Deployment"
// //         selected={params.deployment_id}
// //         onSelect={(id) => setParams((p) => ({ ...p, deployment_id: id }))}
// //       />
// //     ),
// //   },
// //   PruneDockerContainers: {
// //     params: { server_id: "" },
// //     component: ({ params, setParams }) => (
// //       <TypeSelector
// //         type="Server"
// //         selected={params.server_id}
// //         onSelect={(server_id) => setParams((p) => ({ ...p, server_id }))}
// //       />
// //     ),
// //   },
// //   PruneDockerImages: {
// //     params: { server_id: "" },
// //     component: ({ params, setParams }) => (
// //       <TypeSelector
// //         type="Server"
// //         selected={params.server_id}
// //         onSelect={(id) => setParams((p) => ({ ...p, id }))}
// //       />
// //     ),
// //   },
// //   PruneDockerNetworks: {
// //     params: { server_id: "" },
// //     component: ({ params, setParams }) => (
// //       <TypeSelector
// //         type="Server"
// //         selected={params.server_id}
// //         onSelect={(id) => setParams((p) => ({ ...p, id }))}
// //       />
// //     ),
// //   },
// //   PullRepo: {
// //     params: { id: "" },
// //     component: ({ params, setParams }) => (
// //       <TypeSelector
// //         type="Repo"
// //         selected={params.id}
// //         onSelect={(id) => setParams((p) => ({ ...p, id }))}
// //       />
// //     ),
// //   },
// //   RemoveContainer: {
// //     params: { deployment_id: "" },
// //     component: ({ params, setParams }) => (
// //       <TypeSelector
// //         type="Deployment"
// //         selected={params.deployment_id}
// //         onSelect={(id) => setParams((p) => ({ ...p, deployment_id: id }))}
// //       />
// //     ),
// //   },
// //   RunBuild: {
// //     params: { build_id: "" },
// //     component: ({ params, setParams }) => (
// //       <TypeSelector
// //         type="Build"
// //         selected={params.build_id}
// //         onSelect={(build_id) => setParams((p) => ({ ...p, build_id }))}
// //       />
// //     ),
// //   },
// //   RunProcedure: {
// //     params: { procedure_id: "" },
// //     component: ({ params, setParams }) => (
// //       <TypeSelector
// //         type="Procedure"
// //         selected={params.procedure_id}
// //         onSelect={(id) => setParams((p) => ({ ...p, procedure_id: id }))}
// //       />
// //     ),
// //   },
// //   StartContainer: {
// //     params: { deployment_id: "" },
// //     component: ({ params, setParams }) => (
// //       <TypeSelector
// //         type="Deployment"
// //         selected={params.deployment_id}
// //         onSelect={(id) => setParams((p) => ({ ...p, deployment_id: id }))}
// //       />
// //     ),
// //   },
// //   StopAllContainers: {
// //     params: { server_id: "" },
// //     component: ({ params, setParams }) => (
// //       <TypeSelector
// //         type="Server"
// //         selected={params.server_id}
// //         onSelect={(id) => setParams((p) => ({ ...p, server_id: id }))}
// //       />
// //     ),
// //   },
// //   StopContainer: {
// //     params: { deployment_id: "" },
// //     component: ({ params, setParams }) => (
// //       <TypeSelector
// //         type="Deployment"
// //         selected={params.deployment_id}
// //         onSelect={(id) => setParams((p) => ({ ...p, deployment_id: id }))}
// //       />
// //     ),
// //   },
// // };

// const UpdateProcedure = ({
//   id,
//   procedure,
// }: {
//   id: string;
//   procedure: Types.ProcedureConfig;
// }) => {
//   const { mutate } = useWrite("UpdateProcedure");

//   return (
//     <Button onClick={() => mutate({ id, config: procedure })}>
//       <Save className="w-4" />
//     </Button>
//   );
// };

// // const ExecutionConfig = ({ id }: { id: string }) => {
// //   const procedure = useRead("GetProcedure", { id }).data;
// //   if (procedure?.config.type !== "Execution") return null;

// //   // eslint-disable-next-line react-hooks/rules-of-hooks
// //   const [type, setType] = useState<ExecutionType>(procedure.config.data.type);

// //   // eslint-disable-next-line react-hooks/rules-of-hooks
// //   const [params, setParams] = useState(procedure.config.data.params);

// //   // eslint-disable-next-line react-hooks/rules-of-hooks
// //   useEffect(() => {
// //     if (procedure?.config.type !== "Execution") return;
// //     if (type !== procedure.config.data.type) {
// //       setParams(EXEC_TYPES[type].params);
// //     }
// //   }, [procedure, type]);

// //   const Component = EXEC_TYPES[type].component;

// //   return (
// //     <div className="p-4 border rounded-md flex flex-col gap-4">
// //       <div className="flex items-center justify-between">
// //         {procedure.name}
// //         <UpdateProcedure
// //           id={id}
// //           procedure={{ type: "Execution", data: { type, params } as Execution }}
// //         />
// //       </div>
// //       <div className="flex items-center justify-between">
// //         Execution Type
// //         <Select
// //           value={type}
// //           onValueChange={(value) => setType(value as typeof type)}
// //         >
// //           <SelectTrigger className="w-72">
// //             <SelectValue placeholder="Select Type" />
// //           </SelectTrigger>
// //           <SelectContent className="w-72">
// //             <SelectGroup>
// //               {Object.keys(EXEC_TYPES).map((type) => (
// //                 <SelectItem
// //                   value={type}
// //                   className="whitespace-nowrap"
// //                   key={type}
// //                 >
// //                   {type.match(/[A-Z][a-z]+/g)?.join(" ")}
// //                 </SelectItem>
// //               ))}
// //             </SelectGroup>
// //           </SelectContent>
// //         </Select>
// //       </div>
// //       <div className="pt-2 border-t">
// //         {/* eslint-disable-next-line @typescript-eslint/no-explicit-any */}
// //         <Component params={params as any} setParams={setParams as any} />
// //       </div>
// //       <div className="pt-2 border-t">
// //         <pre>{JSON.stringify(procedure?.config, null, 2)}</pre>
// //       </div>
// //     </div>
// //   );
// // };

// // const SequenceConfig = ({ id }: { id: string }) => {
// //   const procedure = useRead("GetProcedure", { id }).data;
// //   if (procedure?.config.type !== "Sequence") return null;

// //   return (
// //     <div className="p-4 border rounded-md flex flex-col gap-4">
// //       <div className="flex items-center justify-between">
// //         {procedure?.name}
// //         <NewProcedure parent={procedure} />
// //       </div>
// //       <pre>{JSON.stringify(procedure?.config, null, 2)}</pre>
// //       <div>
// //         {procedure.config.data.map((p) => (
// //           <ProcedureConfig id={p.id} key={p.id} />
// //         ))}
// //       </div>
// //     </div>
// //   );
// // };

// // export const ProcedureConfig = ({ id }: { id: string }) => {
// //   const procedure = useRead("GetProcedure", { id }).data;
// //   if (procedure?.config.type === "Sequence") return <SequenceConfig id={id} />;
// //   if (procedure?.config.type === "Execution")
// //     return <ExecutionConfig id={id} />;
// // };

// export const ProcedureDashboard = () => {
//   const procedure_count = useRead("ListProcedures", {}).data?.length;
//   return (
//     <Link to="/procedures/" className="w-full">
//       <Card>
//         <CardHeader className="justify-between">
//           <div>
//             <CardTitle>Procedures</CardTitle>
//             <CardDescription>{procedure_count} Total</CardDescription>
//           </div>
//           <Route className="w-4 h-4" />
//         </CardHeader>
//       </Card>
//     </Link>
//   );
// };

// export const ProcedureComponents: RequiredResourceComponents = {
//   Name: ({ id }) => <>{useProcedure(id)?.name}</>,
//   Description: ({ id }) => <>{useProcedure(id)?.info.procedure_type}</>,
//   Info: ({ id }) => <>{id}</>,
//   Icon: () => <Route className="w-4" />,
//   Page: {
//     // Config: ({ id }) => <ProcedureConfig id={id} />,
//   },
//   Actions: ({ id }) => {
//     const running = useRead("GetProcedureActionState", { procedure: id }).data?.running;
//     const { mutate, isPending } = useExecute("RunProcedure");
//     return (
//       <ConfirmButton
//         title={running ? "Building" : "Run"}
//         icon={
//           running ? (
//             <Loader2 className="w-4 h-4 animate-spin" />
//           ) : (
//             <Route className="h-4 w-4" />
//           )
//         }
//         onClick={() => mutate({ procedure: id })}
//         disabled={running || isPending}
//       />
//     );
//   },
//   Table: () => {
//     const alerters = useRead("ListAlerters", {}).data;
//     return (
//       <DataTable
//         data={alerters ?? []}
//         columns={[
//           {
//             accessorKey: "id",
//             header: "Name",
//             cell: ({ row }) => {
//               const id = row.original.id;
//               return (
//                 <Link
//                   to={`/procedures/${id}`}
//                   className="flex items-center gap-2"
//                 >
//                   <Icon id={id} />
//                   <Name id={id} />
//                 </Link>
//               );
//             },
//           },
//           { header: "Tags", accessorFn: ({ tags }) => tags.join(", ") },
//           {
//             header: "Created",
//             accessorFn: ({ created_at }) =>
//               fmt_date_with_minutes(new Date(created_at)),
//           },
//         ]}
//       />
//     );
//   },
//   New: () => <NewProcedure />,
//   Dashboard: ProcedureDashboard,
// };
