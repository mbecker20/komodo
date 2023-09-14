import * as Types from "./types";

export type AuthResponses = {
  GetLoginOptions: Types.GetLoginOptionsResponse;
  CreateLocalUser: Types.CreateLocalUserResponse;
  LoginLocalUser: Types.LoginLocalUserResponse;
  ExchangeForJwt: Types.ExchangeForJwtResponse;
  LoginWithSecret: Types.LoginWithSecretResponse;
};

export type ReadResponses = {
  GetVersion: Types.GetVersionResponse;
  GetUser: Types.User;
  GetUsers: Types.User[];
  GetUsername: Types.GetUsernameResponse;
  GetCoreInfo: Types.GetCoreInfoResponse;

  // ==== SEARCH ====
  FindResources: Types.FindResourcesResponse;

  // ==== SERVER ====
  GetServersSummary: Types.GetServersSummaryResponse;
  GetServer: Types.Server;
  ListServers: Types.ServerListItem[];
  GetServerStatus: Types.GetServerStatusResponse;
  GetPeripheryVersion: Types.GetPeripheryVersionResponse;
  GetSystemInformation: Types.SystemInformation;
  GetDockerContainers: Types.ContainerSummary[];
  GetDockerImages: Types.ImageSummary[];
  GetDockerNetworks: Types.DockerNetwork[];
  GetServerActionState: Types.ServerActionState;
  GetHistoricalServerStats: Types.GetHistoricalServerStatsResponse;
  GetServerAvailableAccounts: Types.GetServerAvailableAccountsResponse;
  GetAvailableNetworks: Types.GetAvailableNetworksResponse;

  // ==== DEPLOYMENT ====
  GetDeploymentsSummary: Types.GetDeploymentsSummaryResponse;
  GetDeployment: Types.Deployment;
  ListDeployments: Types.DeploymentListItem[];
  GetDeploymentStatus: Types.GetDeploymentStatusResponse;
  GetDeploymentActionState: Types.DeploymentActionState;
  GetDeployedVersion: Types.GetDeployedVersionResponse;
  GetDeploymentStats: Types.GetDeploymentStatusResponse;
  GetLog: Types.Log;

  // ==== BUILD ====
  GetBuildsSummary: Types.GetBuildsSummaryResponse;
  GetBuild: Types.Build;
  ListBuilds: Types.BuildListItem[];
  GetBuildActionState: Types.BuildActionState;
  GetBuildMonthlyStats: Types.GetBuildMonthlyStatsResponse;
  GetBuildVersions: Types.BuildVersionResponseItem[];

  // ==== REPO ====
  GetReposSummary: Types.GetReposSummaryResponse;
  GetRepo: Types.Repo;
  ListRepos: Types.RepoListItem[];
  GetRepoActionState: Types.RepoActionState;

  // ==== BUILDER ====
  GetBuildersSummary: Types.GetBuildersSummaryResponse;
  GetBuilder: Types.Builder;
  ListBuilders: Types.BuilderListItem[];
  GetBuilderAvailableAccounts: Types.GetBuilderAvailableAccountsResponse;

  // ==== ALERTER ====
  GetAlertersSummary: Types.GetAlertersSummaryResponse;
  GetAlerter: Types.Alerter;
  ListAlerters: Types.AlerterListItem[];

  // ==== TAG ====
  GetTag: Types.CustomTag;
  ListTags: Types.CustomTag[];

  // ==== UPDATE ====
  GetUpdate: Types.Update;
  ListUpdates: Types.ListUpdatesResponse;

  // ==== ALERT ====
  ListAlerts: Types.ListAlertsResponse;

  // ==== SERVER STATS ====
  GetAllSystemStats: Types.AllSystemStats;
  GetBasicSystemStats: Types.BasicSystemStats;
  GetCpuUsage: Types.CpuUsage;
  GetDiskUsage: Types.DiskUsage;
  GetNetworkUsage: Types.NetworkUsage;
  GetSystemProcesses: Types.SystemProcess[];
  GetSystemComponents: Types.SystemComponent[];
};

export type WriteResponses = {
  // ==== SECRET ====
  CreateLoginSecret: Types.CreateLoginSecretResponse;
  DeleteLoginSecret: Types.DeleteLoginSecretResponse;

  // ==== USER ====
  PushRecentlyViewed: Types.PushRecentlyViewedResponse;
  SetLastSeenUpdate: Types.SetLastSeenUpdateResponse;

  // ==== PERMISSIONS ====
  UpdateUserPerimissions: Types.Update;
  UpdateUserPermissionsOnTarget: Types.Update;

  // ==== DESCRIPTION ====
  UpdateDescription: Types.UpdateDescriptionResponse;

  // ==== SERVER ====
  LaunchServer: Types.Update;
  CreateServer: Types.Server;
  DeleteServer: Types.Server;
  UpdateServer: Types.Server;
  RenameServer: Types.Update;
  CreateNetwork: Types.Update;
  DeleteNetwork: Types.Update;

  // ==== DEPLOYMENT ====
  CreateDeployment: Types.Deployment;
  CopyDeployment: Types.Deployment;
  DeleteDeployment: Types.Deployment;
  UpdateDeployment: Types.Deployment;
  RenameDeployment: Types.Update;

  // ==== BUILD ====
  CreateBuild: Types.Build;
  CopyBuild: Types.Build;
  DeleteBuild: Types.Build;
  UpdateBuild: Types.Build;

  // ==== BUILDER ====
  CreateBuilder: Types.Builder;
  CopyBuilder: Types.Builder;
  DeleteBuilder: Types.Builder;
  UpdateBuilder: Types.Builder;

  // ==== REPO ====
  CreateRepo: Types.Repo;
  CopyRepo: Types.Repo;
  DeleteRepo: Types.Repo;
  UpdateRepo: Types.Repo;

  // ==== ALERTER ====
  CreateAlerter: Types.Alerter;
  CopyAlerter: Types.Alerter;
  DeleteAlerter: Types.Alerter;
  UpdateAlerter: Types.Alerter;

  // ==== TAG ====
  CreateTag: Types.CustomTag;
  DeleteTag: Types.CustomTag;
  UpdateTag: Types.CustomTag;
};

export type ExecuteResponses = {
  // ==== SERVER ====
  PruneContainers: Types.Update;
  PruneImages: Types.Update;
  PruneNetworks: Types.Update;

  // ==== DEPLOYMENT ====
  Deploy: Types.Update;
  StartContainer: Types.Update;
  StopContainer: Types.Update;
  RemoveContainer: Types.Update;

  // ==== BUILD ====
  RunBuild: Types.Update;

  // ==== REPO ====
  CloneRepo: Types.Update;
  PullRepo: Types.Update;
};
