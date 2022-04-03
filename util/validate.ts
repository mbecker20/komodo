import { Command } from "@monitor/types";

export function isValidCommand({ command }: Command) {
	return !command.includes("&&");
}