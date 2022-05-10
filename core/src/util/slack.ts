import { readableTimestamp, timestamp } from "@monitor/util";
import { WebClient, LogLevel } from "@slack/web-api";
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