export interface Collection<T> {
  [index: string]: T;
}

export type BuildActionStates = Collection<BuildActionState>;
export type DeployActionStates = Collection<DeployActionState>;
export type ServerActionStates = Collection<ServerActionState>;
export type Servers = Collection<Server>;
export type Builds = Collection<Build>;
export type Deployments = Collection<Deployment>;

export type User = {
  _id?: string;
  username: string;
  permissions?: number;
  password?: string;
  githubID?: number;
  avatar?: string;
};

export type Action = {
  type: string;
  userID: string;
};

export type Log = {
  stdout?: string;
  stderr?: string;
};

export type Update = {
  _id?: string;
  buildID?: string;
  deploymentID?: string;
  serverID?: string;
  operation: string;
  command: string;
  log: Log;
  timestamp: number;
  note: string;
  isError?: boolean;
  operator: string; // the userID or username
};

export type Server = {
  _id?: string;
  name: string;
  address: string;
  status?: "OK" | "Incorrect Password" | "Could Not Be Reached";
  enabled: boolean;
  isCore?: boolean;
};

export type DockerBuildArgs = {
  buildPath: string; // build folder relative to repo root
  dockerfilePath?: string; // relative to buildPath
  account?: string;
};

// these are potentially dangerous but also useful
// maybe best for admins to add premade curated command strings, so user dev can't input them directly, only give path to run in.
export type Command = {
  name?: string;
  path?: string;
  command: string; // no cd and no sudo
};

export interface Build {
  _id?: string;
  name: string;
  pullName?: string; // used for git listener
  commands?: Command[];
  /* repo related */
  repo?: string;
  subfolder?: string; // subfolder of monorepo. uses sparse clone
  branch?: string;
  githubAccount?: string; // to gain access to private repos
  onClone?: Command;
  /* build related */
  cliBuild?: Command; // run shell commands on build, before docker build step if it exists
  dockerBuildArgs?: DockerBuildArgs; // provided if docker build
  dockerAccount?: string; // username
  owners: string[]; // userID / username
}

export type DeploymentGitConfig = {
  repo?: string;
  branch?: string;
  subfolder?: string; // subfolder of repo to clone (uses sparse clone)
  githubAccount?: string;
  repoMount?: string; // subfolder of repo to mount in container
  containerMount?: string; // the file path to mount repo on inside the container
  onPull?: Command;
};

export type DockerRunArgs = {
  image?: string;
  containerName?: string; // also for auto pull of repo, will be set by time deployment created
  ports?: Conversion[];
  volumes?: Conversion[];
  environment?: EnvironmentVar[];
  network?: string;
  restart?: string;
  postImage?: string; // interpolated into run command after the image string
  containerUser?: string; // after -u in the run command
  dockerAccount?: string;
};

export interface Deployment extends DockerRunArgs {
  _id?: string;
  name: string;
  owners: string[];
  serverID?: string; // only added if running on periphery server
  buildID?: string; // if deploying a monitor build
  /* to manage repo for static frontend, mounted as a volume. locally in REPO_ROOT/containerName */
  repo?: string;
  branch?: string;
  subfolder?: string; // subfolder of repo to clone (uses sparse clone)
  githubAccount?: string;
  repoMount?: string; // subfolder of repo to mount in container
  containerMount?: string; // the file path to mount repo on inside the container
  onPull?: Command;
  onClone?: Command;
  gitConfigs?: DeploymentGitConfig[];
  // running status
  status?: "not deployed" | ContainerStatus;
}

export type Conversion = {
  local: string;
  container: string;
};

export type EnvironmentVar = {
  variable: string;
  value: string;
};

export type BuildActionState = {
  pulling: boolean;
  building: boolean;
  cloning: boolean;
  updating: boolean;
  deleting: boolean;
};

export type DeployActionState = {
  deploying: boolean;
  deleting: boolean;
  starting: boolean;
  stopping: boolean;
  updating: boolean;
  fullDeleting: boolean;
  pulling: boolean;
};

export type ServerActionState = {
  pruningImages: boolean;
  pruningNetworks: boolean;
  deleting: boolean;
};

export type ContainerStatus = {
  name: string;
  Status: string;
  State: "created" | "running" | "exited";
};

export type Network = {
  // _id: string;
  name: string;
  driver: string;
};

export type EntityCollection = {
  _id?: string;
  name: string;
  deploymentIDs: string[];
  buildIDs: string[];
  owners: string[]; // userID
};

export type CommandLogError = {
  command: string;
  log: Log;
  isError: boolean;
};

export type PeripherySecrets = {
  PASSKEY: string;
  DOCKER_ACCOUNTS: {
    [
      username: string
    ]: string /* this is the password for the account (they don't use auth tokens) */;
  };
  GITHUB_ACCOUNTS: {
    [
      username: string
    ]: string /* this is a personal access token for the acct */;
  };
};

export type CoreSecrets = PeripherySecrets & {
  JWT: { SECRET: string };
  GITHUB_OAUTH: { ID: string; SECRET: string };
};
