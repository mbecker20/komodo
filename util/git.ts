import { remove } from "fs-extra";
import { execute } from "./execute";

export async function clone(
  repo: string,
  folder: string,
  branch?: string,
  accessToken?: string
) {
  await remove(folder).catch();
  const cloneForLog = `git clone https://<TOKEN>@github.com/${repo}.git ${folder}${
    branch && branch !== "master" ? ` -b ${branch}` : ""
  }`;
  const clone = `git clone https://${
    accessToken && `${accessToken}@`
  }github.com/${repo}.git ${folder}${
    branch && branch !== "master" ? ` -b ${branch}` : ""
  }`;
  return await execute(clone, cloneForLog);
}

export async function pull(folder: string, branch = "master") {
  const command = `cd ${folder} && git pull origin ${branch}`;
  return await execute(command);
}
