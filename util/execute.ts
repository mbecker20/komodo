import { exec } from "child_process";
import { promisify } from "util";
import { Log } from "@monitor/types";

export const pExec = promisify(exec);

export async function execute(
  command: string
): Promise<{ log: Log; isError: boolean }> {
  try {
    return {
      log: await pExec(command),
      isError: false,
    };
  } catch (err) {
    return {
      log: { stderr: JSON.stringify(err) },
      isError: true,
    };
  }
}
