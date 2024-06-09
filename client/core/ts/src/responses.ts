import * as Types from "./types";

export type AuthResponses = {
  GetLoginOptions: Types.GetLoginOptionsResponse;
  CreateLocalUser: Types.CreateLocalUserResponse;
  LoginLocalUser: Types.LoginLocalUserResponse;
  ExchangeForJwt: Types.ExchangeForJwtResponse;
  GetUser: Types.GetUserResponse;
};

export type UserResponses = {
  PushRecentlyViewed: Types.PushRecentlyViewedResponse;
  SetLastSeenUpdate: Types.SetLastSeenUpdateResponse;
  CreateApiKey: Types.CreateApiKeyResponse;
  DeleteApiKey: Types.DeleteApiKeyResponse;
};

export type ReadResponses = {
  GetVersion: Types.GetVersionResponse;
  GetCoreInfo: Types.GetCoreInfoResponse;

  // ==== USER ====
  ListUsers: Types.ListUsersResponse;
  GetUsername: Types.GetUsernameResponse;
  ListApiKeys: Types.ListApiKeysResponse;
  ListApiKeysForServiceUser: Types.ListApiKeysForServiceUserResponse;
  ListPermissions: Types.ListPermissionsResponse;
  GetPermissionLevel: Types.GetPermissionLevelResponse;
  ListUserTargetPermissions: Types.ListUserTargetPermissionsResponse;

  // ==== USER GROUP ====
  GetUserGroup: Types.GetUserGroupResponse;
  ListUserGroups: Types.ListUserGroupsResponse;

  // ==== SEARCH ====
  FindResources: Types.FindResourcesResponse;

  // ==== PROCEDURE ====
  GetProceduresSummary: Types.GetProceduresSummaryResponse;
  GetProcedure: Types.GetProcedureResponse;
  GetProcedureActionState: Types.GetProcedureActionStateResponse;
  ListProcedures: Types.ListProceduresResponse;
  ListFullProcedures: Types.ListFullProceduresResponse;

  // ==== SERVER TEMPLATE ====
  GetServerTemplate: Types.GetServerTemplateResponse;
  ListServerTemplates: Types.ListServerTemplatesResponse;
  ListFullServerTemplates: Types.ListFullServerTemplatesResponse;
  GetServerTemplatesSummary: Types.GetServerTemplatesSummaryResponse;

  // ==== SERVER ====
  GetServersSummary: Types.GetServersSummaryResponse;
  GetServer: Types.GetServerResponse;
  ListServers: Types.ListServersResponse;
  ListFullServers: Types.ListFullServersResponse;
  GetServerState: Types.GetServerStateResponse;
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
  ListFullDeployments: Types.ListFullDeploymentsResponse;
  GetDeploymentContainer: Types.GetDeploymentContainerResponse;
  GetDeploymentActionState: Types.GetDeploymentActionStateResponse;
  GetDeploymentStats: Types.GetDeploymentStatsResponse;
  GetLog: Types.GetLogResponse;
  SearchLog: Types.SearchLogResponse;
  ListCommonDeploymentExtraArgs: Types.ListCommonDeploymentExtraArgsResponse;

  // ==== BUILD ====
  GetBuildsSummary: Types.GetBuildsSummaryResponse;
  GetBuild: Types.GetBuildResponse;
  ListBuilds: Types.ListBuildsResponse;
  ListFullBuilds: Types.ListFullBuildsResponse;
  GetBuildActionState: Types.GetBuildActionStateResponse;
  GetBuildMonthlyStats: Types.GetBuildMonthlyStatsResponse;
  GetBuildVersions: Types.GetBuildVersionsResponse;
  ListCommonBuildExtraArgs: Types.ListCommonBuildExtraArgsResponse;
  ListGithubOrganizations: Types.ListGithubOrganizationsResponse;
  ListDockerOrganizations: Types.ListDockerOrganizationsResponse;

  // ==== REPO ====
  GetReposSummary: Types.GetReposSummaryResponse;
  GetRepo: Types.GetRepoResponse;
  ListRepos: Types.ListReposResponse;
  ListFullRepos: Types.ListFullReposResponse;
  GetRepoActionState: Types.GetRepoActionStateResponse;

  // ==== BUILDER ====
  GetBuildersSummary: Types.GetBuildersSummaryResponse;
  GetBuilder: Types.GetBuilderResponse;
  ListBuilders: Types.ListBuildersResponse;
  ListFullBuilders: Types.ListFullBuildersResponse;
  GetBuilderAvailableAccounts: Types.GetBuilderAvailableAccountsResponse;

  // ==== ALERTER ====
  GetAlertersSummary: Types.GetAlertersSummaryResponse;
  GetAlerter: Types.GetAlerterResponse;
  ListAlerters: Types.ListAlertersResponse;
  ListFullAlerters: Types.ListFullAlertersResponse;

  // ==== SYNC ====
  GetResourceSyncsSummary: Types.GetResourceSyncsSummaryResponse;
  GetResourceSync: Types.GetResourceSyncResponse;
  ListResourceSyncs: Types.ListResourceSyncsResponse;
  ListFullResourceSyncs: Types.ListFullResourceSyncsResponse;
  GetResourceSyncActionState: Types.GetResourceSyncActionStateResponse;

  // ==== TOML ====
  ExportAllResourcesToToml: Types.ExportAllResourcesToTomlResponse;
  ExportResourcesToToml: Types.ExportResourcesToTomlResponse;

  // ==== TAG ====
  GetTag: Types.GetTagResponse;
  ListTags: Types.ListTagsResponse;

  // ==== UPDATE ====
  GetUpdate: Types.GetUpdateResponse;
  ListUpdates: Types.ListUpdatesResponse;

  // ==== ALERT ====
  ListAlerts: Types.ListAlertsResponse;
  GetAlert: Types.GetAlertResponse;

  // ==== SERVER STATS ====
  GetSystemStats: Types.GetSystemStatsResponse;
  GetSystemProcesses: Types.GetSystemProcessesResponse;

  // ==== VARIABLE ====
  GetVariable: Types.GetVariableResponse;
  ListVariables: Types.ListVariablesResponse;
};

