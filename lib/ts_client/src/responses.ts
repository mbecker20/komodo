import {
  AllSystemStats,
  BasicContainerInfo,
  BasicSystemStats,
  Build,
  CpuUsage,
  CreateLocalUserResponse,
  CreateLoginSecretResponse,
  Deployment,
  DiskUsage,
  DockerNetwork,
  ExchangeForJwtResponse,
  GetLoginOptionsResponse,
  GetPeripheryVersionResponse,
  ImageSummary,
  LoginLocalUserResponse,
  LoginWithSecretResponse,
  NetworkUsage,
  Server,
  SystemComponent,
  SystemInformation,
  SystemProcess,
  Update,
} from "./types";

export type AuthResponses = {
  GetLoginOptions: GetLoginOptionsResponse;
  CreateLocalUser: CreateLocalUserResponse;
  LoginLocalUser: LoginLocalUserResponse;
  ExchangeForJwt: ExchangeForJwtResponse;
  LoginWithSecret: LoginWithSecretResponse;
};

export type ApiResponses = {
  // ==== SECRET ====
  CreateLoginSecret: CreateLoginSecretResponse;
  DeleteLoginSecret: undefined;

  //
  // ==== SERVER ====
  //
  GetPeripheryVersion: GetPeripheryVersionResponse;
  GetSystemInformation: SystemInformation;
  GetDockerContainers: BasicContainerInfo[];
  GetDockerImages: ImageSummary[];
  GetDockerNetworks: DockerNetwork[];
  GetServer: Server;
  ListServers: Server[];
  // CRUD
  CreateServer: Server;
  DeleteServer: Server;
  UpdateServer: Server;
  RenameServer: Update;
  // STATS
  GetAllSystemStats: AllSystemStats;
  GetBasicSystemStats: BasicSystemStats;
  GetCpuUsage: CpuUsage;
  GetDiskUsage: DiskUsage;
  GetNetworkUsage: NetworkUsage;
  GetSystemProcesses: SystemProcess[];
  GetSystemComponents: SystemComponent[];
  // ACTIONS
  PruneContainers: Update;
  PruneImages: Update;
  PruneNetworks: Update;

  //
  // ==== DEPLOYMENT ====
  //
  GetDeployment: Deployment;
  ListDeployments: Deployment[];
  // CRUD
  CreateDeployment: Deployment;
  DeleteDeployment: Deployment;
  UpdateDeployment: Deployment;
  RenameDeployment: Update;
  // ACTIONS
  Deploy: Update;
  StartContainer: Update;
  StopContainer: Update;
  RemoveContainer: Update;

  //
  // ==== BUILD ====
  //
  GetBuild: Build;
  ListBuilds: Build[];
  // CRUD
  CreateBuild: Build;
  DeleteBuild: Build;
  UpdateBuild: Build;
  // ACTIONS
  RunBuild: Update;
};
