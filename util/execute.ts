import { exec } from "child_process";
import { promisify } from "util";
import { CommandLogError } from "@monitor/types";

export const pExec = promisify(exec);

export async function execute(
  command: string
): Promise<CommandLogError> {
  try {
    return {
      command,
      log: await pExec(command),
      isError: false,
    };
  } catch (err) {
    return {
      command,
      log: { stderr: JSON.stringify(err) },
      isError: true,
    };
  }
}
