import { Schema } from "mongoose";

export const Conversion = new Schema({
	local: String,
  container: String,
});

export const Volume = new Schema({
	variable: String,
  value: String,
});

export const EnvironmentVar = new Schema({
	variable: String,
  value: String,
});

export const Log = new Schema({
	stdout: String,
	stderr: String,
});