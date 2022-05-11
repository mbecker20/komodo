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
      if (stats.cpu > CPU_USAGE_NOTIFY_LIMIT) {
        // high cpu usage
        notifySlack(
          `WARNING | ${server.name} has high CPU usage.\n\nusage: ${stats.cpu}%`
        );
      }
      if (stats.mem.usedMemPercentage > MEM_USAGE_NOTIFY_LIMIT) {
        // high memory usage
        notifySlack(
          `WARNING | ${server.name} has high memory usage.\n\nusing ${stats.mem.usedMemMb} MB of ${stats.mem.totalMemMb} MB (${stats.mem.usedMemPercentage}%)`
        );
      }
      if (stats.disk.usedPercentage > DISK_USAGE_NOTIFY_LIMIT) {
        // high disk usage
        notifySlack(
          `WARNING | ${server.name} has high disk usage.\n\nusing ${stats.disk.usedGb} GB of ${stats.disk.totalGb} GB (${stats.disk.usedPercentage}%)`
        );
      }
    });
  };

  const dailyInterval = async () => {
    const servers = await getAllServerStats();
    const statsLog = servers.reduce((prev, curr) => {
      const stats = curr.stats!;
      return (
        prev +
        `${curr.name} | CPU: ${stats.cpu}% | MEM: ${stats.mem.usedMemPercentage}% (${stats.mem.usedMemMb} MB of ${stats.mem.totalMemMb} MB) | DISK: ${stats.disk.usedPercentage}% (${stats.disk.usedGb} GB of ${stats.disk.totalGb} GB)\n------------------------------------------------------------------\n\n`
      );
    }, "");
    const message = "INFO | daily update\n\n" + statsLog;
    notifySlack(message);
  };

  app.decorate("dailyInterval", dailyInterval);

  if (SECRETS.SLACK_TOKEN) {
    setInterval(interval, SERVER_STATS_INTERVAL);
    setInterval(dailyInterval, 24 * 60 * 60 * 1000);
  }

  done();
});

export default slackNotifier;
