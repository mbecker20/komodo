import { getSystemStats } from "@monitor/util-node";
import { FastifyInstance } from "fastify";
import fp from "fastify-plugin";
import {
  CPU_USAGE_NOTIFY_LIMIT,
  DISK_USAGE_NOTIFY_LIMIT,
  MEM_USAGE_NOTIFY_LIMIT,
  SERVER_STATS_INTERVAL,
  SLACK_TOKEN,
} from "../config";
import { getPeripherySystemStats } from "../util/periphery/server";
import { serverStatusPeriphery } from "../util/periphery/status";
import { notifySlack } from "../util/slack";

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
        notifySlack(`
					WARNING | server ${server.name} has high CPU usage.

					usage: ${stats.cpu}%
				`);
      }
      if (stats.mem.usedMemPercentage > MEM_USAGE_NOTIFY_LIMIT) {
        // high memory usage
        notifySlack(`
					WARNING | server ${server.name} has high memory usage.

					using ${stats.mem.usedMemMb} MB of ${stats.mem.totalMemMb} MB (${stats.mem.usedMemPercentage}%)
				`);
      }
      if (stats.disk.usedPercentage > DISK_USAGE_NOTIFY_LIMIT) {
        // high disk usage
        notifySlack(`
					WARNING | server ${server.name} has high disk usage.

					using ${stats.disk.usedGb} GB of ${stats.disk.totalGb} (${stats.disk.usedPercentage}% full)
				`);
      }
    });
  };

  if (SLACK_TOKEN) {
    // only do this if slack token is provided
    setInterval(interval, SERVER_STATS_INTERVAL);
  }

  done();
});

export default slackNotifier;
