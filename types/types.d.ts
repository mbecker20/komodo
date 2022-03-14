export interface Collection<T> {
  [index: string]: T
}

export type BuildActionStates = Collection<BuildActionState>;
export type DeployActionStates = Collection<DeployActionState>;
export type Servers = Collection<Server>;
export type Builds = Collection<Build>;
export type Deployments = Collection<Deployment>;

export type User = {
  _id: string;
  username: string;
  permissions: number;
  password?: string;
  githubID?: string;
  avatar?: string;
};

export type Action = {
  type: string;
};

export type Log = {
  stdout?: string;
  stderr?: string;
};

export type Update = {
  _id: string;
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
  password: string;
  port: string;
  rootDirectory: string;
  status: "OK" | "Incorrect Password" | "Could Not Be Reached";
  enabled: boolean;
  useHTTP?: boolean;
};

export type Build = {
  _id?: string;
  name: string;
  /* repo related */
  repo?: string;
  branch?: string;
  accessToken?: string; // to gain access to private repos
  /* build related */
  buildPath?: string; // build folder relative to repo root
  dockerfilePath?: String; // relative to buildPath
  pullName?: string; // derived on build creation
  imageName?: string; // derived on build creation
  owner: string; // userID / username
};

export type Deployment = {
  _id?: string;
  name: string;
  owner: string;
  serverID: string;
  buildID?: string; // if deploying a monitor build
  /* to create docker run command */
  image?: string; // used if deploying an external image (from docker hub)
  latest?: boolean; // if custom image, use this to add :latest
  ports?: Conversion[];
  volumes?: Volume[];
  environment?: EnvironmentVar[];
  network?: string;
  restart?: string;
  postImage?: string; // interpolated into run command after the image string
  containerUser?: string; // after -u in the run command
  /* to manage repo for static frontend, mounted as a volume */
  repo?: string;
  accessToken?: string;
  pullName?: string; // for auto pull of repo
  containerMount?: string; // the file path to mount repo on inside the container
};

export type Conversion = {
  local: string;
  container: string;
};

export interface Volume extends Conversion {
  useSystemRoot?: boolean;
}

export type EnvironmentVar = {
  variable: string;
  value: string;
};

export type BuildActionState = {
  pulling: boolean;
  building: boolean;
};

export type DeployActionState = {
  deploying: boolean;
  deleting: boolean;
  starting: boolean;
  stopping: boolean;
};

export type ContainerStatus = {
  name: string;
  deploymentID?: string;
  Status: string;
  State: "running" | "exited";
};

export type EntityCollection = {
  _id?: string;
  name: string;
  deploymentIDs: string[];
  buildIDs: string[];
  owner: string; // userID
};