import { exec } from "child_process";
import { promisify } from "util";
import { Log } from "@monitor/types";

export const pExec = promisify(exec);

export async function execute(
  command: string
): Promise<{ log: Log; success: boolean }> {
  try {
    return {
      log: await pExec(command),
      success: true,
    };
  } catch (err) {
    return {
      log: { stderr: JSON.stringify(err) },
      success: false,
    };
  }
}
