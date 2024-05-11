import { Types } from "@monitor/client";

export type ColorIntention =
  | "Good"
  | "Neutral"
  | "Warning"
  | "Critical"
  | "Unknown"
  | "None";

export const hex_color_by_intention = (intention: ColorIntention) => {
  switch (intention) {
    case "Good":
      return "#22C55E";
    case "Neutral":
      return "#3B82F6";
    case "Warning":
      return "#F97316";
    case "Critical":
      return "#EF0044";
    case "Unknown":
      return "#A855F7";
    case "None":
      return "";
  }
};

export const color_class_by_intention = (intention: ColorIntention) => {
  switch (intention) {
    case "Good":
      return "green-500";
    case "Neutral":
      return "blue-500";
    case "Warning":
      return "orange-500";
    case "Critical":
      return "red-500";
    case "Unknown":
      return "purple-500";
    case "None":
      return "";
  }
};

export const fill_color_class_by_intention = (intention: ColorIntention) => {
  if (intention === "None") return "";
  return `fill-${color_class_by_intention(intention)}`;
};

export const stroke_color_class_by_intention = (intention: ColorIntention) => {
  if (intention === "None") return "";
  return `stroke-${color_class_by_intention(intention)}`;
};

export const bg_color_class_by_intention = (intention: ColorIntention) => {
  switch (intention) {
    case "Good":
      return "bg-green-400 dark:bg-green-700";
    case "Neutral":
      return "bg-blue-400 dark:bg-blue-700";
    case "Warning":
      return "bg-orange-400 dark:bg-orange-700";
    case "Critical":
      return "bg-red-400 dark:bg-red-700";
    case "Unknown":
      return "bg-purple-400 dark:bg-purple-700";
    case "None":
      return "";
  }
};

export const text_color_class_by_intention = (intention: ColorIntention) => {
  switch (intention) {
    case "Good":
      return "text-green-500";
    case "Neutral":
      return "text-blue-500";
    case "Warning":
      return "text-orange-500";
    case "Critical":
      return "text-red-500";
    case "Unknown":
      return "text-purple-500";
    case "None":
      return "";
  }
};

export const server_status_intention: (
  status?: Types.ServerStatus
) => ColorIntention = (status) => {
  switch (status) {
    case Types.ServerStatus.Ok:
      return "Good";
    case Types.ServerStatus.NotOk:
      return "Critical";
    case Types.ServerStatus.Disabled:
      return "Neutral";
    case undefined:
      return "None";
  }
};

export const deployment_state_intention: (
  state?: Types.DockerContainerState
) => ColorIntention = (state) => {
  switch (state) {
    case undefined:
      return "None";
    case Types.DockerContainerState.Running:
      return "Good";
    case Types.DockerContainerState.NotDeployed:
      return "Neutral";
    case Types.DockerContainerState.Unknown:
      return "Unknown";
    default:
      return "Critical";
  }
};

export const build_status_intention = (status?: Types.BuildStatus) => {
  switch (status) {
    case undefined:
      return "None";
    case Types.BuildStatus.Unknown:
      return "Unknown";
    case Types.BuildStatus.Ok:
      return "Good";
    case Types.BuildStatus.Building:
      return "Warning";
    case Types.BuildStatus.Failed:
      return "Critical";
    default:
      return "None";
  }
}

export const repo_status_intention = (status?: Types.RepoStatus) => {
  switch (status) {
    case undefined:
      return "None";
    case Types.RepoStatus.Unknown:
      return "Unknown";
    case Types.RepoStatus.Ok:
      return "Good";
    case Types.RepoStatus.Cloning:
      return "Warning";
    case Types.RepoStatus.Pulling:
      return "Warning";
    case Types.RepoStatus.Failed:
      return "Critical";
    default:
      return "None";
  }
};

export const alert_level_intention: (
  level: Types.SeverityLevel
) => ColorIntention = (level) => {
  switch (level) {
    case Types.SeverityLevel.Ok: return "Good"
    case Types.SeverityLevel.Warning: return "Warning"
    case Types.SeverityLevel.Critical: return "Critical"
  }
}