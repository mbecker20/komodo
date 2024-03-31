import { Types } from "@monitor/client";

export type ColorIntention =
  | "Good"
  | "Neutral"
  | "Warning"
  | "Critical"
  | "Unknown";

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
  }
};

export const text_color_class_by_intention = (intention: ColorIntention) => {
  return `text-${color_class_by_intention(intention)}`;
};

export const fill_color_class_by_intention = (intention: ColorIntention) => {
  return `fill-${color_class_by_intention(intention)}`;
};

export const stroke_color_class_by_intention = (intention: ColorIntention) => {
  return `stroke-${color_class_by_intention(intention)}`;
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
    default:
      return "Unknown";
  }
};

export const deployment_state_intention: (
  state?: Types.DockerContainerState
) => ColorIntention = (state) => {
  switch (state) {
    case Types.DockerContainerState.Running:
      return "Good";
    case Types.DockerContainerState.NotDeployed:
      return "Neutral";
    case Types.DockerContainerState.Unknown || undefined:
      return "Unknown";
    default:
      return "Critical";
  }
};
