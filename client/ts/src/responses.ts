import * as Res from "./types";

export type AuthResponses = {
  GetLoginOptions: Res.GetLoginOptionsResponse;
  CreateLocalUser: Res.CreateLocalUserResponse;
  LoginLocalUser: Res.LoginLocalUserResponse;
  ExchangeForJwt: Res.ExchangeForJwtResponse;
  LoginWithSecret: Res.LoginWithSecretResponse;
};

export type ReadResponses = {
  GetVersion: Res.GetVersion;
  GetUser: Res.GetUser;

  // ==== SEARCH ====
  FindResources: Res.FindResources;

  // ==== SERVER ====
  GetPeripheryVersion: Res.GetPeripheryVersion;
  GetSystemInformation: Res.GetSystemInformation;
  GetDockerContainers: Res.GetDockerContainers;
  GetDockerImages: Res.GetDockerImages;
  GetDockerNetworks: Res.GetDockerNetworks;
  GetServer: Res.GetServer;
  ListServers: Res.ListServers;
  GetServerActionState: Res.GetServerActionState;

  // ==== DEPLOYMENT ====
  GetDeployment: Res.GetDeployment;
  ListDeployments: Res.ListDeployments;
  GetDeploymentActionState: Res.GetDeploymentActionState;

  // ==== BUILD ====
  GetBuild: Res.GetBuild;
  ListBuilds: Res.ListBuilds;
  GetBuildActionState: Res.GetBuildActionState;

  // ==== BUILDER ====
  GetBuilder: Res.GetBuilder;
  ListBuilders: Res.ListBuilders;

  // ==== REPO ====
  GetRepo: Res.GetRepo;
  ListRepos: Res.ListRepos;
  GetRepoActionState: Res.GetRepoActionState;

  // ==== TAG ====
  GetTag: Res.GetTag;
  ListTags: Res.ListTags;

  // ==== SERVER STATS ====
  GetAllSystemStats: Res.GetAllSystemStats;
  GetBasicSystemStats: Res.GetBasicSystemStats;
  GetCpuUsage: Res.GetCpuUsage;
  GetDiskUsage: Res.GetDiskUsage;
  GetNetworkUsage: Res.GetNetworkUsage;
  GetSystemProcesses: Res.GetSystemProcesses;
  GetSystemComponents: Res.GetSystemComponents;
};

export type WriteResponses = {
  // ==== SECRET ====
  CreateLoginSecret: Res.CreateLoginSecret;
  DeleteLoginSecret: Res.DeleteLoginSecret;

  // ==== PERMISSIONS ====
  UpdateUserPerimissions: Res.UpdateUserPermissions;
  UpdateUserPermissionsOnTarget: Res.UpdateUserPermissionsOnTarget;

  // ==== SERVER ====
  CreateServer: Res.CreateServer;
  DeleteServer: Res.DeleteServer;
  UpdateServer: Res.UpdateServer;
  RenameServer: Res.RenameServer;

  // ==== DEPLOYMENT ====
  CreateDeployment: Res.CreateDeployment;
  CopyDeployment: Res.CopyDeployment;
  DeleteDeployment: Res.DeleteDeployment;
  UpdateDeployment: Res.UpdateDeployment;
  RenameDeployment: Res.RenameDeployment;

  // ==== BUILD ====
  CreateBuild: Res.CreateBuild;
  CopyBuild: Res.CopyBuild;
  DeleteBuild: Res.DeleteBuild;
  UpdateBuild: Res.UpdateBuild;

  // ==== BUILDER ====
  CreateBuilder: Res.CreateBuilder;
  CopyBuilder: Res.CopyBuilder;
  DeleteBuilder: Res.DeleteBuilder;
  UpdateBuilder: Res.UpdateBuilder;

  // ==== REPO ====
  CreateRepo: Res.CreateRepo;
  CopyRepo: Res.CopyRepo;
  DeleteRepo: Res.DeleteRepo;
  UpdateRepo: Res.UpdateRepo;

  // ==== ALERTER ====
  CreateAlerter: Res.CreateAlerter;
  CopyAlerter: Res.CopyAlerter;
  DeleteAlerter: Res.DeleteAlerter;
  UpdateAlerter: Res.UpdateAlerter;

  // ==== TAG ====
  CreateTag: Res.CreateTag;
  DeleteTag: Res.DeleteTag;
  UpdateTag: Res.UpdateTag;
};

export type ExecuteResponses = {
  // ==== SERVER ====
  PruneContainers: Res.PruneDockerContainers;
  PruneImages: Res.PruneDockerImages;
  PruneNetworks: Res.PruneDockerNetworks;

  // ==== DEPLOYMENT ====
  Deploy: Res.Deploy;
  StartContainer: Res.StartContainer;
  StopContainer: Res.StopContainer;
  RemoveContainer: Res.RemoveContainer;

  // ==== BUILD ====
  RunBuild: Res.RunBuild;

  // ==== REPO ====
  CloneRepo: Res.CloneRepo;
  PullRepo: Res.PullRepo;
};
