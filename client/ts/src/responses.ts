import * as Res from "./types";

export type AuthResponses = {
  GetLoginOptions: Res.GetLoginOptionsResponse;
  CreateLocalUser: Res.CreateLocalUserResponse;
  LoginLocalUser: Res.LoginLocalUserResponse;
  ExchangeForJwt: Res.ExchangeForJwtResponse;
  LoginWithSecret: Res.LoginWithSecretResponse;
};

export type ApiResponses = {
  // ==== SECRET ====
  CreateLoginSecret: Res.CreateLoginSecretResponse;
  DeleteLoginSecret: undefined;

  //
  // ==== SERVER ====
  //
  GetPeripheryVersion: Res.GetPeripheryVersionResponse;
  GetSystemInformation: Res.SystemInformation;
  GetDockerContainers: Res.BasicContainerInfo[];
  GetDockerImages: Res.ImageSummary[];
  GetDockerNetworks: Res.DockerNetwork[];
  GetServer: Res.Server;
  ListServers: Res.Server[];
  // CRUD
  CreateServer: Res.Server;
  DeleteServer: Res.Server;
  UpdateServer: Res.Server;
  RenameServer: Res.Update;
  // STATS
  GetAllSystemStats: Res.AllSystemStats;
  GetBasicSystemStats: Res.BasicSystemStats;
  GetCpuUsage: Res.CpuUsage;
  GetDiskUsage: Res.DiskUsage;
  GetNetworkUsage: Res.NetworkUsage;
  GetSystemProcesses: Res.SystemProcess[];
  GetSystemComponents: Res.SystemComponent[];
  // ACTIONS
  PruneContainers: Res.Update;
  PruneImages: Res.Update;
  PruneNetworks: Res.Update;

  //
  // ==== DEPLOYMENT ====
  //
  GetDeployment: Res.Deployment;
  ListDeployments: Res.Deployment[];
  // CRUD
  CreateDeployment: Res.Deployment;
  DeleteDeployment: Res.Deployment;
  UpdateDeployment: Res.Deployment;
  RenameDeployment: Res.Update;
  // ACTIONS
  Deploy: Res.Update;
  StartContainer: Res.Update;
  StopContainer: Res.Update;
  RemoveContainer: Res.Update;

  //
  // ==== BUILD ====
  //
  GetBuild: Res.Build;
  ListBuilds: Res.Build[];
  // CRUD
  CreateBuild: Res.Build;
  DeleteBuild: Res.Build;
  UpdateBuild: Res.Build;
  // ACTIONS
  RunBuild: Res.Update;
};
