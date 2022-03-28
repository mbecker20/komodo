import {
  Conversion as ConversionType,
  EnvironmentVar as EnvironmentVarType,
  Log as LogType,
  Volume as VolumeType,
} from "@monitor/types";
import { Schema } from "mongoose";

export const Conversion = new Schema<ConversionType>({
	local: String,
  container: String,
});

export const Volume = new Schema<VolumeType>({
	local: String,
  container: String,
	useSystemRoot: Boolean,
});

export const EnvironmentVar = new Schema<EnvironmentVarType>({
	variable: String,
  value: String,
});

export const Log = new Schema<LogType>({
	stdout: String,
	stderr: String,
});