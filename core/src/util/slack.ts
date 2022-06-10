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

export async function notifySlackAdvanced(blocks: (Block | KnownBlock)[]) {
	try {
		await slack.chat.postMessage({
			token: SECRETS.SLACK_TOKEN,
			channel: SLACK_CHANNEL,
			attachments: [{ blocks }],
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
		text: "WARNING"
	}
}

export async function notifySlackCpu(name: string, region: string | undefined, usage: number, toNotify: string[]) {
	await notifySlackAdvanced([
		HEADER,
		{
			type: "section",
			fields: [
				{
					type: "mrkdwn",
					text: `*${name}*${region ? ` (${region})` : ""} has high *CPU usage*`
				}
			]
		},
		{
			type: "section",
			fields: [
				{
					type: "mrkdwn",
					text: `cpu: *${usage}%*`
				}
			]
		},
		{
			type: "section",
			fields: [
				{
					type: "mrkdwn",
					text: toNotify.reduce(
						(prev, curr) => (prev ? " <@" + curr + ">" : "<@" + curr + ">"),
						""
					)
				}
			]
		}
	]);
}

export async function notifySlackMem(name: string, region: string | undefined, usedMem: number, totalMem: number, memPercentage: number, toNotify: string[]) {
	await notifySlackAdvanced([
		HEADER,
		{
			type: "section",
			fields: [
				{
					type: "mrkdwn",
					text: `*${name}*${region ? ` (${region})` : ""} has high *memory usage*`
				}
			]
		},
		{
			type: "section",
			fields: [
				{
					type: "mrkdwn",
					text: `memory: ${usedMem} MB of ${totalMem} MB (*${memPercentage}%*)`
				}
			]
		},
		{
			type: "section",
			fields: [
				{
					type: "mrkdwn",
					text: toNotify.reduce(
						(prev, curr) => (prev ? " <@" + curr + ">" : "<@" + curr + ">"),
						""
					)
				}
			]
		}
	]);
}

export async function notifySlackDisk(name: string, region: string | undefined, usedDisk: number, totalDisk: number, diskPercentage: number, toNotify: string[]) {
	await notifySlackAdvanced([
		HEADER,
		{
			type: "section",
			fields: [
				{
					type: "mrkdwn",
					text: `*${name}*${region ? ` (${region})` : ""} has high *disk usage*`
				}
			]
		},
		{
			type: "section",
			fields: [
				{
					type: "mrkdwn",
					text: `disk: using ${usedDisk} GB of ${totalDisk} GB (*${diskPercentage}%*)`
				}
			]
		},
		{
			type: "section",
			fields: [
				{
					type: "mrkdwn",
					text: toNotify.reduce(
						(prev, curr) => (prev ? " <@" + curr + ">" : "<@" + curr + ">"),
						""
					)
				}
			]
		}
	]);
}