export type WriteResponses = {
  // ==== SERVICE USER ====
  CreateServiceUser: Types.CreateServiceUserResponse;
  UpdateServiceUserDescription: Types.UpdateServiceUserDescriptionResponse;
  CreateApiKeyForServiceUser: Types.CreateApiKeyForServiceUserResponse;
  DeleteApiKeyForServiceUser: Types.DeleteApiKeyForServiceUserResponse;

  // ==== USER GROUP ====
  CreateUserGroup: Types.UserGroup;
  RenameUserGroup: Types.UserGroup;
  DeleteUserGroup: Types.UserGroup;
  AddUserToUserGroup: Types.UserGroup;
  RemoveUserFromUserGroup: Types.UserGroup;
  SetUsersInUserGroup: Types.UserGroup;

  // ==== PERMISSIONS ====
  UpdateUserBasePermissions: Types.UpdateUserBasePermissionsResponse;
  UpdatePermissionOnTarget: Types.UpdatePermissionOnTargetResponse;

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

  // ==== SERVER TEMPLATE ====
  CreateServerTemplate: Types.ServerTemplate;
  CopyServerTemplate: Types.ServerTemplate;
  DeleteServerTemplate: Types.ServerTemplate;
  UpdateServerTemplate: Types.ServerTemplate;

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

  // ==== SYNC ====
  CreateResourceSync: Types.ResourceSync;
  CopyResourceSync: Types.ResourceSync;
  DeleteResourceSync: Types.ResourceSync;
  UpdateResourceSync: Types.ResourceSync;
  RefreshResourceSyncPending: Types.ResourceSync;

  // ==== TAG ====
  CreateTag: Types.Tag;
  DeleteTag: Types.Tag;
  RenameTag: Types.Tag;
  UpdateTagsOnResource: Types.UpdateTagsOnResourceResponse;

  // ==== VARIABLE ====
  CreateVariable: Types.CreateVariableResponse;
  UpdateVariableValue: Types.UpdateVariableValueResponse;
  UpdateVariableDescription: Types.UpdateVariableDescriptionResponse;
  DeleteVariable: Types.DeleteVariableResponse;
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
  CancelBuild: Types.CancelBuildResponse;

  // ==== REPO ====
  CloneRepo: Types.Update;
  PullRepo: Types.Update;

  // ==== PROCEDURE ====
  RunProcedure: Types.Update;

  // ==== SERVER TEMPLATE ====
  LaunchServer: Types.Update;

  // ==== SYNC ====
  RunSync: Types.Update;
};
