import {
  Command as CommandType,
  Conversion as ConversionType,
  EnvironmentVar as EnvironmentVarType,
  Log as LogType,
  DockerBuildArgs as DockerBuildArgsType,
} from "@monitor/types";
import { Schema } from "mongoose";

export const Command = new Schema<CommandType>({
  name: String,
  path: String,
  command: String,
});

export const DockerBuildArgs = new Schema<DockerBuildArgsType>({
  buildPath: String,
  dockerfilePath: String,
  imageName: String,
});

export const Conversion = new Schema<ConversionType>({
	local: String,
  container: String,
});

export const EnvironmentVar = new Schema<EnvironmentVarType>({
	variable: String,
  value: String,
});

export const Log = new Schema<LogType>({
	stdout: String,
	stderr: String,
});