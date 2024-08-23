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
  ListAwsEcrLabels: Types.ListAwsEcrLabelsResponse;
  ListSecrets: Types.ListSecretsResponse;
  ListGitProvidersFromConfig: Types.ListGitProvidersFromConfigResponse;
  ListDockerRegistriesFromConfig: Types.ListDockerRegistriesFromConfigResponse;

  // ==== USER ====
  GetUsername: Types.GetUsernameResponse;
  GetPermissionLevel: Types.GetPermissionLevelResponse;
  FindUser: Types.FindUserResponse;
  ListUsers: Types.ListUsersResponse;
  ListApiKeys: Types.ListApiKeysResponse;
  ListApiKeysForServiceUser: Types.ListApiKeysForServiceUserResponse;
  ListPermissions: Types.ListPermissionsResponse;
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
  GetServerTemplatesSummary: Types.GetServerTemplatesSummaryResponse;
  ListServerTemplates: Types.ListServerTemplatesResponse;
  ListFullServerTemplates: Types.ListFullServerTemplatesResponse;

  // ==== SERVER ====
  GetServersSummary: Types.GetServersSummaryResponse;
  GetServer: Types.GetServerResponse;
  GetServerState: Types.GetServerStateResponse;
  GetPeripheryVersion: Types.GetPeripheryVersionResponse;
  ListDockerContainers: Types.ListDockerContainersResponse;
  InspectDockerContainer: Types.InspectDockerContainerResponse;
  GetContainerLog: Types.GetContainerLogResponse;
  SearchContainerLog: Types.SearchContainerLogResponse;
  ListDockerNetworks: Types.ListDockerNetworksResponse;
  InspectDockerNetwork: Types.InspectDockerNetworkResponse;
  ListDockerImages: Types.ListDockerImagesResponse;
  InspectDockerImage: Types.InspectDockerImageResponse;
  ListDockerImageHistory: Types.ListDockerImageHistoryResponse;
  ListDockerVolumes: Types.ListDockerVolumesResponse;
  InspectDockerVolume: Types.InspectDockerVolumeResponse;
  ListComposeProjects: Types.ListComposeProjectsResponse;
  GetServerActionState: Types.GetServerActionStateResponse;
  GetHistoricalServerStats: Types.GetHistoricalServerStatsResponse;
  ListServers: Types.ListServersResponse;
  ListFullServers: Types.ListFullServersResponse;

  // ==== DEPLOYMENT ====
  GetDeploymentsSummary: Types.GetDeploymentsSummaryResponse;
  GetDeployment: Types.GetDeploymentResponse;
  GetDeploymentContainer: Types.GetDeploymentContainerResponse;
  GetDeploymentActionState: Types.GetDeploymentActionStateResponse;
  GetDeploymentStats: Types.GetDeploymentStatsResponse;
  GetDeploymentLog: Types.GetDeploymentLogResponse;
  SearchDeploymentLog: Types.SearchDeploymentLogResponse;
  ListDeployments: Types.ListDeploymentsResponse;
  ListFullDeployments: Types.ListFullDeploymentsResponse;
  ListCommonDeploymentExtraArgs: Types.ListCommonDeploymentExtraArgsResponse;

  // ==== BUILD ====
  GetBuildsSummary: Types.GetBuildsSummaryResponse;
  GetBuild: Types.GetBuildResponse;
  GetBuildActionState: Types.GetBuildActionStateResponse;
  GetBuildMonthlyStats: Types.GetBuildMonthlyStatsResponse;
  GetBuildWebhookEnabled: Types.GetBuildWebhookEnabledResponse;
  ListBuilds: Types.ListBuildsResponse;
  ListFullBuilds: Types.ListFullBuildsResponse;
  ListBuildVersions: Types.ListBuildVersionsResponse;
  ListCommonBuildExtraArgs: Types.ListCommonBuildExtraArgsResponse;

  // ==== REPO ====
  GetReposSummary: Types.GetReposSummaryResponse;
  GetRepo: Types.GetRepoResponse;
  GetRepoActionState: Types.GetRepoActionStateResponse;
  GetRepoWebhooksEnabled: Types.GetRepoWebhooksEnabledResponse;
  ListRepos: Types.ListReposResponse;
  ListFullRepos: Types.ListFullReposResponse;

  // ==== SYNC ====
  GetResourceSyncsSummary: Types.GetResourceSyncsSummaryResponse;
  GetResourceSync: Types.GetResourceSyncResponse;
  GetResourceSyncActionState: Types.GetResourceSyncActionStateResponse;
  GetSyncWebhooksEnabled: Types.GetSyncWebhooksEnabledResponse;
  ListResourceSyncs: Types.ListResourceSyncsResponse;
  ListFullResourceSyncs: Types.ListFullResourceSyncsResponse;

  // ==== STACK ====
  GetStacksSummary: Types.GetStacksSummaryResponse;
  GetStack: Types.GetStackResponse;
  GetStackActionState: Types.GetStackActionStateResponse;
  GetStackWebhooksEnabled: Types.GetStackWebhooksEnabledResponse;
  GetStackServiceLog: Types.GetStackServiceLogResponse;
  SearchStackServiceLog: Types.SearchStackServiceLogResponse;
  ListStacks: Types.ListStacksResponse;
  ListFullStacks: Types.ListFullStacksResponse;
  ListStackServices: Types.ListStackServicesResponse;
  ListCommonStackExtraArgs: Types.ListCommonStackExtraArgsResponse;

  // ==== BUILDER ====
  GetBuildersSummary: Types.GetBuildersSummaryResponse;
  GetBuilder: Types.GetBuilderResponse;
  ListBuilders: Types.ListBuildersResponse;
  ListFullBuilders: Types.ListFullBuildersResponse;

  // ==== ALERTER ====
  GetAlertersSummary: Types.GetAlertersSummaryResponse;
  GetAlerter: Types.GetAlerterResponse;
  ListAlerters: Types.ListAlertersResponse;
  ListFullAlerters: Types.ListFullAlertersResponse;

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
  GetSystemInformation: Types.GetSystemInformationResponse;
  GetSystemStats: Types.GetSystemStatsResponse;
  ListSystemProcesses: Types.ListSystemProcessesResponse;

  // ==== VARIABLE ====
  GetVariable: Types.GetVariableResponse;
  ListVariables: Types.ListVariablesResponse;

  // ==== PROVIDER ====
  GetGitProviderAccount: Types.GetGitProviderAccountResponse;
  ListGitProviderAccounts: Types.ListGitProviderAccountsResponse;
  GetDockerRegistryAccount: Types.GetDockerRegistryAccountResponse;
  ListDockerRegistryAccounts: Types.ListDockerRegistryAccountsResponse;
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
  UpdatePermissionOnResourceType: Types.UpdatePermissionOnResourceTypeResponse;
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
  RefreshBuildCache: Types.NoData;
  CreateBuildWebhook: Types.CreateBuildWebhookResponse;
  DeleteBuildWebhook: Types.DeleteBuildWebhookResponse;

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
  RefreshRepoCache: Types.NoData;
  CreateRepoWebhook: Types.CreateRepoWebhookResponse;
  DeleteRepoWebhook: Types.DeleteRepoWebhookResponse;

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
  CreateSyncWebhook: Types.CreateSyncWebhookResponse;
  DeleteSyncWebhook: Types.DeleteSyncWebhookResponse;

  // ==== STACK ====
  CreateStack: Types.Stack;
  CopyStack: Types.Stack;
  DeleteStack: Types.Stack;
  UpdateStack: Types.Stack;
  RenameStack: Types.Update;
  RefreshStackCache: Types.NoData;
  CreateStackWebhook: Types.CreateStackWebhookResponse;
  DeleteStackWebhook: Types.DeleteStackWebhookResponse;

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

  // ==== PROVIDERS ====
  CreateGitProviderAccount: Types.CreateGitProviderAccountResponse;
  UpdateGitProviderAccount: Types.UpdateGitProviderAccountResponse;
  DeleteGitProviderAccount: Types.DeleteGitProviderAccountResponse;
  CreateDockerRegistryAccount: Types.CreateDockerRegistryAccountResponse;
  UpdateDockerRegistryAccount: Types.UpdateDockerRegistryAccountResponse;
  DeleteDockerRegistryAccount: Types.DeleteDockerRegistryAccountResponse;
};

