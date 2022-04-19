import { getCoreDeployment } from "../mongoose/deployment";
import { deleteContainer, dockerRun } from "./docker";
import { execute } from "./execute";
import { prettyStringify } from "./general";

export type RestartError = {
	message: string;
	error: string;
}

export async function restart(
  args: { mongoUrl: string, pullLatest: boolean },
  onError: (err: RestartError) => void
) {
  try {
    if (args.pullLatest) {
      try {
        await execute("docker pull mbecker2020/monitor-core");
      } catch (error) {
        onError({
          message: "failed to pull latest image",
          error: prettyStringify(error),
        });
      }
    }
    const deployment = await getCoreDeployment(args);
    if (deployment) {
      try {
        await deleteContainer(deployment.containerName!);
        return await dockerRun(deployment);
      } catch (error) {
        onError({
          message: "failed to restart container",
          error: prettyStringify(error),
        });
      }
    } else {
      onError({
        message: "could not find deployment at name",
        error: "",
      });
    }
  } catch (error) {
    onError({
      message: "failed to connect to mongo at url",
      error: prettyStringify(error),
    });
  }
}
