import { Server } from "@monitor/types";
import { timestamp } from "@monitor/util";
import { getSystemStats } from "@monitor/util-node";
import { FastifyInstance } from "fastify";
import fp from "fastify-plugin";
import { SERVER_STATS_INTERVAL } from "../config";
import { getPeripherySystemStats } from "../util/periphery/server";
import { serverStatusPeriphery } from "../util/periphery/status";

declare module "fastify" {
  interface FastifyInstance {
    statsIntervals: {
      intervals: () => { [serverID: string]: NodeJS.Timer };
      add: (server: Server) => void;
      remove: (serverID: string) => void;
      update: (serverID: string, interval: number) => void;
    };
  }
}

const collectStats = fp((app: FastifyInstance, _: {}, done: () => void) => {
  let intervals: { [serverID: string]: NodeJS.Timer } = {};

  const storeStats = async (serverID: string) => {
    const server = await app.servers.findById(serverID);
    if (!server || !server.enabled) return;
    if (server.isCore) {
      const stats = await getSystemStats();
      app.stats.create({
        ...stats,
        serverID,
        ts: timestamp(),
      });
      app.checkServerToNotify(server, stats);
    } else {
      const reachable = await serverStatusPeriphery(server);
      if (reachable) {
        const stats = await getPeripherySystemStats(server);
        app.stats.create({
          ...stats,
          serverID,
          ts: timestamp(),
        });
        app.checkServerToNotify(server, stats);
      } else {
        app.notifyServerUnreachable(server);
      }
    }
  };

  app.decorate("statsIntervals", {
    intervals: () => intervals,
    add: (server: Server) => {
      intervals[server._id!] = setInterval(
        () => storeStats(server._id!),
        server.statsInterval || SERVER_STATS_INTERVAL
      );
    },
    remove: (serverID: string) => {
      if (intervals[serverID]) clearInterval(intervals[serverID]);
      delete intervals[serverID];
    },
    update: (serverID: string, interval: number) => {
      if (intervals[serverID]) clearInterval(intervals[serverID]);
      intervals[serverID] = setInterval(() => storeStats(serverID), interval);
      app.servers.updateById(serverID, { $set: { statsInterval: interval } });
    },
  });

  app.after(async () => {
    const servers = await app.servers.find({});
    for (const server of servers) {
      app.statsIntervals.add(server);
    }
  });

  done();
});

export default collectStats;
