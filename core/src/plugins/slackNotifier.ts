import { getSystemStats } from "@monitor/util-node";
import { FastifyInstance } from "fastify";
import fp from "fastify-plugin";
import {
  CPU_USAGE_NOTIFY_LIMIT,
  DISK_USAGE_NOTIFY_LIMIT,
  MEM_USAGE_NOTIFY_LIMIT,
  SERVER_STATS_INTERVAL,
  SECRETS,
} from "../config";
import { getPeripherySystemStats } from "../util/periphery/server";
import { serverStatusPeriphery } from "../util/periphery/status";
import { notifySlack } from "../util/slack";

declare module "fastify" {
  interface FastifyInstance {
    dailyInterval: () => Promise<void>;
  }
}

const DIVIDER =
  "-----------------------------------------------------------------------------";

let alreadyAlerted: {
  [serverID: string]: { cpu: boolean; mem: boolean; disk: boolean };
} = {};

const slackNotifier = fp((app: FastifyInstance, _: {}, done: () => void) => {
  const getAllServerStats = async () => {
    const servers = await app.servers.find({});
    const serversWithStatus = (
      await Promise.all(
        servers
          .filter((server) => server.enabled)
          .map(async (server) => {
            const status = await serverStatusPeriphery(server);
            return {
              ...server,
              stats: server.isCore
                ? await getSystemStats()
                : status
                ? await getPeripherySystemStats(server)
                : undefined,
            };
          })
      )
    ).filter((server) => server.stats);
    return serversWithStatus;
  };

  const interval = async () => {
    const servers = await getAllServerStats();
    servers.forEach((server) => {
      // check for out of bounds stats
      const stats = server.stats!;
      if (stats.cpu > (server.cpuAlert || CPU_USAGE_NOTIFY_LIMIT)) {
        // high cpu usage
        if (!alreadyAlerted[server._id!] || !alreadyAlerted[server._id!].cpu) {
          notifySlack(
            `WARNING | ${server.name}${
              server.region ? ` (${server.region})` : ""
            } has high CPU usage.\n\nusage: ${
              stats.cpu
            }%\n\n${server.toNotify.reduce(
              (prev, curr) => (prev ? " <@" + curr + ">" : "<@" + curr + ">"),
              ""
            )}`
          );
          if (alreadyAlerted[server._id!]) {
            alreadyAlerted[server._id!] = {
              ...alreadyAlerted[server._id!],
              cpu: true,
            };
          } else {
            alreadyAlerted[server._id!] = {
              cpu: true,
              mem: false,
              disk: false,
            };
          }
        }
      }
      if (
        stats.mem.usedMemPercentage >
        (server.memAlert || MEM_USAGE_NOTIFY_LIMIT)
      ) {
        // high memory usage
        if (!alreadyAlerted[server._id!] || !alreadyAlerted[server._id!].mem) {
          notifySlack(
            `WARNING | ${server.name}${
              server.region ? ` (${server.region})` : ""
            } has high memory usage.\n\nusing ${stats.mem.usedMemMb} MB of ${
              stats.mem.totalMemMb
            } MB (${stats.mem.usedMemPercentage}%)\n\n${server.toNotify.reduce(
              (prev, curr) => (prev ? " <@" + curr + ">" : "<@" + curr + ">"),
              ""
            )}`
          );
          if (alreadyAlerted[server._id!]) {
            alreadyAlerted[server._id!] = {
              ...alreadyAlerted[server._id!],
              mem: true,
            };
          } else {
            alreadyAlerted[server._id!] = {
              cpu: false,
              mem: true,
              disk: false,
            };
          }
        }
      }
      if (
        stats.disk.usedPercentage >
        (server.diskAlert || DISK_USAGE_NOTIFY_LIMIT)
      ) {
        // high disk usage
        if (!alreadyAlerted[server._id!] || !alreadyAlerted[server._id!].disk) {
          notifySlack(
            `WARNING | ${server.name}${
              server.region ? ` (${server.region})` : ""
            } has high disk usage.\n\nusing ${stats.disk.usedGb} GB of ${
              stats.disk.totalGb
            } GB (${stats.disk.usedPercentage}%)\n\n${server.toNotify.reduce(
              (prev, curr) => (prev ? " <@" + curr + ">" : "<@" + curr + ">"),
              ""
            )}`
          );
          if (alreadyAlerted[server._id!]) {
            alreadyAlerted[server._id!] = {
              ...alreadyAlerted[server._id!],
              disk: true,
            };
          } else {
            alreadyAlerted[server._id!] = {
              cpu: false,
              mem: false,
              disk: true,
            };
          }
        }
      }
    });
  };

  const dailyInterval = async () => {
    const servers = await getAllServerStats();
    const statsLog = servers.reduce((prev, curr) => {
      const stats = curr.stats!;
      return (
        prev +
        `${curr.name}${curr.region ? ` | ${curr.region}` : ""} | CPU: ${
          stats.cpu
        }% | MEM: ${stats.mem.usedMemPercentage}% (${
          stats.mem.usedMemMb
        } MB of ${stats.mem.totalMemMb} MB) | DISK: ${
          stats.disk.usedPercentage
        }% (${stats.disk.usedGb} GB of ${
          stats.disk.totalGb
        } GB)\n${DIVIDER}\n\n`
      );
    }, "");
    const message = "INFO | daily update\n\n" + DIVIDER + "\n\n" + statsLog;
    notifySlack(message);
    alreadyAlerted = {};
  };

  app.decorate("dailyInterval", dailyInterval);

  if (SECRETS.SLACK_TOKEN) {
    setInterval(interval, SERVER_STATS_INTERVAL);
    setInterval(dailyInterval, 24 * 60 * 60 * 1000);
  }

  done();
});

export default slackNotifier;
