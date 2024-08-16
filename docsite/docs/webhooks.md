# Configuring Webhooks

Multiple Monitor resources can take advantage of webhooks from your git provider. Monitor support incoming webhooks using the Github standard, which is also supported by other providers like Gitea.

:::note
On Gitea, the default "Gitea" webhook type works with the Github standard 👍
:::

## Copy the Resource Payload URL

Find the resource in UI, like a `Build`, `Repo`, or `Stack`.
Scroll down to the bottom of Configuration area, and copy the webhook for the action you want.

## Create the webhook on the Git Provider

Navigate to the repo page on your git provider, and go to the settings for the Repo.
Find Webhook settings, and click to create a new webhook.

You will have to input some information. 

1. The `Payload URL` is the link that you copied in the step above, `Copy the Resource Payload URL`.
2. For Content-type, choose `application/json`
3. For Secret, input the secret you configured in the Monitor Core config (`MONITOR_WEBHOOK_SECRET`).
4. Enable SSL Verification, if you have proper TLS setup to your git provider (recommended).
5. For "events that trigger the webhook", just the push request is what post people want.
6. Of course, make sure the webhook is "Active" and hit create.

## When does it trigger?

Your git provider will now push this webhook to Monitor on *every* push to *any* branch. However, your `Build`, `Repo`,
etc. only cares about a specific branch of the repo.

Because of this, the webhook will trigger the action **only on pushes to the branch configured on the resource**.

For example, if I make a build, I may point the build to the `release` branch of a particular repo. If I set up a webhook, and push to the `main` branch, the action will *not trigger*. It will only trigger when the push is to the `release` branch.