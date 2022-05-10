import { cpu, drive, mem } from "node-os-utils";
import { convertFieldsToNumbers } from "@monitor/util";
import { DiskStats, MemStats, SystemStats } from "@monitor/types";

export async function getCpuUsage() {
  return await cpu.usage();
}

export async function getMemoryUsage() {
  return await mem.info();
}

export async function getDriveUsage() {
  return convertFieldsToNumbers(await drive.info("/"));
}

export async function getSystemStats(): Promise<SystemStats> {
  const [cpu, disk, mem] = await Promise.all([
    getCpuUsage(),
    getDriveUsage(),
    getMemoryUsage(),
  ]);
  return {
    cpu,
    disk: disk as DiskStats,
    mem: mem as MemStats,
  };
}