export type ExecuteResponses = {
  // ==== SERVER ====
  StartContainer: Types.Update;
  RestartContainer: Types.Update;
  PauseContainer: Types.Update;
  UnpauseContainer: Types.Update;
  StopContainer: Types.Update;
  DestroyContainer: Types.Update;
  StopAllContainers: Types.Update;
  PruneContainers: Types.Update;
  PruneNetworks: Types.Update;
  PruneImages: Types.Update;
  PruneVolumes: Types.Update;
  PruneSystem: Types.Update;

  // ==== DEPLOYMENT ====
  Deploy: Types.Update;
  StartDeployment: Types.Update;
  RestartDeployment: Types.Update;
  PauseDeployment: Types.Update;
  UnpauseDeployment: Types.Update;
  StopDeployment: Types.Update;
  DestroyDeployment: Types.Update;

  // ==== BUILD ====
  RunBuild: Types.Update;
  CancelBuild: Types.Update;

  // ==== REPO ====
  CloneRepo: Types.Update;
  PullRepo: Types.Update;
  BuildRepo: Types.Update;
  CancelRepoBuild: Types.Update;

  // ==== PROCEDURE ====
  RunProcedure: Types.Update;

  // ==== SERVER TEMPLATE ====
  LaunchServer: Types.Update;

  // ==== SYNC ====
  RunSync: Types.Update;

  // ==== STACK ====
  DeployStack: Types.Update;
  StartStack: Types.Update;
  RestartStack: Types.Update;
  StopStack: Types.Update;
  PauseStack: Types.Update;
  UnpauseStack: Types.Update;
  DestroyStack: Types.Update;

  // ==== STACK Service ====
  DeployStackService: Types.Update;
  StartStackService: Types.Update;
  RestartStackService: Types.Update;
  StopStackService: Types.Update;
  PauseStackService: Types.Update;
  UnpauseStackService: Types.Update;
  DestroyStackService: Types.Update;
};
