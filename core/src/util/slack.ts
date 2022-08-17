import { readableTimestamp, timestamp } from "@monitor/util";
import { WebClient, LogLevel, Block, KnownBlock } from "@slack/web-api";
import { SLACK_CHANNEL, SECRETS } from "../config";

const slack = new WebClient(SECRETS.SLACK_TOKEN, { logLevel: LogLevel.INFO });

export async function notifySlack(text: string) {
  try {
    await slack.chat.postMessage({
      token: SECRETS.SLACK_TOKEN,
      channel: SLACK_CHANNEL,
      text,
    });
  } catch (error) {
    console.log("POST TO SLACK FAILED @", readableTimestamp(timestamp()));
    console.log(error);
  }
}

export async function notifySlackAdvanced(
  text: string,
  blocks: (Block | KnownBlock | undefined)[]
) {
  try {
    await slack.chat.postMessage({
      token: SECRETS.SLACK_TOKEN,
      channel: SLACK_CHANNEL,
      text,
      blocks: blocks.filter(e => e) as (Block | KnownBlock)[],
    });
  } catch (error) {
    console.log("POST TO SLACK FAILED @", readableTimestamp(timestamp()));
    console.log(error);
  }
}

const HEADER = {
  type: "header",
  text: {
    type: "plain_text",
    text: "WARNING 🚨",
  },
};

export async function notifySlackCpu(
  name: string,
  region: string | undefined,
  usage: number,
  toNotify?: string[]
) {
  await notifySlackAdvanced(
    `WARNING | ${name}${region ? ` (${region})` : ""} has high CPU usage 📈 🚨`,
    [
      HEADER,
      {
        type: "section",
        text: {
          type: "mrkdwn",
          text: `*${name}*${
            region ? ` (${region})` : ""
          } has high *CPU usage* 📈`,
        },
      },
      {
        type: "section",
        text: {
          type: "mrkdwn",
          text: `cpu: *${usage}%*`,
        },
      },
      toNotify && toNotify.length > 0 ? {
        type: "section",
        text: {
          type: "mrkdwn",
          text: toNotify.reduce(
            (prev, curr) => (prev ? " <@" + curr + ">" : "<@" + curr + ">"),
            ""
          ),
        },
      } : undefined,
    ]
  );
}

export async function notifySlackMem(
  name: string,
  region: string | undefined,
  usedMem: number,
  totalMem: number,
  memPercentage: number,
  toNotify?: string[]
) {
  await notifySlackAdvanced(
    `WARNING | ${name}${region ? ` (${region})` : ""} has high memory usage 💾 🚨`,
    [
      HEADER,
      {
        type: "section",
        text: {
          type: "mrkdwn",
          text: `*${name}*${
            region ? ` (${region})` : ""
          } has high *memory usage* 💾`,
        },
      },
      {
        type: "section",
        text: {
          type: "mrkdwn",
          text: `memory: ${usedMem} MB of ${totalMem} MB (*${memPercentage}%*)`,
        },
      },
      toNotify && toNotify.length > 0
        ? {
            type: "section",
            text: {
              type: "mrkdwn",
              text: toNotify.reduce(
                (prev, curr) => (prev ? " <@" + curr + ">" : "<@" + curr + ">"),
                ""
              ),
            },
          }
        : undefined,
    ]
  );
}

export async function notifySlackDisk(
  name: string,
  region: string | undefined,
  usedDisk: number,
  totalDisk: number,
  diskPercentage: number,
  toNotify?: string[]
) {
  await notifySlackAdvanced(
    `WARNING | ${name}${region ? ` (${region})` : ""} has high disk usage 💿 🚨`,
    [
      HEADER,
      {
        type: "section",
        text: {
          type: "mrkdwn",
          text: `*${name}*${
            region ? ` (${region})` : ""
          } has high *disk usage* 💿`,
        },
      },
      {
        type: "section",
        text: {
          type: "mrkdwn",
          text: `disk: using ${usedDisk} GB of ${totalDisk} GB (*${diskPercentage}%*)`,
        },
      },
      toNotify && toNotify.length > 0
        ? {
            type: "section",
            text: {
              type: "mrkdwn",
              text: toNotify.reduce(
                (prev, curr) => (prev ? " <@" + curr + ">" : "<@" + curr + ">"),
                ""
              ),
            },
          }
        : undefined,
    ]
  );
}

export async function notifySlackUnreachable(
  name: string,
  region: string | undefined,
  toNotify?: string[]
) {
  await notifySlackAdvanced(
    `WARNING 🚨 | ${name}${region ? ` (${region})` : ""} is unreachable ❌ 🚨`,
    [
      HEADER,
      {
        type: "section",
        text: {
          type: "mrkdwn",
          text: `*${name}*${region ? ` (${region})` : ""} is unreachable ❌ 🚨`,
        },
      },
      toNotify && toNotify.length > 0
        ? {
            type: "section",
            text: {
              type: "mrkdwn",
              text: toNotify.reduce(
                (prev, curr) => (prev ? " <@" + curr + ">" : "<@" + curr + ">"),
                ""
              ),
            },
          }
        : undefined,
    ]
  );
}
