import { readableTimestamp, timestamp } from "@monitor/util";
import { WebClient, LogLevel } from "@slack/web-api";
import { SLACK_CHANNEL, SLACK_TOKEN } from "../config";

const slack = new WebClient(SLACK_TOKEN, { logLevel: LogLevel.INFO });

export async function notifySlack(text: string) {
	try {
		await slack.chat.postMessage({
			token: SLACK_TOKEN,
      channel: SLACK_CHANNEL,
      text,
    });
	} catch (error) {
		console.log("POST TO SLACK FAILED @", readableTimestamp(timestamp()));
		console.log(error);
	}
}