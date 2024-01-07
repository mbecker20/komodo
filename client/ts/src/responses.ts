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
  GetUser: Types.GetUserResponse;
  GetUsers: Types.GetUsersResponse;
  GetUsername: Types.GetUsernameResponse;
  GetCoreInfo: Types.GetCoreInfoResponse;

  // ==== SEARCH ====
  FindResources: Types.FindResourcesResponse;

  // ==== PROCEDURE ====
  GetProceduresSummary: Types.GetProceduresSummaryResponse;
  GetProcedure: Types.GetProcedureResponse;
  GetProcedureActionState: Types.GetProcedureActionStateResponse;
  ListProcedures: Types.ListProceduresResponse;
  ListProceduresByIds: Types.ListProceduresByIdsResponse;

  // ==== SERVER ====
  GetServersSummary: Types.GetServersSummaryResponse;
  GetServer: Types.GetServerResponse;
  ListServers: Types.ListServersResponse;
  GetServerStatus: Types.GetServerStatusResponse;
  GetPeripheryVersion: Types.GetPeripheryVersionResponse;
  GetSystemInformation: Types.GetSystemInformationResponse;
  GetDockerContainers: Types.GetDockerContainersResponse;
  GetDockerImages: Types.GetDockerImagesResponse;
  GetDockerNetworks: Types.GetDockerNetworksResponse;
  GetServerActionState: Types.GetServerActionStateResponse;
  GetHistoricalServerStats: Types.GetHistoricalServerStatsResponse;
  GetAvailableAccounts: Types.GetAvailableAccountsResponse;
  GetAvailableSecrets: Types.GetAvailableSecretsResponse;

  // ==== DEPLOYMENT ====
  GetDeploymentsSummary: Types.GetDeploymentsSummaryResponse;
  GetDeployment: Types.GetDeploymentResponse;
  ListDeployments: Types.ListDeploymentsResponse;
  GetDeploymentStatus: Types.GetDeploymentStatusResponse;
  GetDeploymentActionState: Types.GetDeploymentActionStateResponse;
  GetDeployedVersion: Types.GetDeployedVersionResponse;
  GetDeploymentStats: Types.GetDeploymentStatsResponse;
  GetLog: Types.GetLogResponse;

  // ==== BUILD ====
  GetBuildsSummary: Types.GetBuildsSummaryResponse;
  GetBuild: Types.GetBuildResponse;
  ListBuilds: Types.ListBuildsResponse;
  GetBuildActionState: Types.GetBuildActionStateResponse;
  GetBuildMonthlyStats: Types.GetBuildMonthlyStatsResponse;
  GetBuildVersions: Types.GetBuildVersionsResponse;

  // ==== REPO ====
  GetReposSummary: Types.GetReposSummaryResponse;
  GetRepo: Types.GetRepoResponse;
  ListRepos: Types.ListReposResponse;
  GetRepoActionState: Types.GetRepoActionStateResponse;

  // ==== BUILDER ====
  GetBuildersSummary: Types.GetBuildersSummaryResponse;
  GetBuilder: Types.GetBuilderResponse;
  ListBuilders: Types.ListBuildersResponse;
  GetBuilderAvailableAccounts: Types.GetBuilderAvailableAccountsResponse;

  // ==== ALERTER ====
  GetAlertersSummary: Types.GetAlertersSummaryResponse;
  GetAlerter: Types.GetAlerterResponse;
  ListAlerters: Types.ListAlertersResponse;

  // ==== TAG ====
  GetTag: Types.GetTagResponse;
  ListTags: Types.ListTagsResponse;

  // ==== UPDATE ====
  GetUpdate: Types.GetUpdateResponse;
  ListUpdates: Types.ListUpdatesResponse;

  // ==== ALERT ====
  ListAlerts: Types.ListAlertsResponse;

  // ==== SERVER STATS ====
  GetAllSystemStats: Types.GetAllSystemStatsResponse;
  GetBasicSystemStats: Types.GetBasicSystemStatsResponse;
  GetCpuUsage: Types.GetCpuUsageResponse;
  GetDiskUsage: Types.GetDiskUsageResponse;
  GetNetworkUsage: Types.GetNetworkUsageResponse;
  GetSystemProcesses: Types.GetSystemProcessesResponse;
  GetSystemComponents: Types.GetSystemComponentsResponse;
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

  // ==== PROCEDURE ====
  CreateProcedure: Types.Procedure;
  CopyProcedure: Types.Procedure;
  DeleteProcedure: Types.Procedure;
  UpdateProcedure: Types.Procedure;

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
  StopAllContainers: Types.Update;
  RemoveContainer: Types.Update;

  // ==== BUILD ====
  RunBuild: Types.Update;

  // ==== REPO ====
  CloneRepo: Types.Update;
  PullRepo: Types.Update;

  // ==== PROCEDURE ====
  RunProcedure: Types.Update;
};
