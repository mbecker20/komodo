import { remove } from "fs-extra";
import { execute } from "./execute";

async function fullClone(
  repo: string,
  folder: string,
  branch?: string,
  accessToken?: string
) {
  const _clone = "git clone";
  const url = `https://${
    accessToken ? `${accessToken}@` : ""
  }github.com/${repo}.git`;
  const _branch = ` -b ${branch || "main"}`;
  const clone = `${_clone} ${url} ${folder}${_branch}`;
  return await execute(
    clone,
    accessToken && clone.replace(accessToken, "<TOKEN>")
  );
}

async function sparseClone(
  repo: string,
  folder: string,
  subfolder: string,
  branch?: string,
  accessToken?: string
) {
  const _clone = "git clone --no-checkout";
  const url = `https://${
    accessToken ? `${accessToken}@` : ""
  }github.com/${repo}.git`;
  const _branch = ` -b ${branch || "main"}`;
  const sparseCheckout = `cd ${folder} && git sparse-checkout init --cone && git sparse-checkout set ${subfolder}`;
  const command = `${_clone} ${url} ${folder}${_branch} && ${sparseCheckout}`;
  return await execute(
    command,
    accessToken && command.replace(accessToken, "<TOKEN>")
  );
}

export async function clone(
  repo: string,
  folder: string,
  subfolder?: string,
  branch?: string,
  accessToken?: string
) {
  await remove(folder).catch();
  if (subfolder) {
    return await sparseClone(repo, folder, subfolder, branch, accessToken);
  } else {
    return await fullClone(repo, folder, branch, accessToken);
  }
}

export async function pull(folder: string, branch = "master") {
  const command = `cd ${folder} && git pull origin ${branch}`;
  return await execute(command);
}
